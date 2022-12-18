#![feature(let_chains)]
#![no_std]
extern crate alloc;




use crate::colors::BasicPixelColor;
use crate::framebuffer::Framebuffer;

pub type PixelColor = BasicPixelColor;
pub mod display;
pub mod colors;
mod framebuffer;

pub struct Display<'a> {
    pub width: usize,
    pub height: usize,
    framebuffer: Framebuffer<'a>,
}