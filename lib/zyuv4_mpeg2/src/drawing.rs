#![allow(unused_mut, unused_variables, dead_code)]

use crate::*;

pub struct Canvas {
    pub data: Vec<YCbCrColor>,
    pub width: usize,
    pub height: usize,
}

pub struct Rectangle {
    pub x1: usize,
    pub y1: usize,
    pub width: usize,
    pub height: usize,
}

pub fn gen_canvas(w: usize, h: usize) -> Canvas {
    let mut out = Vec::<YCbCrColor>::with_capacity(w * h);
    for _ in 0..w*h {
        out.push(rgb_to_ycbcr(RgbColor{r: 0, g: 0, b: 0}));
    }

    Canvas {
        data: out, 
        width: w, 
        height: h,
    }
}

pub fn fill_canvas(canvas: &mut Canvas, 
               color: RgbColor) {
    for i in 0..canvas.data.len() {
        canvas.data[i] = rgb_to_ycbcr(color);
    }
}

pub fn fill_square(canvas: &mut Canvas, 
                   x: usize, 
                   y: usize, 
                   width: usize, 
                   height: usize,
                   color: RgbColor) {
    for i in y..(y + height) {
        for j in x..(x + width) {
            canvas.data[(i * canvas.width) + j] = rgb_to_ycbcr(color);
        }
    }
}

pub fn fill_circle(canvas: &mut Canvas,
                   x: usize,
                   y: usize,
                   radius: usize,
                   color: RgbColor) {
    for i in (y - radius)..(y + radius) {
        for j in (x - radius)..(x + radius) {
            let (yi, cont) = (y as isize).overflowing_sub(i as isize);
            // if cont { continue; }
            let (xj, cont) = (x as isize).overflowing_sub(j as isize);
            // if cont { continue; }
            if (yi * yi) + (xj * xj) <= (radius * radius) as isize {
                canvas.data[(i * canvas.width) + j] = rgb_to_ycbcr(color);
            }
        }
    }
}
