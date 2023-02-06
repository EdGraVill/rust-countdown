use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

#[derive(Clone, Copy)]
pub struct Frames {
    pub countdown: u32,
    pub width: u32,
    pub height: u32,
    pub frame: u32,
}

impl Frames {
    pub fn new(countdown: u32, width: u32, height: u32) -> Frames {
        Frames {
            countdown,
            width,
            height,
            frame: 0,
        }
    }
}

impl Iterator for Frames {
    type Item = ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let seconds_left = self.countdown - self.frame;
        self.frame += 1;

        if seconds_left == 0 {
            return None;
        }

        let clock_minutes_left = seconds_left / 60;
        let clock_seconds_left = seconds_left % 60;
        let clock = format!("{:02}:{:02}", clock_minutes_left, clock_seconds_left);

        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(
            self.width,
            self.height,
            Rgb([255, 255, 255]),
        );

        let font_data = include_bytes!("fonts/CascadiaCode-SemiBold.otf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

        let scale = Scale { x: 24.0, y: 24.0 };
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs = font.layout(&clock, scale, offset);
        let last_glyph = glyphs.last().clone().unwrap();
        let text_width =
            &last_glyph.position().x + &last_glyph.unpositioned().h_metrics().advance_width;
        let x = (self.width - text_width.clone() as u32) / 2;
        let y = self.height / 2;
        draw_text_mut(
            &mut image,
            Rgb([0, 0, 0]),
            x as i32,
            y as i32,
            scale,
            &font,
            &clock,
        );

        return Some(image);
    }
}
