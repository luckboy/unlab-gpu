//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::ops::Range;
use std::result;
use plotters::drawing::IntoDrawingArea;
use plotters::element::DashedPathElement;
use plotters::element::DottedPathElement;
use plotters::prelude::*;
use crate::winit::application::ApplicationHandler;
use crate::winit::event::WindowEvent;
use crate::winit::event_loop::ActiveEventLoop;
use crate::error::*;
use crate::value::*;

#[derive(Copy, Clone, Debug)]
pub struct F32Key
{
    value: f32,
}

impl F32Key
{
    pub fn new(value: f32) -> Self
    { F32Key { value, } }
    
    pub fn to_f32(&self) -> f32
    { self.value }
    
    pub fn to_f32_without_nan(&self) -> f32
    {
        if !self.value.is_nan() {
            self.value
        } else {
            -f32::INFINITY
        }
    }
}

impl Eq for F32Key
{}

impl PartialEq for F32Key
{
    fn eq(&self, other: &Self) -> bool
    { self.to_f32_without_nan() == other.to_f32_without_nan() }    
}

impl Ord for F32Key
{
    fn cmp(&self, other: &Self) -> Ordering
    { self.to_f32_without_nan().partial_cmp(&other.to_f32_without_nan()).unwrap() }
}

impl PartialOrd for F32Key
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    { Some(self.cmp(other)) }
}

#[derive(Clone, Debug)]
pub struct Axes2d
{
    pub x: Range<f32>,
    pub y: Range<f32>,
}

#[derive(Clone, Debug)]
pub struct Axes3d
{
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

#[derive(Clone)]
pub enum HistogramValue
{
    Bool(bool),
    Int(i64),
    Float(f32),
    String(String),
}

impl HistogramValue
{
    pub fn to_bool(&self) -> bool
    {
        match self {
            HistogramValue::Bool(b) => *b,
            HistogramValue::Int(n) => *n != 0,
            HistogramValue::Float(n) => *n != 0.0,
            HistogramValue::String(_) => true,
        }
    }

    pub fn to_i64(&self) -> i64
    {
        match self {
            HistogramValue::Bool(b) => if *b { 1 } else { 0 },
            HistogramValue::Int(n) => *n,
            HistogramValue::Float(n) => *n as i64,
            HistogramValue::String(_) => 1,
        }
    }

    pub fn to_f32(&self) -> f32
    {
        match self {
            HistogramValue::Bool(b) => if *b { 1.0 } else { 0.0 },
            HistogramValue::Int(n) => *n as f32,
            HistogramValue::Float(n) => *n,
            HistogramValue::String(_) => 1.0,
        }
    }

    pub fn to_opt_bool(&self) -> Option<bool>
    {
        match self {
            HistogramValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn to_opt_i64(&self) -> Option<i64>
    {
        match self {
            HistogramValue::Int(n) => Some(*n),
            HistogramValue::Float(n) => Some(*n as i64),
            _ => None,
        }
    }

    pub fn to_opt_f32(&self) -> Option<f32>
    {
        match self {
            HistogramValue::Int(n) => Some(*n as f32),
            HistogramValue::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn to_opt_string(&self) -> Option<String>
    {
        match self {
            HistogramValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl PartialEq for HistogramValue
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other) {
            (HistogramValue::Bool(b), HistogramValue::Bool(b2)) => b == b2,
            (HistogramValue::Int(n), HistogramValue::Int(n2)) => n == n2,
            (HistogramValue::Int(_) | HistogramValue::Float(_), HistogramValue::Int(_) | HistogramValue::Float(_)) => self.to_f32() == other.to_f32(),
            (HistogramValue::String(s), HistogramValue::String(s2)) => s == s2, 
            (_, _) => false,
        }
    }
}

impl fmt::Display for HistogramValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            HistogramValue::Bool(b) => write!(f, "{}", b),
            HistogramValue::Int(n) => write!(f, "{}", n),
            HistogramValue::Float(n) => write!(f, "{}", n),
            HistogramValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for HistogramValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { fmt::Display::fmt(self, f) }
}

#[derive(Clone, Debug)]
pub struct HistogramAxes
{
    pub x: Vec<HistogramValue>,
    pub y: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Chart<T>
{
    pub title: Option<String>,
    pub axes: T,
    pub window_id: Option<WindowId>,
    pub has_window: bool,
    pub file: Option<String>,
    pub size: Option<(u32, u32)>,
}

#[derive(Clone, Debug)]
pub enum Series2d
{
    Line(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    DashedLine(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    DottedLine(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Circle(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Cross(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Point(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Triangle(Vec<f32>, Vec<f32>, RGBColor, Option<String>),
}

#[derive(Clone, Debug)]
pub enum Series3d
{
    Line(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    DashedLine(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    DottedLine(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Circle(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Cross(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Point(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    Triangle(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>),
    XYSurface(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>, BTreeMap<F32Key, usize>, BTreeMap<F32Key, usize>),
    XZSurface(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>, BTreeMap<F32Key, usize>, BTreeMap<F32Key, usize>),
    YZSurface(Vec<f32>, Vec<f32>, Vec<f32>, RGBColor, Option<String>, BTreeMap<F32Key, usize>, BTreeMap<F32Key, usize>),
}

#[derive(Clone, Debug)]
pub struct HistogramSeries(pub Vec<HistogramValue>, pub RGBColor, pub Option<String>);

const TITLE_FONT_SIZE: i32 = 40;
const MARGIN: i32 = 5;
const X_LABEL_AREA_SIZE: i32 = 30;
const Y_LABEL_AREA_SIZE: i32 = 30;

const LEGEND_WIDTH: i32 = 20;
const LEGEND_HEIGHT: i32 = 10;

const DASH_SIZE: i32 = 8;
const DASH_SPACING: i32 = 2;
const DOT_SHIFT: i32 = 0;
const DOT_SPACING: i32 = 2;
const MARKER_SIZE: i32 = 4;
const POINT_SIZE: i32 = 1;

const SURFACE_MIX: f64 = 0.2;
const HISTOGRAM_MIX: f64 = 0.2;

fn draw_chart2d<T: IntoDrawingArea>(backend: T, chart_desc: &Chart<Axes2d>, serieses: &[Series2d]) -> result::Result<(), Box<dyn error::Error>>
    where T::ErrorType: 'static
{
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart_builder = ChartBuilder::on(&root);
    match &chart_desc.title {
        Some(title) => {
            chart_builder.caption(title, ("sans-serif", TITLE_FONT_SIZE).into_font());
        },
        None => (),
    }
    let mut chart = chart_builder
        .margin(MARGIN)
        .x_label_area_size(X_LABEL_AREA_SIZE)
        .y_label_area_size(Y_LABEL_AREA_SIZE)
        .build_cartesian_2d(chart_desc.axes.x.clone(), chart_desc.axes.y.clone())?;
    chart.configure_mesh().draw()?;
    for series in serieses {
        match series {
            Series2d::Line(xs, ys, color, label) => {
                let series_anno = chart.draw_series(LineSeries::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), *color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], &color2));
            },
            Series2d::DashedLine(xs, ys, color, label) => {
                let series_anno = chart.draw_series(DashedLineSeries::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), DASH_SIZE, DASH_SPACING, Into::<ShapeStyle>::into(color)))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| DashedPathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], DASH_SIZE, DASH_SPACING, Into::<ShapeStyle>::into(&color2)));
            },
            Series2d::DottedLine(xs, ys, color, label) => {
                let color2 = *color;
                let series_anno = chart.draw_series(DottedLineSeries::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), DOT_SHIFT, DOT_SPACING, move |p| Circle::new(p, POINT_SIZE, Into::<ShapeStyle>::into(&color2).filled())))?;
                let color3 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| DottedPathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], DOT_SHIFT, DOT_SPACING, move |p| Circle::new(p, POINT_SIZE, Into::<ShapeStyle>::into(&color3).filled())));
            },
            Series2d::Circle(xs, ys, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Circle<_, i32>, i32>::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Circle::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
            Series2d::Cross(xs, ys, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Cross<_, i32>, i32>::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Cross::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
            Series2d::Point(xs, ys, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Circle<_, i32>, i32>::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), POINT_SIZE, color.filled()))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Circle::new((x + LEGEND_WIDTH / 2, y), POINT_SIZE, color2.filled()));
            },
            Series2d::Triangle(xs, ys, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, TriangleMarker<_, i32>, i32>::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| TriangleMarker::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
        }
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

fn draw_chart3d<T: IntoDrawingArea>(backend: T, chart_desc: &Chart<Axes3d>, serieses: &[Series3d]) -> result::Result<(), Box<dyn error::Error>>
    where T::ErrorType: 'static
{
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart_builder = ChartBuilder::on(&root);
    match &chart_desc.title {
        Some(title) => {
            chart_builder.caption(title, ("sans-serif", TITLE_FONT_SIZE).into_font());
        },
        None => (),
    }
    let mut chart = chart_builder.build_cartesian_3d(chart_desc.axes.x.clone(), chart_desc.axes.y.clone(), chart_desc.axes.z.clone())?;
    chart.configure_axes().draw()?;
    for series in serieses {
        match series {
            Series3d::Line(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(LineSeries::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), *color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], &color2));
            },
            Series3d::DashedLine(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(DashedLineSeries::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), DASH_SIZE, DASH_SPACING, Into::<ShapeStyle>::into(color)))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| DashedPathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], DASH_SIZE, DASH_SPACING, Into::<ShapeStyle>::into(&color2)));
            },
            Series3d::DottedLine(xs, ys, zs, color, label) => {
                let color2 = *color;
                let series_anno = chart.draw_series(DottedLineSeries::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), DOT_SHIFT, DOT_SPACING, move |p| Circle::new(p, POINT_SIZE, Into::<ShapeStyle>::into(&color2).filled())))?;
                let color3 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| DottedPathElement::new(vec![(x, y), (x + LEGEND_WIDTH, y)], DOT_SHIFT, DOT_SPACING, move |p| Circle::new(p, POINT_SIZE, Into::<ShapeStyle>::into(&color3).filled())));
            },
            Series3d::Circle(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Circle<_, i32>, i32>::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Circle::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
            Series3d::Cross(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Cross<_, i32>, i32>::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Cross::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
            Series3d::Point(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, Circle<_, i32>, i32>::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), POINT_SIZE, color.filled()))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Circle::new((x + LEGEND_WIDTH / 2, y), POINT_SIZE, color2.filled()));
            },
            Series3d::Triangle(xs, ys, zs, color, label) => {
                let series_anno = chart.draw_series(PointSeries::<_, _, TriangleMarker<_, i32>, i32>::new(xs.iter().zip(ys.iter()).zip(zs.iter()).map(|((x, y), z)| (*x, *y, *z)), MARKER_SIZE, color))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| TriangleMarker::new((x + LEGEND_WIDTH / 2, y), MARKER_SIZE, &color2));
            },
            Series3d::XYSurface(xs, ys, zs, color, label, xis, yis) => {
                let series_anno = chart.draw_series(SurfaceSeries::xoy(xs.iter().map(|x| *x), ys.iter().map(|y| *y), |x, y| {
                        let xi = xis.get(&F32Key::new(x));
                        let yi = yis.get(&F32Key::new(y));
                        match (xi, yi) {
                            (Some(xi), Some(yi)) => zs.get(yi * xs.len() + xi).map(|z| *z).unwrap_or(0.0),
                            (_, _) => 0.0,
                        }
                }).style(color.mix(SURFACE_MIX).filled()))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Rectangle::new([(x, y - LEGEND_HEIGHT / 2), (x + LEGEND_WIDTH, y + LEGEND_HEIGHT / 2)], color2.mix(SURFACE_MIX).filled()));
            },
            Series3d::XZSurface(xs, ys, zs, color, label, xis, zis) => {
                let series_anno = chart.draw_series(SurfaceSeries::xoz(xs.iter().map(|x| *x), zs.iter().map(|z| *z), |x, z| {
                        let xi = xis.get(&F32Key::new(x));
                        let zi = zis.get(&F32Key::new(z));
                        match (xi, zi) {
                            (Some(xi), Some(zi)) => ys.get(zi * xs.len() + xi).map(|y| *y).unwrap_or(0.0),
                            (_, _) => 0.0,
                        }
                }).style(color.mix(SURFACE_MIX).filled()))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Rectangle::new([(x, y - LEGEND_HEIGHT / 2), (x + LEGEND_WIDTH, y + LEGEND_HEIGHT / 2)], color2.mix(SURFACE_MIX).filled()));
            },
            Series3d::YZSurface(xs, ys, zs, color, label, yis, zis) => {
                let series_anno = chart.draw_series(SurfaceSeries::yoz(ys.iter().map(|y| *y), zs.iter().map(|z| *z), |y, z| {
                        let yi = yis.get(&F32Key::new(y));
                        let zi = zis.get(&F32Key::new(z));
                        match (yi, zi) {
                            (Some(yi), Some(zi)) => xs.get(zi * ys.len() + yi).map(|x| *x).unwrap_or(0.0),
                            (_, _) => 0.0,
                        }
                }).style(color.mix(SURFACE_MIX).filled()))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Rectangle::new([(x, y - LEGEND_HEIGHT / 2), (x + LEGEND_WIDTH, y + LEGEND_HEIGHT / 2)], color2.mix(SURFACE_MIX).filled()));
            },
        }
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

fn draw_histogram<T: IntoDrawingArea>(backend: T, chart_desc: &Chart<HistogramAxes>, serieses: &[HistogramSeries]) -> result::Result<(), Box<dyn error::Error>>
    where T::ErrorType: 'static
{
    let root = backend.into_drawing_area();
    let mut chart_builder = ChartBuilder::on(&root);
    match &chart_desc.title {
        Some(title) => {
            chart_builder.caption(title, ("sans-serif", TITLE_FONT_SIZE).into_font());
        },
        None => (),
    }
    let mut chart = chart_builder
        .margin(MARGIN)
        .x_label_area_size(X_LABEL_AREA_SIZE)
        .y_label_area_size(Y_LABEL_AREA_SIZE)
        .build_cartesian_2d(chart_desc.axes.x.as_slice().into_segmented(), chart_desc.axes.y.clone())?;
    for series in serieses {
        match series {
            HistogramSeries(data, color, label) => {
                let series_anno = chart.draw_series(Histogram::vertical(&chart).style(color.mix(HISTOGRAM_MIX).filled()).data(data.iter().map(|v| (v, 1))))?;
                let color2 = *color;
                match label {
                    Some(label) => series_anno.label(label.as_str()),
                    None => series_anno,
                }.legend(move |(x, y)| Rectangle::new([(x, y - LEGEND_HEIGHT / 2), (x + LEGEND_WIDTH, y + LEGEND_HEIGHT / 2)], color2.mix(HISTOGRAM_MIX).filled()));
            },
        }
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

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
