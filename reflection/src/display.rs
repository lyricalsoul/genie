use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use crate::Display;
use uefi::{Error, Result};
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::{Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888};
use tinybmp::Bmp;
use alloc::vec;
use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_7X13_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use uefi::table::boot::ScopedProtocol;
use uefi_services::println;
use crate::colors::{BasicPixelColor};
use crate::framebuffer::Framebuffer;
use crate::PixelColor;

const LOGO: &[u8] = include_bytes!("../../assets/birthday.bmp");
const LOGO_DIMENSIONS: (usize, usize) = (480, 197);

impl<'a> Display<'a> {
    pub fn new(gop: ScopedProtocol<'a, GraphicsOutput<'a>>) -> Self {
        let (width, height) = gop.current_mode_info().resolution();
        let mut fb = Framebuffer::new(gop);
        let _ = fb.clear_display((0, 0, 0));

        Display { width, height, framebuffer: fb, }
    }

    pub fn draw_logo(&mut self) -> core::result::Result<(), Error>{
        let bmp = Bmp::from_slice(LOGO).unwrap();
        Image::new(&bmp, self.center_point(LOGO_DIMENSIONS)).draw(self)
    }

    fn center_point(&self, coords: (usize, usize)) -> Point {
        let x = ((self.width - coords.0) / 2) + 1;
        let y = ((self.height - coords.1) / 2) + 1;
        Point::new(x as i32, y as i32)
    }

    pub fn blit(&mut self) -> Result {
        self.framebuffer.blit()
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut PixelColor> {
        self.framebuffer.pixel(x, y)
    }

    pub fn point(&self, x: u32, y: u32) -> Point {
        Point::new(x as i32, y as i32)
    }
    pub fn point_fixed_for_font(&self, point: Point) -> Point {
        let new_y = if point.y < 13 { point.y + 13 } else { point.y };
        Point::new(point.x, new_y)
    }

    // TODO: create a printer ASAP this is horrendous
    pub fn write<'s>(&mut self, text: &str, coord: Point) -> core::result::Result<Point, Error> {
        let small_style = MonoTextStyle::new(&FONT_7X13, PixelColor::white());
        Text::new(text, self.point_fixed_for_font(coord), small_style).draw(self)
    }

    pub fn write_bold<'s>(&mut self, text: &str, coord: Point) -> core::result::Result<Point, Error> {
        let large_style = MonoTextStyle::new(&FONT_7X13_BOLD, PixelColor::white());
        Text::new(text, self.point_fixed_for_font(coord), large_style).draw(self)
    }
}

impl<'a> OriginDimensions for Display<'a> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl<'a> DrawTarget for Display<'a> {
    type Color = PixelColor;
    // `ExampleDisplay` uses a framebuffer and doesn't need to communicate with the display
    // controller to draw pixel, which means that drawing operations can never fail. To reflect
    // this the type `Infallible` was chosen as the `Error` type.
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), ()>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
                // Calculate the index in the framebuffer.
                if coord.x > 0 && coord.y > 0 && let Some(pixel) = self.pixel(coord.x as usize, coord.y as usize) {
                    *pixel = color;
                }
        }

        self.blit()?;

        Ok(())
    }
}