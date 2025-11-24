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
use std::mem::size_of;
use std::ops::Range;
use std::result;
use std::sync::Arc;
use plotters::drawing::IntoDrawingArea;
use plotters::element::DashedPathElement;
use plotters::element::DottedPathElement;
use plotters::prelude::*;
use crate::winit::application::ApplicationHandler;
use crate::winit::event::WindowEvent;
use crate::winit::event_loop::ActiveEventLoop;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::utils::*;
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
    
    pub fn to_key_f32(&self) -> f32
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
    { self.to_key_f32() == other.to_key_f32() }
}

impl Ord for F32Key
{
    fn cmp(&self, other: &Self) -> Ordering
    { self.to_key_f32().partial_cmp(&other.to_key_f32()).unwrap() }
}

impl PartialOrd for F32Key
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    { Some(self.cmp(other)) }
}

#[derive(Clone, Debug)]
pub struct Chart
{
    pub title: Option<String>,
    pub window_id: Option<WindowId>,
    pub has_window: bool,
    pub file: Option<String>,
    pub size: Option<(u32, u32)>,
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
    {
        match self {
            HistogramValue::Bool(b) => write!(f, "{:?}", b),
            HistogramValue::Int(n) => write!(f, "{:?}", n),
            HistogramValue::Float(n) => write!(f, "{:?}", n),
            HistogramValue::String(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HistogramAxes
{
    pub x: Vec<HistogramValue>,
    pub y: Range<usize>,
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

const COLORS: [RGBColor; 6] = [RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW];

fn draw_chart2d<T: IntoDrawingArea>(backend: T, chart_desc: &Chart, axes: &Axes2d, serieses: &[Series2d]) -> result::Result<(), Box<dyn error::Error>>
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
        .build_cartesian_2d(axes.x.clone(), axes.y.clone())?;
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

fn draw_chart3d<T: IntoDrawingArea>(backend: T, chart_desc: &Chart, axes: &Axes3d, serieses: &[Series3d]) -> result::Result<(), Box<dyn error::Error>>
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
    let mut chart = chart_builder.build_cartesian_3d(axes.x.clone(), axes.y.clone(), axes.z.clone())?;
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

fn draw_histogram<T: IntoDrawingArea>(backend: T, chart_desc: &Chart, axes: &HistogramAxes, serieses: &[HistogramSeries]) -> result::Result<(), Box<dyn error::Error>>
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
        .build_cartesian_2d(axes.x.as_slice().into_segmented(), axes.y.clone())?;
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
pub enum Plot
{
    Plot(Arc<Chart>, Arc<Axes2d>, Arc<Vec<Series2d>>),
    Plot3(Arc<Chart>, Arc<Axes3d>, Arc<Vec<Series3d>>),
    Histogram(Arc<Chart>, Arc<HistogramAxes>, Arc<Vec<HistogramSeries>>),
}

#[derive(Clone, Debug)]
pub enum PlotterAppEvent
{
    Plot(Arc<Plot>),
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
            _ => (),
        }
    }
}

fn create_size(value: &Value) -> Result<(u32, u32)>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Array(elems) => {
                    if elems.len() != 2 {
                        return Err(Error::Interp(String::from("invalid numner of elements for size")));
                    }
                    let width = match elems.get(0) {
                        Some(elem) => elem.to_i64(),
                        None => return Err(Error::Interp(String::from("no element for size"))),
                    };
                    let height = match elems.get(1) {
                        Some(elem) => elem.to_i64(),
                        None => return Err(Error::Interp(String::from("no element for size"))),
                    };
                    if width < 0 {
                        return Err(Error::Interp(String::from("too small width")));
                    }
                    if width > (u32::MAX as i64) {
                        return Err(Error::Interp(String::from("too large width")));
                    }
                    if height < 0 {
                        return Err(Error::Interp(String::from("too small height")));
                    }
                    if height > (u32::MAX as i64) {
                        return Err(Error::Interp(String::from("too large height")));
                    }
                    Ok((width as u32, height as u32))
                },
                _ => Err(Error::Interp(String::from("unsupported type for size"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for size"))),
    }
}

fn create_chart(value: &Value) -> Result<Chart>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Struct(fields) => {
                    let title = match fields.get(&String::from("title")) {
                        Some(field) => Some(format!("{}", field)),
                        None => None,
                    };
                    let window_id = match fields.get(&String::from("windowid")) {
                        Some(field) => {
                            match field {
                                Value::Object(object) => {
                                    match &**object {
                                        Object::WindowId(tmp_window_id) => Some(*tmp_window_id),
                                        _ => return Err(Error::Interp(String::from("invalid type for window identifier"))),
                                    }
                                },
                                _ => return Err(Error::Interp(String::from("unsupported type for window identifier"))),
                            }
                        },
                        None => None,
                    };
                    let has_window = match fields.get(&String::from("haswindow")) {
                        Some(field) => field.to_bool(),
                        None => true,
                    };
                    let file = match fields.get(&String::from("file")) {
                        Some(field) => Some(format!("{}", field)),
                        None => None,
                    };
                    let size = match fields.get(&String::from("file")) {
                        Some(field) => Some(create_size(field)?),
                        None => None,
                    };
                    Ok(Chart { title, window_id, has_window, file, size, })
                },
                _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
    }
}

fn create_f32_range(value: &Value) -> Result<Range<f32>>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Array(elems) => {
                    if elems.len() != 2 {
                        return Err(Error::Interp(String::from("invalid numner of elements for range")));
                    }
                    let start = match elems.get(0) {
                        Some(elem) => elem.to_f32(),
                        None => return Err(Error::Interp(String::from("no element for range"))),
                    };
                    let end = match elems.get(1) {
                        Some(elem) => elem.to_f32(),
                        None => return Err(Error::Interp(String::from("no element for range"))),
                    };
                    Ok(start..end)
                },
                _ => Err(Error::Interp(String::from("unsupported type for range"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for range"))),
    }
}

fn create_axes2d(value: &Value) -> Result<Axes2d>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Struct(fields) => {
                    let x = match fields.get(&String::from("x")) {
                        Some(field) => create_f32_range(field)?,
                        None => return Err(Error::Interp(String::from("no field x"))),
                    };
                    let y = match fields.get(&String::from("y")) {
                        Some(field) => create_f32_range(field)?,
                        None => return Err(Error::Interp(String::from("no field y"))),
                    };
                    Ok(Axes2d { x, y, })
                },
                _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
    }
}

fn create_axes3d(value: &Value) -> Result<Axes3d>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Struct(fields) => {
                    let x = match fields.get(&String::from("x")) {
                        Some(field) => create_f32_range(field)?,
                        None => return Err(Error::Interp(String::from("no field x"))),
                    };
                    let y = match fields.get(&String::from("y")) {
                        Some(field) => create_f32_range(field)?,
                        None => return Err(Error::Interp(String::from("no field y"))),
                    };
                    let z = match fields.get(&String::from("z")) {
                        Some(field) => create_f32_range(field)?,
                        None => return Err(Error::Interp(String::from("no field z"))),
                    };
                    Ok(Axes3d { x, y, z })
                },
                _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
    }
}

fn create_histogram_values(value: &Value) -> Result<Vec<HistogramValue>>
{
    match value.iter()? {
        Some(iter) => {
            let mut values: Vec<HistogramValue> = Vec::new();
            for elem in iter {
                match elem {
                    Ok(elem) => {
                        match elem {
                            Value::Bool(b) => values.push(HistogramValue::Bool(b)),
                            Value::Int(n) => values.push(HistogramValue::Int(n)),
                            Value::Float(n) => values.push(HistogramValue::Float(n)),
                            _ => values.push(HistogramValue::String(format!("{}", elem))),
                        }
                    },
                    Err(err) => return Err(err),
                }
            }
            Ok(values)
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

fn create_usize_range(value: &Value) -> Result<Range<usize>>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Array(elems) => {
                    if elems.len() != 2 {
                        return Err(Error::Interp(String::from("invalid numner of elements for range")));
                    }
                    let start = match elems.get(0) {
                        Some(elem) => elem.to_i64(),
                        None => return Err(Error::Interp(String::from("no element for range"))),
                    };
                    let end = match elems.get(1) {
                        Some(elem) => elem.to_i64(),
                        None => return Err(Error::Interp(String::from("no element for range"))),
                    };
                    if start < 0 {
                        return Err(Error::Interp(String::from("too small range start")));
                    }
                    if start > (isize::MAX as i64) {
                        return Err(Error::Interp(String::from("too large range start")));
                    }
                    if end < 0 {
                        return Err(Error::Interp(String::from("too small range end")));
                    }
                    if end > (isize::MAX as i64) {
                        return Err(Error::Interp(String::from("too large range end")));
                    }
                    Ok((start as usize)..(end as usize))
                },
                _ => Err(Error::Interp(String::from("unsupported type for range"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for range"))),
    }
}

fn create_histogram_axes(value: &Value) -> Result<HistogramAxes>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(&*object)?;
            match &*object_g {
                MutObject::Struct(fields) => {
                    let x = match fields.get(&String::from("x")) {
                        Some(field) => create_histogram_values(field)?,
                        None => return Err(Error::Interp(String::from("no field x"))),
                    };
                    let y = match fields.get(&String::from("y")) {
                        Some(field) => create_usize_range(field)?,
                        None => return Err(Error::Interp(String::from("no field y"))),
                    };
                    Ok(HistogramAxes { x, y, })
                },
                _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
            }
        },
        _ => Err(Error::Interp(String::from("unsupported type for plotter function"))),
    }
}

fn create_f32s(value: &Value) -> Result<Vec<f32>>
{
    match value.iter()? {
        Some(iter) => {
            let mut xs: Vec<f32> = Vec::new();
            for elem in iter {
                match elem {
                    Ok(elem) => xs.push(elem.to_f32()),
                    Err(err) => return Err(err),
                }
            }
            Ok(xs)
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

fn create_f32s_for_fun_value(interp: &mut Interp, env: &mut Env, fun_value: &Value, xs: &[f32]) -> Result<Vec<f32>>
{
    let mut ys = vec![0.0f32; xs.len()];
    for (i, x) in xs.iter().enumerate() {
        ys[i] = fun_value.apply(interp, env, &[Value::Float(*x)])?.to_f32();
    }
    Ok(ys)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum SeriesKind
{
    Line,
    DashedLine,
    DottedLine,
    Circle,
    Cross,
    Point,
    Triangle,
    XYSurface,
    XZSurface,
    YZSurface,
}

fn str_to_color(s: &str, color_idx: usize) -> Result<RGBColor>
{
    if s.is_empty() {
        match COLORS.get(color_idx) {
            Some(color) => Ok(*color),
            None => Err(Error::Interp(String::from("invalid color index"))),
        }
    } else if s == "r" || s == "red" {
        Ok(RED)
    } else if s == "g" || s == "green" {
        Ok(GREEN)
    } else if s == "b" || s == "blue" {
        Ok(GREEN)
    } else if s == "c" || s == "cyan" {
        Ok(CYAN)
    } else if s == "m" || s == "magenta" {
        Ok(MAGENTA)
    } else if s == "y" || s == "yellow" {
        Ok(YELLOW)
    } else if s == "k" || s == "black" {
        Ok(BLACK)
    } else if s == "w" || s == "white" {
        Ok(WHITE)
    } else {
        Err(Error::Interp(String::from("invalid color")))
    }
}

fn str_to_opt_string(s: &str) -> Option<String>
{
    if !s.is_empty() {
        Some(String::from(s))
    } else {
        None
    }
}

fn create_series_tuple(value: &Value, color_idx: usize) -> Result<(SeriesKind, RGBColor, Option<String>)>
{
    let s = format!("{}", value);
    let (t, u) = match s.split_once(",") {
        Some((tmp_t, tmp_u)) => (tmp_t, tmp_u),
        None => (s.as_str(), ""),
    };
    let (series_kind, t2) = if t.starts_with("-") {
        (SeriesKind::Line, &t[1..])
    } else if t.starts_with("--") {
        (SeriesKind::DashedLine, &t[2..])
    } else if t.starts_with(":") {
        (SeriesKind::DottedLine, &t[1..])
    } else if t.starts_with("o") {
        (SeriesKind::Circle, &t[1..])
    } else if t.starts_with("x") {
        (SeriesKind::Cross, &t[1..])
    } else if t.starts_with(".") {
        (SeriesKind::Point, &t[1..])
    } else if t.starts_with("^") {
        (SeriesKind::Triangle, &t[1..])
    } else if t.starts_with("sxy") {
        (SeriesKind::XYSurface, &t[3..])
    } else if t.starts_with("sxz") {
        (SeriesKind::XZSurface, &t[3..])
    } else if t.starts_with("syz") {
        (SeriesKind::YZSurface, &t[3..])
    } else {
        (SeriesKind::Line, t)
    };
    let color = str_to_color(t2, color_idx)?;
    let label = str_to_opt_string(u);
    Ok((series_kind, color, label))
}

fn create_series2d(interp: &mut Interp, env: &mut Env, x_value: &Value, y_value: &Value, s_value: &Value, color_idx: usize) -> Result<Series2d>
{
    let (series_kind, color, label) = create_series_tuple(s_value, color_idx)?;
    let (xs, ys) = match (x_value.is_fun(), y_value.is_fun()) {
        (false, false) => (create_f32s(x_value)?, create_f32s(y_value)?),
        (false, true) => {
            let tmp_xs = create_f32s(x_value)?;
            let tmp_ys = create_f32s_for_fun_value(interp, env, y_value, tmp_xs.as_slice())?;
            (tmp_xs, tmp_ys)
        },
        (true, false) => {
            let tmp_ys = create_f32s(y_value)?;
            let tmp_xs = create_f32s_for_fun_value(interp, env, x_value, tmp_ys.as_slice())?;
            (tmp_xs, tmp_ys)
        },
        (_, _) => return Err(Error::Interp(String::from("unsupported types for plotter function"))),
    };
    match series_kind {
        SeriesKind::Line => Ok(Series2d::Line(xs, ys, color, label)),
        SeriesKind::DashedLine => Ok(Series2d::DashedLine(xs, ys, color, label)),
        SeriesKind::DottedLine => Ok(Series2d::DottedLine(xs, ys, color, label)),
        SeriesKind::Circle => Ok(Series2d::Circle(xs, ys, color, label)),
        SeriesKind::Cross => Ok(Series2d::Cross(xs, ys, color, label)),
        SeriesKind::Point => Ok(Series2d::Point(xs, ys, color, label)),
        SeriesKind::Triangle => Ok(Series2d::Triangle(xs, ys, color, label)),
        _ => Err(Error::Interp(String::from("invalid series kind")))
    }
}

fn create_surface_f32s(value: &Value, x_count: usize, y_count: usize, x_name: &str, y_name: &str, z_name: &str) -> Result<Vec<f32>>
{
    match value.iter()? {
        Some(iter) => {
            let mut xs: Vec<f32> = Vec::new();
            let mut row_count = 0usize;
            for row in iter {
                match row {
                    Ok(row) => {
                        let ys = create_f32s(&row)?;
                        if ys.len() != x_count {
                            return Err(Error::Interp(format!("number of {} columns isn't equal to number of {} elements", z_name, x_name)))
                        }
                        xs.extend_from_slice(ys.as_slice());
                    },
                    Err(err) => return Err(err),
                }
                match row_count.checked_add(1) {
                    Some(new_row_count) => row_count = new_row_count,
                    None => return Err(Error::Interp(format!("too many {} rows", z_name))),
                }
            }
            if row_count != y_count {
                return Err(Error::Interp(format!("number of {} rows isn't equal to number of {} elements", z_name, y_name)))
            }
            Ok(xs)
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

fn checked_mul_row_count_and_col_count(row_count: usize, col_count: usize, name: &str) -> Result<usize>
{
    if row_count > (isize::MAX as usize) {
        return Err(Error::Interp(String::from("too large number of rows")));
    }
    if col_count > (isize::MAX as usize) {
        return Err(Error::Interp(String::from("too large number of columns")));
    }
    match row_count.checked_mul(col_count) {
        Some(len) => {
            if len > (isize::MAX as usize) {
                return Err(Error::Interp(format!("too large number of {} elements", name)));
            }
            match (len as isize).checked_mul(size_of::<f32>() as isize) {
                Some(_) => Ok(len as usize),
                None => Err(Error::Interp(format!("too large number of {} elements", name))),
            }
        },
        None => Err(Error::Interp(format!("too large number of {} elements", name))),
    }
}

fn create_surface_f32s_for_fun_value(interp: &mut Interp, env: &mut Env, fun_value: &Value, xs: &[f32], ys: &[f32], z_name: &str) -> Result<Vec<f32>>
{
    let len = checked_mul_row_count_and_col_count(ys.len(), xs.len(), z_name)?;
    let mut zs = vec![0.0f32; len];
    for (yi, y) in ys.iter().enumerate() {
        for (xi, x) in xs.iter().enumerate() {
            zs[yi * xs.len() + xi] = fun_value.apply(interp, env, &[Value::Float(*x), Value::Float(*y)])?.to_f32();
        }
    }
    Ok(zs)
}

fn create_indices(xs: &[f32]) -> BTreeMap<F32Key, usize>
{
    let mut idxs: BTreeMap<F32Key, usize> = BTreeMap::new();
    for (i, x) in xs.iter().enumerate() {
        idxs.insert(F32Key::new(*x), i);
    }
    idxs
}

fn create_series3d(interp: &mut Interp, env: &mut Env, x_value: &Value, y_value: &Value, z_value: &Value, s_value: &Value, color_idx: usize) -> Result<Series3d>
{
    let (series_kind, color, label) = create_series_tuple(s_value, color_idx)?;
    let (xs, ys, zs) = match series_kind {
        SeriesKind::XYSurface => {
            let xs = create_f32s(x_value)?;
            let ys = create_f32s(y_value)?;
            let zs = if z_value.is_fun() {
                create_surface_f32s_for_fun_value(interp, env, z_value, xs.as_slice(), ys.as_slice(), "z")?
            } else {
                create_surface_f32s(z_value, xs.len(), ys.len(), "x", "y", "z")?
            };
            let xis = create_indices(xs.as_slice());
            let yis = create_indices(ys.as_slice());
            return Ok(Series3d::XYSurface(xs, ys, zs, color, label, xis, yis));
        },
        SeriesKind::XZSurface => {
            let xs = create_f32s(x_value)?;
            let zs = create_f32s(z_value)?;
            let ys = if y_value.is_fun() {
                create_surface_f32s_for_fun_value(interp, env, y_value, xs.as_slice(), zs.as_slice(), "y")?
            } else {
                create_surface_f32s(y_value, xs.len(), zs.len(), "x", "z", "y")?
            };
            let xis = create_indices(xs.as_slice());
            let zis = create_indices(zs.as_slice());
            return Ok(Series3d::XZSurface(xs, ys, zs, color, label, xis, zis));
        },
        SeriesKind::YZSurface => {
            let ys = create_f32s(y_value)?;
            let zs = create_f32s(z_value)?;
            let xs = if x_value.is_fun() {
                create_surface_f32s_for_fun_value(interp, env, x_value, ys.as_slice(), zs.as_slice(), "x")?
            } else {
                create_surface_f32s(x_value, ys.len(), zs.len(), "y", "z", "x")?
            };
            let yis = create_indices(ys.as_slice());
            let zis = create_indices(zs.as_slice());
            return Ok(Series3d::YZSurface(xs, ys, zs, color, label, yis, zis));
        },
        _ => {
            match (x_value.is_fun(), y_value.is_fun(), z_value.is_fun()) {
                (false, false, false) => (create_f32s(x_value)?, create_f32s(y_value)?, create_f32s(z_value)?),
                (false, true, true) => {
                    let tmp_xs = create_f32s(x_value)?;
                    let tmp_ys = create_f32s_for_fun_value(interp, env, y_value, tmp_xs.as_slice())?;
                    let tmp_zs = create_f32s_for_fun_value(interp, env, z_value, tmp_xs.as_slice())?;
                    (tmp_xs, tmp_ys, tmp_zs)
                },
                (true, false, true) => {
                    let tmp_ys = create_f32s(x_value)?;
                    let tmp_xs = create_f32s_for_fun_value(interp, env, x_value, tmp_ys.as_slice())?;
                    let tmp_zs = create_f32s_for_fun_value(interp, env, z_value, tmp_ys.as_slice())?;
                    (tmp_xs, tmp_ys, tmp_zs)
                },
                (true, true, false) => {
                    let tmp_zs = create_f32s(x_value)?;
                    let tmp_xs = create_f32s_for_fun_value(interp, env, x_value, tmp_zs.as_slice())?;
                    let tmp_ys = create_f32s_for_fun_value(interp, env, y_value, tmp_zs.as_slice())?;
                    (tmp_xs, tmp_ys, tmp_zs)
                },
                (_, _, _) => return Err(Error::Interp(String::from("unsupported types for plotter function"))),
            }
        },
    };
    match series_kind {
        SeriesKind::Line => Ok(Series3d::Line(xs, ys, zs, color, label)),
        SeriesKind::DashedLine => Ok(Series3d::DashedLine(xs, ys, zs, color, label)),
        SeriesKind::DottedLine => Ok(Series3d::DottedLine(xs, ys, zs, color, label)),
        SeriesKind::Circle => Ok(Series3d::Circle(xs, ys, zs, color, label)),
        SeriesKind::Cross => Ok(Series3d::Cross(xs, ys, zs, color, label)),
        SeriesKind::Point => Ok(Series3d::Point(xs, ys, zs, color, label)),
        SeriesKind::Triangle => Ok(Series3d::Triangle(xs, ys, zs, color, label)),
        _ => Err(Error::Interp(String::from("invalid series kind")))
    }
}
