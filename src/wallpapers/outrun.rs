//! Based from https://github.com/joshiemoore/xwow/blob/master/bg/bg_cyber.c

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::f64::consts::PI;

use crate::Wallpaper;

pub struct Outrun {
    line_offset: f64,
    sun_offset: f64,
}

impl Wallpaper for Outrun {
    fn new(_: Vec<String>) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        Ok(Outrun {
            line_offset: 0.0,
            sun_offset: 0.0,
        })
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        let (width, height) = canvas.output_size().unwrap();

        // Clear canvas
        canvas.set_draw_color((0, 0, 0));
        canvas.clear();

        let color = Color::RGB(255, 0, 255);

        // Draw horizon line
        canvas
            .hline(0, width as i16, (height / 2) as i16, color)
            .unwrap();

        // Draw horizontal lines
        let mut y: f64 = height as f64 / 2.0 - 20.0;
        let mut dy: f64 = 1.0;
        while y <= height as f64 {
            let ty: f64 = y + self.line_offset as f64 * dy / 10.0;
            if ty > (height / 2) as f64 {
                let fade_start = (height - 10) as f64;

                let a = if ty > fade_start {
                    255 - ((ty - fade_start) / 10.0 * 255.0) as u8
                } else {
                    255
                };

                canvas
                    .hline(
                        0,
                        width as i16,
                        ty as i16,
                        Color::RGBA(255, 0, 255, a as u8),
                    )
                    .unwrap();
            }

            y += dy;
            dy += 5.0;
        }

        // Draw vertical lines
        let mut theta: u8 = 0;
        while theta < 180 {
            let trad = theta as f64 * (PI / 180.0);

            // BUG : Vertical lines are 2000 pixel long, so they won't fit a big screen (4K)
            canvas
                .aa_line(
                    (width / 2) as i16,
                    (height / 2) as i16,
                    ((width / 2) as f64 + (trad.cos() * 2000.0)) as i16,
                    ((height / 2) as f64 + (trad.sin() * 2000.0)) as i16,
                    color,
                )
                .unwrap();

            theta += 6;
        }

        // Draw sun
        let middle = Point::new((width / 2) as i32, (height / 2) as i32);
        let radius = (width / 8) as i32;

        let sun_line = |i: f64, y: i16| {
            // Pythagorean theorem
            let a = radius.pow(2);
            let b = (radius - i as i32).pow(2);
            let c = f64::sqrt((a - b) as f64) as i32;

            // Purple-yellow gradient
            let t = (i as f64 + 1.0) / radius as f64;
            let color = Color::RGB(255, (t * 192.0) as u8, (192.0 - t * 255.0) as u8);

            canvas
                .hline(
                    (middle.x() - c) as i16,
                    (middle.x() + c) as i16,
                    y as i16,
                    color,
                )
                .unwrap();
        };

        let mut i = 0.0;
        let mut y = (middle.y() - radius) as f64;
        let first_band = y + (radius / 2) as f64 + (self.sun_offset * 5.0);

        while y < first_band {
            sun_line(i as f64, y as i16);
            i += 1.0;
            y += 1.0;
        }

        let mut spacing = 5.0 + self.sun_offset;
        'a: loop {
            i += spacing;
            y += spacing;

            let band = y + spacing * 2.0;

            let will_overflow = band >= middle.y() as f64;

            if !will_overflow {
                while y < band {
                    sun_line(i, y as i16);
                    i += 1.0;
                    y += 1.0;
                }
            } else {
                while y < band {
                    if y >= middle.y() as f64 {
                        break 'a;
                    }
                    sun_line(i, y as i16);

                    i += 1.0;
                    y += 1.0;
                }
            }

            spacing += 3.0;
        }

        if self.line_offset >= 9.9 {
            self.line_offset = 0.0;
        } else {
            self.line_offset += 0.2;
        }

        if self.sun_offset >= 2.9 {
            self.sun_offset = 0.0;
        } else {
            self.sun_offset += 0.02;
        }
    }
}
