//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of synchronization object. 
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

/// A synchronization object.
///
/// The synchrozization object can be used to synchronization of threads. These threads can use
/// this object to synchronization, notifying about, sending messages, and receiving messages.
#[derive(Debug)]
pub enum SyncObject
{
    /// A barrier.
    Barrier(Barrier),
    /// A mutex.
    Mutex(Mutex<Value>),
    /// A monitor.
    Monitor(Mutex<Value>, Condvar),
    /// A read-write lock.
    RwLock(RwLock<Value>),
    /// A channel.
    Channel(Mutex<VecDeque<Value>>, Condvar),
}

impl SyncObject
{
    /// Waits for the specified number of threads.
    pub fn wait(&self) -> Result<bool>
    {
        match self {
            SyncObject::Barrier(barrier) => Ok(barrier.wait().is_leader()),
            _ => Err(Error::Interp(String::from("synchronization object isn't barrier"))),
        }
    }
    
    /// Locks the mutex or the monitor.
    ///
    /// This method locks the mutex or the monitor and then applies the function. The mutex or
    /// the monitor is unlocked after the leaving from the function. The function takes a mutable
    /// reference to the mutex value or the monitor value.
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

    /// Locks the monitor and waits for a notification.
    ///
    /// This methods locks the monitor and then applies the first function. If the first function
    /// returns `true`, this method waits for the notification. This method applies the second
    /// function. Waiting for the notification is repeated if the second function returns `true`.
    /// The third is applied and then the monitor is unlocked if the first function or the second
    /// function returns `false`. All functions take the data and a mutable reference to the 
    /// monitor value. If an error occurs, this method automatically unlocks the monitor.
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

    /// Locks the monitor and waits for a notification until timeout.
    ///
    /// This method locks the monitor and then applies the first function. If the first function
    /// returns `true`, this method wait for the notification until timeout. This method applies
    /// the second function. Waiting for the notification is repeated if the second function
    /// returns `true`. The third function and then the monitor is unlocked if first function or
    /// the second function returns `true`. The second function takes the flag that is set if a
    /// timeout occurs. All functions take the data and a mutable reference to the monitor value.
    /// If an error occurs, this method automatically unlocks the monitor.
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

    /// Locks and notifies one threads.
    ///
    /// This method locks the monitor and then applies the function. One thread is notified and
    /// then the monitor is unlocked after the leaving from the function. The function takes a 
    /// mutable reference to the monitor value. If the function returns an error, the monitor is
    /// automatically unlocked.
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

    /// Locks and notifies all threads.
    ///
    /// This method locks the monitor and then applies the function. All threads are notified and
    /// then the monitor is unlocked after the leaving from the function. The function takes a 
    /// mutable reference to the monitor value. If the function returns an error, the monitor is
    /// automatically unlocked.
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
    
    /// Locks the reader-writer lock with shared read access.
    ///
    /// This method locks the reader-writer lock with shared read access and then applies the
    /// function. The reader-writer lock is unlocked after the leaving from the function. The
    /// function takes an immutable reference to the reader-writer lock value.
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

    /// Locks the reader-writer lock with exclusive write access.
    ///
    /// This method locks the reader-writer lock with exclusive write access and then applies the
    /// function. The reader-writer lock is unlocked after the leaving from the function. The
    /// function takes a mutable reference to the reader-writer lock value.
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
    
    /// Receives a message via the channel.
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

    /// Receives a message via the channel until timeout.
    pub fn recv_timeout(&self, duration: Duration) -> Result<Option<Value>>
    {
        match self {
            SyncObject::Channel(queue, condvar) => {
                let mut instant = Instant::now();
                let mut waiting_duration = duration;
                let mut queue_g = mutex_lock(queue)?;
                let value = loop {
                    if waiting_duration < instant.elapsed() {
                        break None;
                    }
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

    /// Sends the message via the channel.
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
