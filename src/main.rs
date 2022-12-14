use std::os::raw::c_ulong;
use std::process::exit;

use crate::args::parse_args;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use x11::xlib::{XDefaultRootWindow, XOpenDisplay};

use crate::wallpapers::Wallpaper;

mod args;
mod wallpapers;

const TEST: bool = false;
const TEST_WIDTH: u32 = 1920 / 4;
const TEST_HEIGHT: u32 = 1080 / 4;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = if !TEST {
        get_root_window(video_subsystem)
    } else {
        video_subsystem
            .window("wallpepper", TEST_WIDTH, TEST_HEIGHT)
            .opengl()
            .build()
            .unwrap()
    };

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut wallpaper = match parse_args() {
        Ok(wallpaper) => wallpaper,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };

    let mut frame_rate = FPSManager::new();
    frame_rate
        .set_framerate(wallpaper.frame_rate() as u32)
        .unwrap();

    'a: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'a;
            }
        }

        wallpaper.draw(&mut canvas);

        canvas.present();
        frame_rate.delay();
    }
}

/// Opens the X server's root window as an SDL Window
fn get_root_window(video_subsystem: VideoSubsystem) -> Window {
    unsafe {
        let display = XOpenDisplay(std::ptr::null());
        let root_window = XDefaultRootWindow(display);

        if root_window == false as c_ulong {
            panic!("Failed to open X11 display");
        }

        let window = sdl2::sys::SDL_CreateWindowFrom(root_window as *const _);

        Window::from_ll(video_subsystem, window)
    }
}
