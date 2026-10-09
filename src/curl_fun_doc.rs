//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::doc::*;
use crate::mod_node::*;

pub fn add_curl_fun_doc(sig_root_mod: &mut ModNode<Sig, ()>, doc_root_mod: &mut ModNode<String, Option<String>>)
{
    let doc = r#"
# CURL function

The [cURL](#var.curl) function allows to connect with network by many protocols. Data can be
transfered by the [cURL](#var.curl) function from/to servers by using for example HTTP or HTTPS.
"#;
    match doc_root_mod.value() {
        Some(prev_doc) => doc_root_mod.set_value(Some(prev_doc.clone() + "\n" + &doc[1..])),
        None => doc_root_mod.set_value(Some(String::from(&doc[1..]))),
    }

    let doc = r#"
Transfers data from/to the `url` URL.

This function returns an array or a string. The result of this function is the header or the
content if this function returns the string, otherwise the array with the header and the content.
Also, this function can return `true` if the header isn't received and the content isn't
downloaded. If the `opts` structure isn't passed, this function returns the content as a string.
The `opts` structure can have the following fields:

- `read` - the uploading data (optional)
- `write` -  if this field has the convertible value to `true`, this function returns a downloaded
  content (default: false) (optional)
- `header` - if this field has the convertible value to `true`, this function returns a received
  header (default: false) (optional)
- `progress` - if this field has the convertible value to `true`, this function prints a progress
  of uploading and downloading (default: false) (optional)
- `username` - the username (cURL option: `CURLOPT_USERNAME`) (optional)
- `password` - the password (cURL option: `CURLOPT_PASSWORD`) (optional)
- `httpheaders` - the HTTP headers are an array with strings (cURL option: `CURLOPT_HTTPHEADER`)
- `get` - if this field has the convertible value to `true`, this function uses the HTTP GET
  request (cURL option: `CURLOPT_HTTPGET`) (optional)
- `post` - if this field has the convertible value to `true`, this function uses the HTTP POST
  request (cURL option: `CURLOPT_POST`) (optional)
- `put` - if this field has the convertible value to `true`, this function uses the HTTP PUT
  request (cURL option: `CURLOPT_PUT`)(optional)
- `postfieldsize` - the size of uploading data for the HTTP POST request (cURL option:
  `CURLOPT_POSTFIELDSIZE_LARGE`) (optional)
- `failonerror` - if this field has the convertible value to `true`, this function returns an
  error for the HTTP response code >= 400 (cURL option: `CURLOPT_FAILONERROR`) (optional)
- `followlocation` - if this field has the convertible value to `true`, this function follows the
  HTTP 3xx redirect (cURL option: `CURLOPT_FOLLOWLOCATION`) (optional)
- `customrequest` - the custom method of HTTP request (cURL option: `CURLOPT_CUSTOMREQUEST`)
  (optional)
- `sslkeytype` - the type of private key (cURL option: `CURLOPT_SSLKEYTYPE`) (optional)
- `sslkey` - the file of private key (cURL option: `CURLOPT_SSLKEY`) (optional)
- `keypassword` - the passphrase to private key (cURL option: `CURLOPT_KEYPASSWD`) (optional)
- `pinnedpublickey` - the pinned public key (cURL option: `CURLOPT_PINNEDPUBLICKEY`) (optional)
- `connecttimeout` - the timeout in milliseconds for the connect phase (cURL:
  `CURLOPT_CONNECTTIMEOUT_MS`) (optional)
- `timeout` - the timeout in milliseconds for the request (cURL: `CURLOPT_TIMEOUT_MS`) (optional)
- `uploadfile` - the uploading file (optional)
- `downloadfile` - the downloading file (optional)

If an error occurs while this operation, this functio returns an error with the `"curl"` error
kind, the `"io"` error kind, or the `"utf8"` error kind.
"#;
    sig_root_mod.add_var(String::from("curl"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("url")),
        BuiltinFunArg::OptArg(String::from("opts"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("curl"), String::from(&doc[1..]));
}
