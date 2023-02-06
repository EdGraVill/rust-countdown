use gif::{Encoder as GifEncoder, Frame, Repeat};
use openh264::{
    encoder::{Encoder as H264Encoder, EncoderConfig as H264EncoderConfig},
    formats::YUVBuffer,
};
use std::io::{self, Write};
use webp_animation::Encoder as WebPEncoder;

pub enum Format {
    Mp4 {
        countdown: u32,
        width: u32,
        height: u32,
    },
    Gif {
        countdown: u32,
        width: u32,
        height: u32,
    },
    WebP {
        countdown: u32,
        width: u32,
        height: u32,
    },
}

impl Format {
    pub fn new(format: &str, countdown: u32, width: u32, height: u32) -> Format {
        match format {
            "mp4" => Format::Mp4 {
                countdown,
                width,
                height,
            },
            "gif" => Format::Gif {
                countdown,
                width,
                height,
            },
            "webp" => Format::WebP {
                countdown,
                width,
                height,
            },
            _ => panic!("Unknown format: {}", format),
        }
    }

    pub fn process_file(&self) {
        match self {
            Format::Mp4 {
                countdown: _,
                height: _,
                width: _,
            } => {
                self.process_as_mp4();
            }
            Format::Gif {
                countdown: _,
                height: _,
                width: _,
            } => {
                self.process_as_gif();
            }
            Format::WebP {
                countdown: _,
                height: _,
                width: _,
            } => {
                self.process_as_webp();
            }
        }
    }

    fn process_as_mp4(&self) {
        if let Format::Mp4 {
            countdown,
            width,
            height,
        } = self
        {
            let config = H264EncoderConfig::new(width.clone(), height.clone());
            config.max_frame_rate(1.0);
            let mut encoder = H264Encoder::with_config(config).unwrap();

            let frames =
                super::frames::Frames::new(countdown.clone(), width.clone(), height.clone());
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            for frame in frames {
                let yuv_frame = YUVBuffer::with_rgb(
                    usize::try_from(width.clone()).unwrap(),
                    usize::try_from(height.clone()).unwrap(),
                    &frame.as_raw(),
                );
                let encoded = encoder.encode(&yuv_frame).unwrap();

                handle.write_all(&encoded.to_vec()).unwrap();
            }

            handle.flush().unwrap();
        };
    }

    fn process_as_gif(&self) {
        if let Format::Gif {
            countdown,
            width,
            height,
        } = self
        {
            let stdout = io::stdout();
            let handle = stdout.lock();

            let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
            let mut encoder = GifEncoder::new(
                handle,
                width.clone() as u16,
                height.clone() as u16,
                color_map,
            )
            .unwrap();
            encoder.set_repeat(Repeat::Finite(1)).unwrap();

            let frames =
                super::frames::Frames::new(countdown.clone(), width.clone(), height.clone());

            for frame in frames {
                let gif_frame =
                    Frame::from_rgb(width.clone() as u16, height.clone() as u16, &frame.as_raw());

                encoder.write_frame(&gif_frame).unwrap();
            }
        };
    }

    fn process_as_webp(&self) {
        if let Format::WebP {
            countdown,
            width,
            height,
        } = self
        {
            let mut encoder = WebPEncoder::new((width.clone(), height.clone())).unwrap();

            let mut frames =
                super::frames::Frames::new(countdown.clone(), width.clone(), height.clone());
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            for index in 0..countdown.clone() {
                let frame = frames.next().unwrap();

                encoder
                    .add_frame(frame.as_raw(), index as i32 * 1000)
                    .unwrap();
            }

            let webp = encoder.finalize(countdown.clone() as i32 * 1000).unwrap();

            handle.write_all(&webp).unwrap();
            handle.flush().unwrap();
        };
    }
}
