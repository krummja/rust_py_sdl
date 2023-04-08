extern crate pyo3;
extern crate sdl2;
extern crate send_wrapper;

use pyo3::prelude::*;
use sdl2::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;

use send_wrapper::SendWrapper;
use std::rc::Rc;
use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::ops::{Deref, DerefMut};


fn test() -> SendWrapper<Rc<i32>> {
    let value = Rc::new(42);
    let wrapped_value = SendWrapper::new(value);
    wrapped_value
}


#[pyclass]
struct SdlContext {
    sdl_context: sdl2::Sdl,
}

#[pymethods]
impl SdlContext {
    #[new]
    fn new() -> Self {
        Self {
            sdl_context: sdl2::init().unwrap()
        }
    }
}


#[pyfunction]
fn create_context() -> PyResult<()> {
    let _sdl_context = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = _sdl_context.video().unwrap();

    let window: Window = video_subsystem.window("rust_py sdl", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = _sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - 1));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

#[pyclass]
struct ColorWrapper {
    color: Color,
}

#[pymethods]
impl ColorWrapper {
    #[new]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorWrapper {
            color: Color::RGBA(r, g, b, a),
        }
    }

    fn get_r(&self) -> u8 {
        self.color.r
    }

    fn get_g(&self) -> u8 {
        self.color.g
    }

    fn get_b(&self) -> u8 {
        self.color.b
    }

    fn get_a(&self) -> u8 {
        self.color.a
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    let result: String = (a + b).to_string();
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_context, m)?)?;
    m.add_class::<ColorWrapper>()?;
    Ok(())
}
