use ffmpeg_next::codec::Context;
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling;
use ffmpeg_next::software::scaling::Flags;
use ffmpeg_next::{decoder, frame, Error, Rational};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::path::Path;

use crate::{Wallpaper, TEST, TEST_HEIGHT, TEST_WIDTH};

pub struct Video {
    frames: Vec<frame::Video>,
    frame_index: usize,
    frame_rate: Rational,
    width: u32,
    height: u32,
}

impl Wallpaper for Video {
    fn new(args: Vec<String>) -> Result<Self, &'static str> {
        if args.is_empty() {
            return Err("Usage: wallpepper video <file name>");
        }

        ffmpeg_next::init().unwrap();

        let mut ictx = match input(&Path::new(args[0].as_str())) {
            Ok(ictx) => ictx,
            Err(_) => {
                return Err("The specified file could not be opened");
            }
        };
        let input = ictx
            .streams()
            .best(Type::Video)
            .expect("No streams found in video");

        let video_stream_index = input.index();

        let context_decoder = Context::from_parameters(input.parameters()).unwrap();
        let mut decoder = context_decoder.decoder().video().unwrap();

        let (width, height) = if TEST {
            (TEST_WIDTH, TEST_HEIGHT)
        } else {
            (decoder.width(), decoder.height())
        };

        let mut scaler = scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::YUV420P,
            width,
            height,
            Flags::BILINEAR,
        )
        .unwrap();

        let mut frames: Vec<frame::Video> = Vec::new();

        let mut receive_and_process_decoded_frames =
            |decoder: &mut decoder::Video| -> Result<(), Error> {
                let mut decoded = frame::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = frame::Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;

                    frames.push(rgb_frame);
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet).unwrap();
                receive_and_process_decoded_frames(&mut decoder).unwrap();
            }
        }
        decoder.send_eof().unwrap();
        receive_and_process_decoded_frames(&mut decoder).unwrap();

        Ok(Video {
            frames,
            frame_index: 0,
            frame_rate: decoder.frame_rate().unwrap(),
            width,
            height,
        })
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        let frame = &self.frames[self.frame_index];

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::IYUV, self.width, self.height)
            .unwrap();

        texture
            .update_yuv(
                None,
                frame.data(0),
                frame.stride(0),
                frame.data(1),
                frame.stride(1),
                frame.data(2),
                frame.stride(2),
            )
            .unwrap();

        canvas.clear();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(0, 0, self.width, self.height)),
            )
            .unwrap();

        if self.frame_index >= self.frames.len() - 1 {
            self.frame_index = 0;
        } else {
            self.frame_index += 1;
        }
    }

    fn frame_rate(&self) -> f64 {
        f64::from(self.frame_rate)
    }
}
