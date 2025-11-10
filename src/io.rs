//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::env::*;
use crate::error::*;
use crate::tree::*;
use crate::utils::*;
use crate::value::*;

fn read_magic(r: &mut dyn Read) -> Result<()>
{
    let mut buf = [0u8; 6];
    match r.read_exact(&mut buf) {
        Ok(()) => {
            if &buf != b"unlab1" {
                return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid data format")));
            }
            Ok(())
        },
        Err(err) => Err(Error::Io(err)),
    }
}

fn read_u8(r: &mut dyn Read) -> Result<u8>
{
    let mut buf = [0u8; 1];
    match r.read_exact(&mut buf) {
        Ok(()) => Ok(buf[0]),
        Err(err) => Err(Error::Io(err)),
    }
}

fn read_bool(r: &mut dyn Read) -> Result<bool>
{ 
    match read_u8(r)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid boolean"))),
    }
}

fn read_u64(r: &mut dyn Read) -> Result<u64>
{
    let mut buf = [0u8; 8];
    match r.read_exact(&mut buf) {
        Ok(()) => Ok(u64::from_le_bytes(buf)),
        Err(err) => Err(Error::Io(err)),
    }
}

fn read_usize(r: &mut dyn Read) -> Result<usize>
{
    let n = read_u64(r)?;
    if n > (usize::MAX as u64) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large unsigned number")));
    }
    Ok(n as usize)
}

fn read_i64(r: &mut dyn Read) -> Result<i64>
{
    let mut buf = [0u8; 8];
    match r.read_exact(&mut buf) {
        Ok(()) => Ok(i64::from_le_bytes(buf)),
        Err(err) => Err(Error::Io(err)),
    }
}

fn read_f32(r: &mut dyn Read) -> Result<f32>
{
    let mut buf = [0u8; 4];
    match r.read_exact(&mut buf) {
        Ok(()) => Ok(f32::from_le_bytes(buf)),
        Err(err) => Err(Error::Io(err)),
    }
}

fn read_string(r: &mut dyn Read) -> Result<String>
{
    let len = read_usize(r)?;
    let mut buf = vec![0u8; len];
    match r.read_exact(&mut buf) {
        Ok(()) => {
            match String::from_utf8(buf) {
                Ok(s) => Ok(s),
                Err(err) => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, format!("invalid string: {}", err)))),
            }
        },
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_magic(w: &mut dyn Write) -> Result<()>
{
    match w.write_all(b"unlab1") {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_u8(w: &mut dyn Write, n: u8) -> Result<()>
{
    let buf = [n];
    match w.write_all(&buf) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_bool(w: &mut dyn Write, b: bool) -> Result<()>
{ write_u8(w, if b { 1 } else { 0 }) }

fn write_u64(w: &mut dyn Write, n: u64) -> Result<()>
{
    let buf = n.to_le_bytes();
    match w.write_all(&buf) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_usize(w: &mut dyn Write, n: usize) -> Result<()>
{ write_u64(w, n as u64) }

fn write_i64(w: &mut dyn Write, n: i64) -> Result<()>
{
    let buf = n.to_le_bytes();
    match w.write_all(&buf) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_f32(w: &mut dyn Write, n: f32) -> Result<()>
{
    let buf = n.to_le_bytes();
    match w.write_all(&buf) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

fn write_str(w: &mut dyn Write, s: &str) -> Result<()>
{
    write_usize(w, s.as_bytes().len())?;
    match w.write_all(s.as_bytes()) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

const VALUE_NONE: u8 = 0;
const VALUE_BOOL: u8 = 1;
const VALUE_INT: u8 = 2;
const VALUE_FLOAT: u8 = 3;
const VALUE_OBJECT: u8 = 4;
const VALUE_REF: u8 = 5;
const VALUE_WEAK: u8 = 6;
const VALUE_WEAK_NONE: u8 = 7;
const VALUE_OBJECT_INDEX: u8 = 8;
const VALUE_REF_INDEX: u8 = 9;
const VALUE_WEAK_INDEX: u8 = 10;

const OBJECT_STRING: u8 = 0;
const OBJECT_INT_RANGE: u8 = 1;
const OBJECT_FLOAT_RANGE: u8 = 2;
const OBJECT_MATRIX: u8 = 3;
const OBJECT_FUN: u8 = 4;
const OBJECT_BUILTIN_FUN: u8 = 5;
const OBJECT_MATRIX_ARRAY: u8 = 6;
const OBJECT_MATRIX_ROW_SLICE: u8 = 7;
const OBJECT_ERROR: u8 = 8;

const MUT_OBJECT_ARRAY: u8 = 0;
const MUT_OBJECT_STRUCT: u8 = 1;

const MATRIX_ARRAY_OBJECT: u8 = 0;
const MATRIX_ARRAY_INDEX: u8 = 1;

struct ObjectTab<T>
{
    indices: HashMap<*const T, usize>,
    objects: HashMap<usize, Arc<T>>,
    count: usize,
}

impl<T> ObjectTab<T>
{
    fn new() -> Self
    { ObjectTab { indices: HashMap::new(), objects: HashMap::new(), count: 0, } }
    
    fn index(&self, object: &Arc<T>) -> Option<usize>
    { self.indices.get(&Arc::as_ptr(object)).map(|i| *i) }
    
    fn object(&self, idx: usize) -> Option<&Arc<T>>
    { self.objects.get(&idx) }
    
    fn add_object(&mut self, object: Arc<T>) -> bool
    {
        self.indices.insert(Arc::as_ptr(&object), self.count);
        self.objects.insert(self.count, object);
        match self.count.checked_add(1) {
            Some(new_count) => {
                self.count = new_count;
                true
            },
            None => false, 
        }
    }
}

fn checked_mul_row_count_and_col_count(row_count: usize, col_count: usize) -> Result<usize>
{
    if row_count > (isize::MAX as usize) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large number of rows")));
    }
    if col_count > (isize::MAX as usize) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large number of columns")));
    }
    match row_count.checked_mul(col_count) {
        Some(len) => {
            if len > (isize::MAX as usize) {
                return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large number of matrix elements")));
            }
            match (len as isize).checked_mul(size_of::<f32>() as isize) {
                Some(_) => Ok(len as usize),
                None => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large number of matrix elements"))),
            }
        },
        None => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large number of matrix elements"))),
    }
}

fn read_object(r: &mut dyn Read, env: &Env, object_tab: &mut ObjectTab<Object>) -> Result<Arc<Object>>
{
    let object = match read_u8(r)? {
        OBJECT_STRING => Arc::new(Object::String(read_string(r)?)),
        OBJECT_INT_RANGE => {
            let from = read_i64(r)?;
            let to = read_i64(r)?;
            let step = read_i64(r)?;
            Arc::new(Object::IntRange(from, to, step))
        },
        OBJECT_FLOAT_RANGE => {
            let from = read_f32(r)?;
            let to = read_f32(r)?;
            let step = read_f32(r)?;
            Arc::new(Object::FloatRange(from, to, step))
        },
        OBJECT_MATRIX => {
            let row_count = read_usize(r)?;
            let col_count = read_usize(r)?;
            let is_transposed = read_bool(r)?;
            let len = checked_mul_row_count_and_col_count(row_count, col_count)?;
            let mut xs = vec![0.0f32; len];
            for i in 0..len {
                xs[i] = read_f32(r)?;
            }
            if !is_transposed {
                Arc::new(Object::Matrix(matrix_create_and_set_elems(row_count, col_count, xs.as_slice())?))
            } else {
                Arc::new(Object::Matrix(matrix_create_and_set_elems(col_count, row_count, xs.as_slice())?.transpose()))
            }
        },
        OBJECT_FUN => {
            let ident_count = read_usize(r)?;
            let mut idents: Vec<String> = Vec::new();
            for _ in 0..ident_count {
                idents.push(read_string(r)?);
            }
            let ident = read_string(r)?;
            match env.var(&Name::Abs(idents, ident))? {
                Some(Value::Object(object)) => object.clone(),
                Some(_) => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid function type"))),
                None => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "undefined function"))),
            }
        },
        OBJECT_BUILTIN_FUN => {
            let ident = read_string(r)?;
            match env.var(&Name::Abs(Vec::new(), ident))? {
                Some(Value::Object(object)) => object.clone(),
                Some(_) => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid built-in function type"))),
                None => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "undefined built-in function"))),
            }
        },
        OBJECT_MATRIX_ARRAY => {
            let row_count = read_usize(r)?;
            let col_count = read_usize(r)?;
            let transpose_flag = if read_bool(r)? {
                TransposeFlag::Transpose
            } else {
                TransposeFlag::NoTranspose
            };
            let len = checked_mul_row_count_and_col_count(row_count, col_count)?;
            let mut xs = vec![0.0f32; len];
            for i in 0..len {
                xs[i] = read_f32(r)?;
            }
            Arc::new(Object::MatrixArray(row_count, col_count, transpose_flag, xs))
        },
        OBJECT_MATRIX_ROW_SLICE => {
            let matrix_array = match read_u8(r)? {
                MATRIX_ARRAY_OBJECT => read_object(r, env, object_tab)?,
                MATRIX_ARRAY_INDEX => {
                    match object_tab.object(read_usize(r)?) {
                        Some(tmp_matrix_array) => tmp_matrix_array.clone(),
                        None => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid object index"))),
                    }
                },
                _ => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid matrix array type"))),
            };
            let i = read_usize(r)?;
            Arc::new(Object::MatrixRowSlice(matrix_array, i))
        },
        OBJECT_ERROR => {
            let kind = read_string(r)?;
            let msg = read_string(r)?;
            Arc::new(Object::Error(kind, msg))
        },
        _ => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid object type"))),
    };
    if !object_tab.add_object(object.clone()) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large index")));
    }
    Ok(object)
}

fn read_mut_object(r: &mut dyn Read, env: &Env, object_tab: &mut ObjectTab<Object>, mut_object_tab: &mut ObjectTab<RwLock<MutObject>>) -> Result<Arc<RwLock<MutObject>>>
{
    let object = match read_u8(r)? {
        MUT_OBJECT_ARRAY => Arc::new(RwLock::new(MutObject::Array(Vec::new()))),
        MUT_OBJECT_STRUCT => Arc::new(RwLock::new(MutObject::Struct(BTreeMap::new()))),
        _ => return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid mutable object type"))),
    };
    if !mut_object_tab.add_object(object.clone()) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large index")));
    }
    let len = read_usize(r)?;
    {
        let mut object_g = rw_lock_write(&object)?;
        match &mut *object_g {
            MutObject::Array(elems) => {
                for _ in 0..len {
                    elems.push(read_value(r, env, object_tab, mut_object_tab)?);
                }
            },
            MutObject::Struct(fields) => {
                for _ in 0..len {
                    let ident = read_string(r)?;
                    let field = read_value(r, env, object_tab, mut_object_tab)?;
                    fields.insert(ident, field);
                }
            },
        }
    }
    Ok(object)
}

fn read_value(r: &mut dyn Read, env: &Env, object_tab: &mut ObjectTab<Object>, mut_object_tab: &mut ObjectTab<RwLock<MutObject>>) -> Result<Value>
{
    match read_u8(r)? {
        VALUE_NONE => Ok(Value::None),
        VALUE_BOOL => Ok(Value::Bool(read_bool(r)?)),
        VALUE_INT => Ok(Value::Int(read_i64(r)?)),
        VALUE_FLOAT => Ok(Value::Float(read_f32(r)?)),
        VALUE_OBJECT => Ok(Value::Object(read_object(r, env, object_tab)?)),
        VALUE_REF => Ok(Value::Ref(read_mut_object(r, env, object_tab, mut_object_tab)?)),
        VALUE_WEAK => Ok(Value::Weak(Arc::downgrade(&read_mut_object(r, env, object_tab, mut_object_tab)?))),
        VALUE_WEAK_NONE => Ok(Value::Weak(Weak::new())),
        VALUE_OBJECT_INDEX => {
            match object_tab.object(read_usize(r)?) {
                Some(object) => Ok(Value::Object(object.clone())),
                None => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid object index"))),
            }
        },
        VALUE_REF_INDEX => {
            match mut_object_tab.object(read_usize(r)?) {
                Some(object) => Ok(Value::Ref(object.clone())),
                None => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid mutable object index"))),
            }
        },
        VALUE_WEAK_INDEX => {
            match mut_object_tab.object(read_usize(r)?) {
                Some(object) => Ok(Value::Weak(Arc::downgrade(object))),
                None => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid mutable object index"))),
            }
        },
        _ => Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "invalid value type"))),
    }
}

pub fn read_values(r: &mut dyn Read, env: &Env) -> Result<Vec<Value>>
{ 
    let mut object_tab: ObjectTab<Object> = ObjectTab::new();
    let mut mut_object_tab: ObjectTab<RwLock<MutObject>> = ObjectTab::new();
    read_magic(r)?;
    let count = read_usize(r)?;
    let mut values: Vec<Value> = Vec::new();
    for _ in 0..count {
        values.push(read_value(r, env, &mut object_tab, &mut mut_object_tab)?);
    }
    Ok(values)
}

fn write_object(w: &mut dyn Write, object: &Arc<Object>, object_tab: &mut ObjectTab<Object>) -> Result<()>
{
    match &**object {
        Object::String(s) => {
            write_u8(w, OBJECT_STRING)?;
            write_str(w, s.as_str())?;
        },
        Object::IntRange(from, to, step) => {
            write_u8(w, OBJECT_INT_RANGE)?;
            write_i64(w, *from)?;
            write_i64(w, *to)?;
            write_i64(w, *step)?;
        },
        Object::FloatRange(from, to, step) => {
            write_u8(w, OBJECT_FLOAT_RANGE)?;
            write_f32(w, *from)?;
            write_f32(w, *to)?;
            write_f32(w, *step)?;
        },
        Object::Matrix(a) => {
            let xs = matrix_elems_and_transpose_flag(a)?.0;
            write_u8(w, OBJECT_MATRIX)?;
            write_usize(w, a.row_count())?;
            write_usize(w, a.col_count())?;
            write_bool(w, a.is_transposed())?;
            for x in &xs {
                write_f32(w, *x)?;
            }
        },
        Object::Fun(idents, ident, _) => {
            write_u8(w, OBJECT_FUN)?;
            write_usize(w, idents.len())?;
            for ident2 in idents {
                write_str(w, ident2.as_str())?;
            }
            write_str(w, ident.as_str())?;
        },
        Object::BuiltinFun(ident, _) => {
            write_u8(w, OBJECT_FUN)?;
            write_str(w, ident.as_str())?;
        },
        Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
            write_u8(w, OBJECT_MATRIX_ARRAY)?;
            write_usize(w, *row_count)?;
            write_usize(w, *col_count)?;
            write_bool(w, *transpose_flag == TransposeFlag::Transpose)?;
            for x in xs {
                write_f32(w, *x)?;
            }
        },
        Object::MatrixRowSlice(matrix_array, i) => {
            write_u8(w, OBJECT_MATRIX_ROW_SLICE)?;
            match object_tab.index(object) {
                Some(idx) => {
                    write_u8(w, MATRIX_ARRAY_INDEX)?;
                    write_usize(w, idx)?;
                },
                None => {
                    write_u8(w, MATRIX_ARRAY_OBJECT)?;
                    write_object(w, matrix_array, object_tab)?;
                },
            }
            write_usize(w, *i)?;
        },
        Object::Error(kind, msg) => {
            write_u8(w, OBJECT_ERROR)?;
            write_str(w, kind.as_str())?;
            write_str(w, msg.as_str())?;
        },
    }
    if !object_tab.add_object(object.clone()) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large index")));
    }
    Ok(())
}

fn write_mut_object(w: &mut dyn Write, object: &Arc<RwLock<MutObject>>, object_tab: &mut ObjectTab<Object>, mut_object_tab: &mut ObjectTab<RwLock<MutObject>>) -> Result<()>
{
    if !mut_object_tab.add_object(object.clone()) {
        return Err(Error::Io(io::Error::new(ErrorKind::InvalidData, "too large index")));
    }
    let object_g = rw_lock_read(object)?;
    match &*object_g {
        MutObject::Array(elems) => {
            write_u8(w, MUT_OBJECT_ARRAY)?;
            write_usize(w, elems.len())?;
            for elem in elems {
                write_value(w, elem, object_tab, mut_object_tab)?;
            }
        },
        MutObject::Struct(fields) => {
            write_u8(w, MUT_OBJECT_STRUCT)?;
            write_usize(w, fields.len())?;
            for (ident, field) in fields {
                write_str(w, ident.as_str())?;
                write_value(w, field, object_tab, mut_object_tab)?;
            }
        },
    }
    Ok(())
}

fn write_value(w: &mut dyn Write, value: &Value, object_tab: &mut ObjectTab<Object>, mut_object_tab: &mut ObjectTab<RwLock<MutObject>>) -> Result<()>
{
    match value {
        Value::None => write_u8(w, VALUE_NONE)?,
        Value::Bool(b) => {
            write_u8(w, VALUE_BOOL)?;
            write_bool(w, *b)?;
        },
        Value::Int(n) => {
            write_u8(w, VALUE_INT)?;
            write_i64(w, *n)?;
        },
        Value::Float(n) => {
            write_u8(w, VALUE_FLOAT)?;
            write_f32(w, *n)?;
        },
        Value::Object(object) => {
            match object_tab.index(object) {
                Some(idx) => {
                    write_u8(w, VALUE_OBJECT_INDEX)?;
                    write_usize(w, idx)?;
                },
                None => {
                    write_u8(w, VALUE_OBJECT)?;
                    write_object(w, object, object_tab)?;
                },
            }
        },
        Value::Ref(object) => {
            match mut_object_tab.index(object) {
                Some(idx) => {
                    write_u8(w, VALUE_REF_INDEX)?;
                    write_usize(w, idx)?;
                },
                None => {
                    write_u8(w, VALUE_REF)?;
                    write_mut_object(w, object, object_tab, mut_object_tab)?;
                },
            }
        },
        Value::Weak(object) => {
            match object.upgrade() {
                Some(object) => {
                    match mut_object_tab.index(&object) {
                        Some(idx) => {
                            write_u8(w, VALUE_WEAK_INDEX)?;
                            write_usize(w, idx)?;
                        },
                        None => {
                            write_u8(w, VALUE_WEAK)?;
                            write_mut_object(w, &object, object_tab, mut_object_tab)?;
                        },
                    }
                },
                None => write_u8(w, VALUE_WEAK_NONE)?,
            }
        },
    }
    Ok(())
}

pub fn write_values(w: &mut dyn Write, values: &[Value]) -> Result<()>
{ 
    let mut object_tab: ObjectTab<Object> = ObjectTab::new();
    let mut mut_object_tab: ObjectTab<RwLock<MutObject>> = ObjectTab::new();
    write_magic(w)?;
    write_usize(w, values.len())?;
    for value in values {
        write_value(w, value, &mut object_tab, &mut mut_object_tab)?;
    }
    Ok(())
}

pub fn load_values(path: &str, env: &Env) -> Result<Vec<Value>>
{
    match File::open(path) {
        Ok(file) => {
            let mut r = BufReader::new(file);
            read_values(&mut r, env)
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn save_values(path: &str, values: &[Value]) -> Result<()>
{
    match File::create(path) {
        Ok(file) => {
            let mut w = BufWriter::new(file);
            write_values(&mut w, values)
        },
        Err(err) => Err(Error::Io(err)),
    }
}
