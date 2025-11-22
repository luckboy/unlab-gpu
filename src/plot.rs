//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::winit::application::ApplicationHandler;
use crate::winit::event::WindowEvent;
use crate::winit::event_loop::ActiveEventLoop;
use crate::value::WindowId;

#[derive(Clone, Debug)]
pub enum PlotterAppEvent
{
    Quit,
}

#[derive(Clone, Debug)]
pub struct PlotterApp;

impl PlotterApp
{
    pub fn new() -> Self
    { PlotterApp }
}

impl ApplicationHandler<PlotterAppEvent> for PlotterApp
{
    fn resumed(&mut self, _event_loop: &ActiveEventLoop)
    {}
    
    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent)
    {}

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: PlotterAppEvent)
    {
        match event {
            PlotterAppEvent::Quit => event_loop.exit(),
        }
    }
}
