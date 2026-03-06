//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! An interruption module.
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use crate::ctrlc;
use crate::error::*;

/// A trait of interruption checker.
///
/// The interruption checker is used by a `checkintr` built-in function to check whether an
/// interruption is occurred. If the interruption is occurred, this built-in function returns an
/// interruption error.
pub trait IntrCheck
{
    /// Checks whether an interruption is occurred.
    ///
    /// This method returns an interruption error if the interruption is occurred.
    fn check(&self) -> Result<()>;
}

/// A structure of empty interruption checker.
///
/// The empty interruption checker is dummy that ignores interruptions.
#[derive(Copy, Clone, Debug)]
pub struct EmptyIntrChecker;

impl EmptyIntrChecker
{
    /// Create an empty interruption checker.
    pub fn new() -> Self
    { EmptyIntrChecker }
}

impl IntrCheck for EmptyIntrChecker
{
    fn check(&self) -> Result<()>
    { Ok(()) }
}

static INTR_FLAG: AtomicBool = AtomicBool::new(false);

/// A structure of `Ctrl-C` interruption checker.
///
/// The `Ctrl-C` interruption checker checks whether keys `Ctrl-C` are pressed. If the keys
/// `Ctrl-C` is pressed, the `Ctrl-C` interruption checker interprets it as an interruption.
#[derive(Copy, Clone, Debug)]
pub struct CtrlCIntrChecker;

impl CtrlCIntrChecker
{
    /// Creates an Ctrl-C` interruption checker.
    pub fn new() -> Self
    { CtrlCIntrChecker }
    
    /// Initializes the Ctrl-C` interruption checker.
    pub fn initialize() -> Result<()>
    {
        match ctrlc::set_handler(move || INTR_FLAG.store(true, Ordering::SeqCst)) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::Ctrlc(err)),
        }
    }
    
    /// Resets the Ctrl-C` interruption checker.
    pub fn reset()
    { INTR_FLAG.store(false, Ordering::SeqCst); }
}

impl IntrCheck for CtrlCIntrChecker
{
    fn check(&self) -> Result<()>
    {
        if INTR_FLAG.swap(false, Ordering::SeqCst) {
            Err(Error::Intr)
        } else {
            Ok(())
        }
    }
}
