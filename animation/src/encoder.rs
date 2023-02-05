use gif::{Encoder as GifEncoder, Frame, Repeat};
use openh264::{
    encoder::{Encoder as H264Encoder, EncoderConfig},
    formats::YUVBuffer,
};
use std::io::{self, Write};

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
                countdown,
                height,
                width,
            } => {
                self.process_as_mp4();
            }
            Format::Gif {
                countdown,
                height,
                width,
            } => {
                self.process_as_gif();
            }
            Format::WebP {
                countdown,
                height,
                width,
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
            let config = EncoderConfig::new(width.clone(), height.clone());
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

            handle.flush();
        };
    }

    fn process_as_gif(&self) {
        if let Format::Gif {
            countdown,
            width,
            height,
        } = self
        {};
    }

    fn process_as_webp(&self) {
        if let Format::WebP {
            countdown,
            width,
            height,
        } = self
        {};
    }
}
