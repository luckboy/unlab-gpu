//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::VecDeque;
use std::sync::Barrier;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Instant;
use std::time::Duration;
use crate::error::*;
use crate::utils::*;
use crate::value::Value;

#[derive(Debug)]
pub enum SyncObject
{
    Barrier(Barrier),
    Mutex(Mutex<Value>),
    Monitor(Mutex<Value>, Condvar),
    RwLock(RwLock<Value>),
    Channel(Mutex<VecDeque<Value>>, Condvar),
}

impl SyncObject
{
    pub fn wait(&self) -> Result<bool>
    {
        match self {
            SyncObject::Barrier(barrier) => Ok(barrier.wait().is_leader()),
            _ => Err(Error::Interp(String::from("synchronization object isn't barrier"))),
        }
    }
    
    pub fn lock<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&mut Value) -> Result<()>
    {
        match self {
            SyncObject::Mutex(mutex) => {
                let mut guard = mutex_lock(mutex)?;
                f(&mut *guard)?;
                Ok(())
            },
            SyncObject::Monitor(mutex, _) => {
                let mut guard = mutex_lock(mutex)?;
                f(&mut *guard)?;
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't mutex and monitor"))),
        }
    }

    pub fn lock_and_wait<T, F, G, H>(&self, data: &mut T, f: F, mut g: G, h: H) -> Result<()>
        where F: FnOnce(&mut T, &mut Value) -> Result<bool>,
            G: FnMut(&mut T, &mut Value) -> Result<bool>,
            H: FnOnce(&mut T, &mut Value) -> Result<()>
    {
        match self {
            SyncObject::Monitor(mutex, condvar) => {
                let mut guard = mutex_lock(mutex)?;
                if f(data, &mut *guard)? {
                    loop {
                        guard = condvar_wait(condvar, guard)?;
                        if !g(data, &mut *guard)? {
                            break;
                        }
                    }
                }
                h(data, &mut *guard)?;
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't monitor"))),
        }
    }

    pub fn lock_and_wait_timeout<T, F, G, H>(&self, duration: Duration, data: &mut T, f: F, mut g: G, h: H) -> Result<()>
        where F: FnOnce(&mut T, &mut Value) -> Result<bool>,
            G: FnMut(&mut T, &mut Value, bool) -> Result<bool>,
            H: FnOnce(&mut T, &mut Value) -> Result<()>
    {
        match self {
            SyncObject::Monitor(mutex, condvar) => {
                let mut guard = mutex_lock(mutex)?;
                if f(data, &mut *guard)? {
                    loop {
                        let pair = condvar_wait_timeout(condvar, guard, duration)?;
                        let wait_timeout_res = pair.1;
                        guard = pair.0;
                        if !g(data, &mut *guard, wait_timeout_res.timed_out())? {
                            break;
                        }
                    }
                }
                h(data, &mut *guard)?;
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't monitor"))),
        }
    }

    pub fn lock_and_notify_one<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&mut Value) -> Result<()>
    {
        match self {
            SyncObject::Monitor(mutex, condvar) => {
                let mut guard = mutex_lock(mutex)?;
                f(&mut *guard)?;
                condvar.notify_one();
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't monitor"))),
        }
    }

    pub fn lock_and_notify_all<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&mut Value) -> Result<()>
    {
        match self {
            SyncObject::Monitor(mutex, condvar) => {
                let mut guard = mutex_lock(mutex)?;
                f(&mut *guard)?;
                condvar.notify_all();
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't monitor"))),
        }
    }    
    
    pub fn read<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&Value) -> Result<()>
    {
        match self {
            SyncObject::RwLock(rw_lock) => {
                let guard = rw_lock_read(rw_lock)?;
                f(&*guard)?;
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't rw lock"))),
        }
    }

    pub fn write<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&mut Value) -> Result<()>
    {
        match self {
            SyncObject::RwLock(rw_lock) => {
                let mut guard = rw_lock_write(rw_lock)?;
                f(&mut *guard)?;
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't rw lock"))),
        }
    }
    
    pub fn recv(&self) -> Result<Value>
    {
        match self {
            SyncObject::Channel(queue, condvar) => {
                let mut queue_g = mutex_lock(queue)?;
                let value = loop {
                    match queue_g.pop_front() {
                        Some(tmp_value) => break tmp_value.clone(),
                        None => (),
                    }
                    queue_g = condvar_wait(condvar, queue_g)?;
                };
                Ok(value)
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't channel"))),
        }
    }

    pub fn recv_timeout(&self, duration: Duration) -> Result<Option<Value>>
    {
        match self {
            SyncObject::Channel(queue, condvar) => {
                let mut instant = Instant::now();
                let mut waiting_duration = duration;
                let mut queue_g = mutex_lock(queue)?;
                let value = loop {
                    match queue_g.pop_front() {
                        Some(tmp_value) => break Some(tmp_value.clone()),
                        None => (),
                    }
                    waiting_duration = match waiting_duration.checked_sub(instant.elapsed()) {
                        Some(tmp_waiting_duration) => tmp_waiting_duration,
                        None => break None,
                    };
                    instant = Instant::now();
                    let pair = condvar_wait_timeout(condvar, queue_g, waiting_duration)?;
                    let wait_timeout_res = pair.1;
                    queue_g = pair.0;
                    if wait_timeout_res.timed_out() {
                        break None;
                    }
                };
                Ok(value)
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't channel"))),
        }
    }

    pub fn send(&self, value: Value) -> Result<()>
    {
        match self {
            SyncObject::Channel(queue, condvar) => {
                let mut queue_g = mutex_lock(queue)?;
                queue_g.push_back(value);                
                condvar.notify_one();
                Ok(())
            },
            _ => Err(Error::Interp(String::from("synchronization object isn't channel"))),
        }
    }
}

#[cfg(test)]
mod tests;
