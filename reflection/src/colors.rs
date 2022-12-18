use embedded_graphics::pixelcolor::{Rgb555, Rgb565, Rgb888, Bgr555, Bgr565, Bgr888};
use embedded_graphics::prelude::*;
use uefi::proto::console::gop::BltPixel;

// Defines a pixel color that can be converted to a BltPixel
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BasicPixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl BasicPixelColor {
    pub fn white() -> Self {
        BasicPixelColor {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn new(r: u8, g: u8, b: u8) -> BasicPixelColor {
        BasicPixelColor { r, g, b }
    }
    pub fn to_blt_pixel(&self) -> BltPixel {
        BltPixel::new(self.r, self.g, self.b)
    }
}

impl PixelColor for BasicPixelColor {
    type Raw = ();
}

impl From<Rgb555> for BasicPixelColor {
    fn from(color: Rgb555) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl From<Rgb565> for BasicPixelColor {
    fn from(color: Rgb565) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl From<Rgb888> for BasicPixelColor {
    fn from(color: Rgb888) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl From<Bgr555> for BasicPixelColor {
    fn from(color: Bgr555) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl From<Bgr565> for BasicPixelColor {
    fn from(color: Bgr565) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

impl From<Bgr888> for BasicPixelColor {
    fn from(color: Bgr888) -> Self {
        BasicPixelColor {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}