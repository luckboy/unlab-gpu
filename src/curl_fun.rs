//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs;
use std::fs::File;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::io::stdout;
use std::result;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use crate::curl;
use crate::curl::easy::List;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::value::*;
use crate::utils::*;

#[derive(Clone, Debug)]
struct CurlOptions
{
    read: Option<Arc<String>>,
    write: Option<bool>,
    header: Option<bool>,
    progress: Option<bool>,
    http_headers: Option<Vec<String>>,
    get: Option<bool>,
    post: Option<bool>,
    put: Option<bool>,
    fail_on_error: Option<bool>,
    follow_location: Option<bool>,
    custom_request: Option<String>,
    upload_file: Option<String>,
    download_file: Option<String>,
}

fn create_curl_options(value: &Value) -> Result<CurlOptions>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Struct(fields) => {
                    let read = match fields.get(&String::from("read")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(Arc::new(format!("{}", field))),
                            }
                        },
                        None => None,
                    };
                    let write = match fields.get(&String::from("write")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let header = match fields.get(&String::from("header")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let progress = match fields.get(&String::from("progress")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let http_headers = match fields.get(&String::from("httpheaders")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                Value::Ref(http_header_list_object) => {
                                    let http_header_list_object = rw_lock_read(&*http_header_list_object)?;
                                    match &*http_header_list_object {
                                        MutObject::Array(elems) => Some(elems.iter().map(|v| format!("{}", v)).collect()),
                                        _ => return Err(Error::Interp(String::from("invalid type for HTTP headers"))),
                                    }
                                },
                                _ => return Err(Error::Interp(String::from("unsupported type for HTTP headers"))),
                            }
                        },
                        None => None,
                    };
                    let get = match fields.get(&String::from("get")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let post = match fields.get(&String::from("post")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let put = match fields.get(&String::from("put")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let fail_on_error = match fields.get(&String::from("failonerror")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let follow_location = match fields.get(&String::from("followlocation")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(field.to_bool()),
                            }
                        },
                        None => None,
                    };
                    let custom_request = match fields.get(&String::from("customrequest")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(format!("{}", field)),
                            }
                        },
                        None => None,
                    };
                    let upload_file = match fields.get(&String::from("uploadfile")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(format!("{}", field)),
                            }
                        },
                        None => None,
                    };
                    let download_file = match fields.get(&String::from("downloadfile")) {
                        Some(field) => {
                            match field {
                                Value::None => None,
                                _ => Some(format!("{}", field)),
                            }
                        },
                        None => None,
                    };
                    Ok(CurlOptions {
                            read,
                            write,
                            header,
                            progress,
                            http_headers,
                            get,
                            post,
                            put,
                            fail_on_error,
                            follow_location,
                            custom_request,
                            upload_file,
                            download_file,
                    })
                },
                _ => Err(Error::Interp(String::from("unsupported type for curl function"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for curl function"))),
    }
}

fn print_progress(uploading_byte_count: f64, total_uploading_byte_count: f64, downloading_byte_count: f64, total_downloading_byte_count: f64, is_done: bool) -> Result<()>
{
    let uploading_kib = (uploading_byte_count / 1024.0).ceil();
    let uploading_perc = if total_uploading_byte_count != 0.0 {
        Some(((uploading_byte_count * 100.0) / total_uploading_byte_count).floor())
    } else {
        None
    };
    let downloading_kib = (downloading_byte_count / 1024.0).ceil();
    let downloading_perc = if total_downloading_byte_count != 0.0 {
        Some(((downloading_byte_count * 100.0) / total_downloading_byte_count).floor())
    } else {
        None
    };
    if is_done {
        println!("cURL progress: {}KiB ({}%), {}KiB ({}%)", uploading_kib, uploading_perc.map(|n| format!("{}", n)).unwrap_or(String::from("?")), downloading_kib, downloading_perc.map(|n| format!("{}", n)).unwrap_or(String::from("?")));
    } else {
        print!("cURL progress: {}KiB ({}%), {}KiB ({}%)\r", uploading_kib, uploading_perc.map(|n| format!("{}", n)).unwrap_or(String::from("?")), downloading_kib, downloading_perc.map(|n| format!("{}", n)).unwrap_or(String::from("?")));
        match stdout().flush() {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
    }
    Ok(())
}

fn curl_res_curl_fun(url: &str, opts: &Option<CurlOptions>) -> result::Result<(Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<(f64, f64)>>), curl::Error>
{
    let mut easy = curl::easy::Easy::new();
    easy.url(url)?;
    let byte_counts = Arc::new(Mutex::new((0.0, 0.0)));
    let mut is_writing = false;
    let header: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    match &opts {
        Some(opts) => {
            match opts.get {
                Some(get) => easy.get(get)?,
                None => (),
            }
            match opts.post {
                Some(post) => easy.post(post)?,
                None => (),
            }
            match opts.put {
                Some(put) => easy.put(put)?,
                None => (),
            }
            match opts.fail_on_error {
                Some(fail_on_error) => easy.fail_on_error(fail_on_error)?,
                None => (),
            }
            match opts.follow_location {
                Some(follow_location) => easy.follow_location(follow_location)?,
                None => (),
            }
            match &opts.custom_request {
                Some(custom_request) => easy.custom_request(custom_request.as_str())?,
                None => (),
            }
            match &opts.http_headers {
                Some(http_headers) => {
                    let mut http_headers2 = List::new();
                    for http_header in http_headers {
                        http_headers2.append(http_header)?;
                    }
                    easy.http_headers(http_headers2)?;
                },
                None => (),
            }
            match opts.progress {
                Some(true) => {
                    let byte_counts2 = byte_counts.clone();
                    easy.progress(true)?;
                    easy.progress_function(move |total_downloading_byte_count, downloading_byte_count, total_uploading_byte_count, uploading_byte_count| {
                            match print_progress(uploading_byte_count, total_uploading_byte_count, downloading_byte_count, total_downloading_byte_count, false) {
                                Ok(()) => (),
                                Err(err) => eprint_error(&err),
                            }
                            let mut byte_counts2_g = byte_counts2.lock().unwrap();
                            *byte_counts2_g = (uploading_byte_count, downloading_byte_count);
                            true
                    })?;
                },
                Some(false) => easy.progress(false)?,
                None => (),
            }
            match &opts.read {
                Some(s) => {
                    let mut i = 0usize;
                    let s2 = s.clone();
                    easy.read_function(move |buf| {
                            let data = s2.as_bytes();
                            let data2 = &data[i..];
                            let mut len = data2.len();
                            if buf.len() < len {
                                len = buf.len();
                            }
                            let (dst, _) = buf.split_at_mut(len);
                            let (src, _) = data2.split_at(len);
                            dst.copy_from_slice(src);
                            i += len;
                            Ok(len)
                    })?;
                },
                None => (),
            }
            match &opts.upload_file {
                Some(path) => {
                    let path2 = path.clone();
                    let mut off = 0u64;
                    easy.read_function(move |buf| {
                            let file_len = match fs::metadata(path2.as_str()) {
                                Ok(metadata) => metadata.len(),
                                Err(err) => {
                                    eprint_error(&Error::Io(err));
                                    return Ok(0);
                                },
                            };
                            match File::open(path2.as_str()) {
                                Ok(mut file) => {
                                    match file.seek(SeekFrom::Start(off)) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            eprint_error(&Error::Io(err));
                                            return Ok(0);
                                        },
                                    }
                                    let data_len = file_len - off;
                                    let len = if (buf.len() as u64) < data_len {
                                        buf.len()
                                    } else {
                                        data_len as usize
                                    };
                                    let (dst, _) = buf.split_at_mut(len);
                                    match file.read_exact(dst) {
                                        Ok(()) => (),
                                        Err(err) => {
                                            eprint_error(&Error::Io(err));
                                            return Ok(0);
                                        },
                                    }
                                    off += len as u64;
                                    Ok(len)
                                },
                                Err(err) => {
                                    eprint_error(&Error::Io(err));
                                    Ok(0)
                                },
                            }
                    })?;
                },
                None => (),
            }
            match opts.header {
                Some(true) => {
                    let header2 = header.clone();
                    easy.header_function(move |buf| {
                            let mut header2_g = header2.lock().unwrap();
                            let mut data: Vec<u8> = Vec::new();
                            data.extend_from_slice(buf);
                            *header2_g = Some(data);
                            true
                    })?;
                },
                _ => (),
            }
            match opts.write {
                Some(true) => is_writing = true,
                _ => (),
            }
            match &opts.download_file {
                Some(path) => {
                    let path2 = path.clone();
                    easy.write_function(move |buf| {
                            match File::options().create(true).append(true).open(path2.as_str()) {
                                Ok(mut file) => {
                                    match file.write_all(buf) {
                                        Ok(()) => (),
                                        Err(err) => eprint_error(&Error::Io(err)),
                                    }
                                },
                                Err(err) => eprint_error(&Error::Io(err)),
                            }
                            Ok(buf.len())
                    })?;
                    is_writing = false;
                },
                None => (),
            }
        },
        None => is_writing = true,
    }
    let content: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    if is_writing {
        let content2 = content.clone();
        easy.write_function(move |buf| {
                let mut content2_g = content2.lock().unwrap();
                match &mut *content2_g {
                    Some(data) => data.extend_from_slice(buf),
                    None => {
                        let mut data: Vec<u8> = Vec::new();
                        data.extend_from_slice(buf);
                        *content2_g = Some(data); 
                    },
                }
                Ok(buf.len())
        })?;
    }
    easy.perform()?;
    Ok((header, content, byte_counts))
}

pub fn curl_fun(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() < 1 && arg_values.len() > 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let (url, opts) = match (arg_values.get(0), arg_values.get(1)) {
        (Some(url_value), Some(option_value)) => (format!("{}", url_value), Some(create_curl_options(option_value)?)),
        (Some(url_value), None) => (format!("{}", url_value), None),
        (_, _) => return Err(Error::Interp(String::from("no argument"))),
    };
    match &opts {
        Some(opts) => {
            match &opts.upload_file {
                Some(path) => {
                    match fs::metadata(path.as_str()) {
                        Ok(_) => (),
                        Err(err) => return Ok(Value::Object(Arc::new(Object::Error(String::from("io"), format!("{}", err))))),
                     }
                },
                None => (),
            }
            match &opts.download_file {
                Some(path) => {
                    match remove_file(path.as_str()) {
                        Ok(()) => (),
                        Err(err) if err.kind() == ErrorKind::NotFound => (),
                        Err(err) => return Ok(Value::Object(Arc::new(Object::Error(String::from("io"), format!("{}", err))))),
                     }
                },
                None => (),
            }
        },
        None => (),
    }
    if opts.as_ref().map(|os| os.progress.unwrap_or(false)).unwrap_or(false) {
        print_progress(0.0, 0.0, 0.0, 0.0, false)?;
    }
    match curl_res_curl_fun(url.as_str(), &opts) {
        Ok((header, content, byte_counts)) => {
            if opts.as_ref().map(|os| os.progress.unwrap_or(false)).unwrap_or(false) {
                let byte_counts_g = mutex_lock(&byte_counts)?;
                print_progress(byte_counts_g.0, byte_counts_g.0, byte_counts_g.1, byte_counts_g.1, true)?;
            }
            let mut elems: Vec<Value> = Vec::new();
            let mut header_g = mutex_lock(&*header)?;
            let mut content_g = mutex_lock(&*content)?;
            match header_g.take() {
                Some(header2) => {
                    match String::from_utf8(header2) {
                        Ok(s) => elems.push(Value::Object(Arc::new(Object::String(s)))),
                        Err(err) => return Ok(Value::Object(Arc::new(Object::Error(String::from("curlutf8"), format!("{}", err))))),
                    }
                },
                None => (),
            }
            match content_g.take() {
                Some(content2) => {
                    match String::from_utf8(content2) {
                        Ok(s) => elems.push(Value::Object(Arc::new(Object::String(s)))),
                        Err(err) => return Ok(Value::Object(Arc::new(Object::Error(String::from("curlutf8"), format!("{}", err))))),
                    }
                },
                None => (),
            }
            Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems)))))
        },
        Err(err) => Ok(Value::Object(Arc::new(Object::Error(String::from("curl"), format!("{}", err))))),
    }
}
