use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

pub mod drawing;

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub pixels: Vec<YCbCrColor>,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct YCbCrColor {
    pub y: u8,
    pub cb: u8,
    pub cr: u8,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn from_u32(i: u32) -> Self {
        // AARRGGBB
        Self {
            r: ((i >> (8*2)) & 0xff) as u8,
            g: ((i >> (8*1)) & 0xff) as u8,
            b: ((i >> (8*0)) & 0xff) as u8,
        }
    }
    pub fn from_components(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b,
        }
    }
}

/// converts a `RgbaColor` into a `YCbCrColor`. Formula was ripped from: 
/// (Tsoding's 2mpeg4yuv/render.rs)<https://github.com/tsoding/2mpeg4yuv/blob/479490b97ba43d14c8e57c2139787ed9c224800d/src/render.rs#L13-L21>
pub fn rgb_to_ycbcr(input: RgbColor) -> YCbCrColor{
    let rf = input.r as f32;
    let gf = input.g as f32;
    let bf = input.b as f32;
    YCbCrColor {
        y:  (16.0f32  + 65.738f32  * rf/256.0f32 + 
                        129.057    * gf/256.0f32 + 
                        25.064f32  * bf/256.0) as u8,
        cb: (128.0f32 - 37.945f32  * rf/256.0f32 - 
                        74.494f32  * gf/256.0f32 + 
                        112.439f32 * bf/256.0f32) as u8,
        cr: (128.0f32 + 112.439f32 * rf/256.0f32 -
                        94.154f32  * gf/256.0f32 - 
                        18.285f32  * bf/256.0f32) as u8,
    }
}

fn map_err(err: impl std::error::Error) -> () {
    eprintln!("{:?}", err);
    ()
}

pub fn render_y4m_file<'a>(fp: impl AsRef<Path>,
                           width: usize, height: usize,
                           frames: Vec<Frame>,
                           framerate_numerator: u32,
                           framerate_denominator: u32) -> Result<(), ()> {
    let mut out_stream: BufWriter<File> = BufWriter::new(File::create(fp).map_err(map_err)?);
    write!(&mut out_stream, "YUV4MPEG2 W{} H{} F{}:{} Ip A0:0 C444\n", width, height, 
           framerate_numerator, framerate_denominator).map_err(map_err)?;
    for frame in frames {
        out_stream.write(b"FRAME\n").map_err(map_err)?;
        let mut yb: Vec<u8> = Vec::<u8>::new();
        let mut cbb: Vec<u8> = Vec::<u8>::new();
        let mut crb: Vec<u8> = Vec::<u8>::new();

        for pixel in frame.pixels {
            yb.push(pixel.y);
            cbb.push(pixel.cb);
            crb.push(pixel.cr);
        }
        out_stream.write_all(&yb).map_err(map_err)?;
        out_stream.write_all(&cbb).map_err(map_err)?;
        out_stream.write_all(&crb).map_err(map_err)?;
        out_stream.flush().map_err(map_err)?;
    }
    Ok(())
}
