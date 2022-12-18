use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::table::boot::ScopedProtocol;
use uefi_services::println;
use crate::colors::BasicPixelColor;

pub struct Framebuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<BasicPixelColor>,
    gop: ScopedProtocol<'a, GraphicsOutput<'a>>
}

impl<'a> Framebuffer<'a> {
    pub fn new(gop: ScopedProtocol<'a, GraphicsOutput<'a>>) -> Self {
        let mode_info = gop.current_mode_info();
        let (width, height) = mode_info.resolution();

        println!("display resolution: {}x{}", width, height);
        println!("display stride: {}", mode_info.stride());
        println!("display pixel format: {:?}", mode_info.pixel_format());

        Framebuffer {
            width,
            height,
            pixels: vec![BasicPixelColor::new(0, 0, 0); width * height],
            gop
        }
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BasicPixelColor> {
        self.pixels.get_mut((y + 8) * self.width + x)
    }

    pub fn draw_rectangle(&mut self, coords: (usize, usize), dimensions: (usize, usize), color: BasicPixelColor) -> uefi::Result {
        for x in coords.0..(coords.0 + dimensions.0) {
            for y in coords.1..(coords.1 + dimensions.1) {
                if let Some(pixel) = self.pixel(x, y) {
                    *pixel = color;
                }
            }
        }

        self.gop.blt(BltOp::VideoFill {
            dest: coords,
            dims: dimensions,
            color: BltPixel::new(color.r, color.g, color.b),
        })
    }

    pub fn clear_display(&mut self, color: (u8, u8, u8)) -> uefi::Result {
        self.gop.blt(BltOp::VideoFill {
            color: BltPixel::new(color.0, color.1, color.2),
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    pub fn blit(&mut self) -> uefi::Result {
        self.get_blt_pixels_and_blit()
    }

    fn get_blt_pixels_and_blit (&mut self) -> uefi::Result {
        let mut blt_pixels = Vec::new();
        for pixel in self.pixels.iter() {
            blt_pixels.push(pixel.to_blt_pixel());
        }

        self.gop.blt(BltOp::BufferToVideo {
            buffer: &*blt_pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}