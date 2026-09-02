//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::thread::sleep;
use super::*;

#[test]
fn test_sync_object_wait_waits_for_all_threads()
{
    let sync_object = Arc::new(SyncObject::Barrier(Barrier::new(5)));
    let mut join_handles: Vec<Option<JoinHandle<bool>>> = Vec::new();
    for i in 0..5 {
        let sync_object2 = sync_object.clone();
        let join_handle = thread::spawn(move || {
                sleep(Duration::from_millis(((i + 1) * 100) as u64));
                sync_object2.wait().unwrap()
        });
        join_handles.push(Some(join_handle));
    }
    let mut xs: Vec<bool> = Vec::new();
    for i in 0..5 {
        xs.push(join_handles[i].take().unwrap().join().unwrap());
    }
    assert_eq!(vec![false, false, false, false, true], xs);
}

#[test]
fn test_sync_object_lock_locks_mutex_and_writes_value()
{
    let sync_object = Arc::new(SyncObject::Mutex(Mutex::new(Value::Int(1))));
    let res = sync_object.lock(|value| {
            *value = Value::Int(2);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match &*sync_object {
        SyncObject::Mutex(mutex) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(2), *guard);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_locks_monitor_and_writes_value()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Int(1)), Condvar::new()));
    let res = sync_object.lock(|value| {
            *value = Value::Int(2);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(2), *guard);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_locks_and_returns()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Int(1)), Condvar::new()));
    let mut xs: Vec<i32> = Vec::new();
    let res = sync_object.lock_and_wait(&mut xs, |xs, value| {
            xs.push(1);
            *value = Value::Int(2);
            Ok(false)
    }, |xs, value| {
            xs.push(2);
            *value = Value::Int(3);
            Ok(false)
    }, |xs, value| {
            xs.push(3);
            *value = Value::Int(4);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![1, 3], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(4), *guard);
        },
        _ => assert!(false),
    }    
}

#[test]
fn test_sync_object_lock_and_wait_locks_and_waits_for_notification_and_sync_object_lock_and_notify_one_locks_and_notifies_one()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Bool(false)), Condvar::new()));
    let mut xs: Vec<i32> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            sleep(Duration::from_millis(100));
            sync_object2.lock_and_notify_one(|value| {
                    *value = Value::Bool(true);
                    Ok(())
            }).unwrap();
    });
    let res = sync_object.lock_and_wait(&mut xs, |xs, _| {
            xs.push(1);
            Ok(true)
    }, |xs, value| {
            xs.push(2);
            Ok(!value.to_bool())
    }, |xs, _| {
            xs.push(3);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![1, 2, 3], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(true), *guard);
        },
        _ => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_locks_and_waits_for_notification_and_sync_object_lock_and_notify_all_locks_and_notifies_all()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Bool(false)), Condvar::new()));
    let mut xs: Vec<i32> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            sleep(Duration::from_millis(100));
            sync_object2.lock_and_notify_all(|value| {
                    *value = Value::Bool(true);
                    Ok(())
            }).unwrap();
    });
    let sync_object3 = sync_object.clone();
    let join_handle2 =  thread::spawn(move || {
            let mut ys: Vec<i32> = Vec::new();
            sync_object3.lock_and_wait(&mut ys, |ys, _| {
                    ys.push(1);
                    Ok(true)
            }, |ys, value| {
                    ys.push(2);
                    Ok(!value.to_bool())
            }, |ys, _| {
                    ys.push(3);
                    Ok(())
            }).unwrap();
            ys
    });
    let res = sync_object.lock_and_wait(&mut xs, |xs, _| {
            xs.push(1);
            Ok(true)
    }, |xs, value| {
            xs.push(2);
            Ok(!value.to_bool())
    }, |xs, _| {
            xs.push(3);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![1, 2, 3], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(true), *guard);
        },
        _ => assert!(false),
    }
    match join_handle2.join() {
        Ok(ys) => assert_eq!(vec![1, 2, 3], ys),
        Err(_) => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_executes_loop()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Int(5)), Condvar::new()));
    let mut xs: Vec<i32> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_millis(100));
                sync_object2.lock_and_notify_one(|value| {
                        *value -= Value::Int(1);
                        Ok(())
                }).unwrap();
            }
    });
    let res = sync_object.lock_and_wait(&mut xs, |xs, _| {
            xs.push(1);
            Ok(true)
    }, |xs, value| {
            xs.push(2);
            Ok(value.to_i64() > 0)
    }, |xs, _| {
            xs.push(3);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![1, 2, 2, 2, 2, 2, 3], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(0), *guard);
        },
        _ => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_timeout_locks_and_returns()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Int(1)), Condvar::new()));
    let mut xs: Vec<(i32, bool)> = Vec::new();
    let res = sync_object.lock_and_wait_timeout(Duration::from_millis(100), &mut xs, |xs, value| {
            xs.push((1, false));
            *value = Value::Int(2);
            Ok(false)
    }, |xs, value, is_timeout| {
            xs.push((2, is_timeout));
            *value = Value::Int(3);
            Ok(false)
    }, |xs, value| {
            xs.push((3, false));
            *value = Value::Int(4);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![(1, false), (3, false)], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(4), *guard);
        },
        _ => assert!(false),
    }    
}

#[test]
fn test_sync_object_lock_and_wait_timeout_locks_and_waits_for_notification_and_sync_object_lock_and_notify_one_locks_and_notifies_one()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Bool(false)), Condvar::new()));
    let mut xs: Vec<(i32, bool)> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            sleep(Duration::from_millis(100));
            sync_object2.lock_and_notify_one(|value| {
                    *value = Value::Bool(true);
                    Ok(())
            }).unwrap();
    });
    let res = sync_object.lock_and_wait_timeout(Duration::from_millis(200), &mut xs, |xs, _| {
            xs.push((1, false));
            Ok(true)
    }, |xs, value, is_timeout| {
            xs.push((2, is_timeout));
            Ok(!value.to_bool())
    }, |xs, _| {
            xs.push((3, false));
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![(1, false), (2, false), (3, false)], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(true), *guard);
        },
        _ => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_timeout_locks_and_waits_for_notification_and_sync_object_lock_and_notify_all_locks_and_notifies_all()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Bool(false)), Condvar::new()));
    let mut xs: Vec<(i32, bool)> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            sleep(Duration::from_millis(100));
            sync_object2.lock_and_notify_all(|value| {
                    *value = Value::Bool(true);
                    Ok(())
            }).unwrap();
    });
    let sync_object3 = sync_object.clone();
    let join_handle2 =  thread::spawn(move || {
            let mut ys: Vec<(i32, bool)> = Vec::new();
            sync_object3.lock_and_wait_timeout(Duration::from_millis(200), &mut ys, |ys, _| {
                    ys.push((1, false));
                    Ok(true)
            }, |ys, value, is_timeout| {
                    ys.push((2, is_timeout));
                    Ok(!value.to_bool())
            }, |ys, _| {
                    ys.push((3, false));
                    Ok(())
            }).unwrap();
            ys
    });
    let res = sync_object.lock_and_wait_timeout(Duration::from_millis(200), &mut xs, |xs, _| {
            xs.push((1, false));
            Ok(true)
    }, |xs, value, is_timeout| {
            xs.push((2, is_timeout));
            Ok(!value.to_bool())
    }, |xs, _| {
            xs.push((3, false));
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![(1, false), (2, false), (3, false)], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(true), *guard);
        },
        _ => assert!(false),
    }
    match join_handle2.join() {
        Ok(ys) => assert_eq!(vec![(1, false), (2, false), (3, false)], ys),
        Err(_) => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_timeout_executes_loop()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Int(5)), Condvar::new()));
    let mut xs: Vec<(i32, bool)> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_millis(100));
                sync_object2.lock_and_notify_one(|value| {
                        *value -= Value::Int(1);
                        Ok(())
                }).unwrap();
            }
    });
    let res = sync_object.lock_and_wait_timeout(Duration::from_millis(200), &mut xs, |xs, _| {
            xs.push((1, false));
            Ok(true)
    }, |xs, value, is_timeout| {
            xs.push((2, is_timeout));
            Ok(value.to_i64() > 0)
    }, |xs, _| {
            xs.push((3, false));
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![(1, false), (2, false), (2, false), (2, false), (2, false), (2, false), (3, false)], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Int(0), *guard);
        },
        _ => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_sync_object_lock_and_wait_timeout_interrupts_waiting_for_notification()
{
    let sync_object = Arc::new(SyncObject::Monitor(Mutex::new(Value::Bool(false)), Condvar::new()));
    let mut xs: Vec<(i32, bool)> = Vec::new();
    let sync_object2 = sync_object.clone();
    let join_handle = thread::spawn(move || {
            sleep(Duration::from_millis(200));
            sync_object2.lock_and_notify_one(|value| {
                    *value = Value::Bool(true);
                    Ok(())
            }).unwrap();
    });
    let res = sync_object.lock_and_wait_timeout(Duration::from_millis(100), &mut xs, |xs, _| {
            xs.push((1, false));
            Ok(true)
    }, |xs, _, is_timeout| {
            xs.push((2, is_timeout));
            Ok(false)
    }, |xs, _| {
            xs.push((3, false));
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(vec![(1, false), (2, true), (3, false)], xs);
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(false), *guard);
        },
        _ => assert!(false),
    }
    match join_handle.join() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match &*sync_object {
        SyncObject::Monitor(mutex, _) => {
            let guard = mutex.lock().unwrap();
            assert_eq!(Value::Bool(true), *guard);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_sync_object_read_reads_rw_lock()
{
    let sync_object = Arc::new(SyncObject::RwLock(RwLock::new(Value::Int(1))));
    let mut is_read = false;
    let res = sync_object.read(|_| {
            is_read = true;
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(true, is_read);
    match &*sync_object {
        SyncObject::RwLock(rw_lock) => {
            let guard = rw_lock.read().unwrap();
            assert_eq!(Value::Int(1), *guard);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_sync_object_write_writes_rw_lock()
{
    let sync_object = Arc::new(SyncObject::RwLock(RwLock::new(Value::Int(1))));
    let res = sync_object.write(|value| {
            *value = Value::Int(2);
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match &*sync_object {
        SyncObject::RwLock(rw_lock) => {
            let guard = rw_lock.read().unwrap();
            assert_eq!(Value::Int(2), *guard);
        },
        _ => assert!(false),
    }
}
