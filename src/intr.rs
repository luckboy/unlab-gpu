//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
#[cfg(target_family = "unix")]
use std::io;
#[cfg(target_family = "unix")]
use std::mem::MaybeUninit;
#[cfg(target_family = "unix")]
use std::ptr::null_mut;
#[cfg(target_family = "unix")]
use libc::SA_RESTART;
use libc::SIGINT;
use libc::c_int;
use libc::sighandler_t;
#[cfg(target_family = "unix")]
use libc::sigset_t;
#[cfg(target_family = "unix")]
use libc::sigaction;
#[cfg(target_family = "unix")]
use libc::sigfillset;
#[cfg(not(target_family = "unix"))]
use libc::signal;
use crate::error::*;

pub trait IntrCheck
{
    fn check(&self) -> Result<()>;
}

#[derive(Copy, Clone, Debug)]
pub struct EmptyIntrChecker;

impl EmptyIntrChecker
{
    pub fn new() -> Self
    { EmptyIntrChecker }
}

impl IntrCheck for EmptyIntrChecker
{
    fn check(&self) -> Result<()>
    { Ok(()) }
}

static mut INTR_FLAG: bool = false;

extern "C" fn unlab_gpu_signal_handler(_sig: c_int)
{ unsafe { INTR_FLAG = true; } }

#[cfg(target_family = "unix")]
#[derive(Copy, Clone, Debug)]
pub struct SignalHandler
{
    sigaction: MaybeUninit<sigaction>,
}

#[cfg(not(target_family = "unix"))]
#[derive(Copy, Clone, Debug)]
pub struct SignalHandler
{
    signal_handler: sighandler_t,
}

#[derive(Copy, Clone, Debug)]
pub struct CtrlCIntrChecker;

impl CtrlCIntrChecker
{
    pub fn new() -> Self
    { CtrlCIntrChecker }
    
    #[cfg(target_family = "unix")]
    pub fn set_signal_handler() -> Result<SignalHandler>
    {
        let mut saved_signal_handler = SignalHandler { sigaction: MaybeUninit::uninit(), };
        let mut new_sigaction: MaybeUninit<sigaction> = MaybeUninit::uninit();
        unsafe {
            new_sigaction.assume_init_mut().sa_sigaction = unlab_gpu_signal_handler as sighandler_t;
            sigfillset(&mut new_sigaction.assume_init_mut().sa_mask as *mut sigset_t);
            new_sigaction.assume_init_mut().sa_flags = SA_RESTART;
        }
        let res = unsafe { sigaction(SIGINT, &new_sigaction.assume_init() as *const sigaction, saved_signal_handler.sigaction.assume_init_mut() as *mut sigaction) };
        if res != -1 {
            Ok(saved_signal_handler)
        } else {
            Err(Error::Io(io::Error::last_os_error()))
        }
    }
    
    #[cfg(target_family = "unix")]
    pub fn restore_signal_handler(signal_handler: &SignalHandler) -> Result<()>
    {
        let res = unsafe { sigaction(SIGINT, &signal_handler.sigaction.assume_init() as *const sigaction, null_mut()) };
        if res != -1 {
            Ok(())
        } else {
            Err(Error::Io(io::Error::last_os_error()))
        }
    }
    
    #[cfg(not(target_family = "unix"))]
    pub fn set_signal_handler() -> Result<SignalHandler>
    {
        let signal_handler = unsafe { signal(SIGINT, unlab_gpu_signal_handler as sighandler_t) };
        Ok(SignalHandler { signal_handler, })
    }

    #[cfg(not(target_family = "unix"))]
    pub fn restore_signal_handler(signal_handler: &SignalHandler) -> Result<()>
    {
        unsafe { signal(SIGINT, signal_handler.signal_handler as sighandler_t); }
        Ok(())        
    }
    
    pub fn reset()
    { unsafe { INTR_FLAG = false; } }
}

impl IntrCheck for CtrlCIntrChecker
{
    fn check(&self) -> Result<()>
    {
        if unsafe { INTR_FLAG } {
            unsafe { INTR_FLAG = false; }
            Err(Error::Intr)
        } else {
            Ok(())
        }
    }
}
