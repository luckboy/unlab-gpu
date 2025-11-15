//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use crate::ctrlc;
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

static INTR_FLAG: AtomicBool = AtomicBool::new(true);

#[derive(Copy, Clone, Debug)]
pub struct CtrlCIntrChecker;

impl CtrlCIntrChecker
{
    pub fn new() -> Self
    { CtrlCIntrChecker }
    
    pub fn initialize() -> Result<()>
    {
        match ctrlc::set_handler(move || INTR_FLAG.store(true, Ordering::SeqCst)) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::Ctrlc(err)),
        }
    }
    
    pub fn reset()
    { INTR_FLAG.store(false, Ordering::SeqCst); }
}

impl IntrCheck for CtrlCIntrChecker
{
    fn check(&self) -> Result<()>
    {
        if INTR_FLAG.load(Ordering::SeqCst) {
            INTR_FLAG.store(false, Ordering::SeqCst);
            Err(Error::Intr)
        } else {
            Ok(())
        }
    }
}
