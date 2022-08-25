use sdl2::render::WindowCanvas;

use crate::wallpapers::Wallpaper;

pub struct Chroma {
    counter: u16,
}

impl Wallpaper for Chroma {
    fn new(_: Vec<String>) -> Result<Self, &'static str> {
        Ok(Chroma { counter: 0 })
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(hsv_to_rgb(self.counter, 1.0, 1.0));
        canvas.clear();

        if self.counter < 359 {
            self.counter += 1;
        } else {
            self.counter = 0;
        }
    }
}

/// Converts an HSV color to an RGB color
///
/// See https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
fn hsv_to_rgb(hue: u16, saturation: f64, value: f64) -> (u8, u8, u8) {
    debug_assert!(hue < 360);
    debug_assert!(saturation >= 0.0);
    debug_assert!(saturation <= 1.0);
    debug_assert!(value >= 0.0);
    debug_assert!(value <= 1.0);

    let f = |x: f64| {
        let k: f64 = (x + (hue as f64) / 60.0) % 6.0;
        value - value * saturation * fmax(0.0, fmin(fmin(k, 1.0), 4.0 - k))
    };

    (
        (f(5.0) * 256.0) as u8,
        (f(3.0) * 256.0) as u8,
        (f(1.0) * 256.0) as u8,
    )
}

fn fmin<T: PartialOrd<T>>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn fmax<T: PartialOrd<T>>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
