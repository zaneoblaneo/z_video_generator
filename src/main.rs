#![allow(unused_mut, unused_variables, dead_code)]

use zyuv4_mpeg2::drawing::*;
use zyuv4_mpeg2::*;
use rand::Rng;
use rand::rngs::ThreadRng;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const TIME: usize = 3000; // number of frames to generate
const FRAMERATE_N: u32 = 30;
const FRAMERATE_D: u32 = 1;
const FRAMERATE: f32 = FRAMERATE_N as f32 / FRAMERATE_D as f32;
const DT: f32 = 1f32 / FRAMERATE;

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    dx: f32,
    dy: f32,
    color: u32,
}

impl Circle {
    pub fn random(rng: &mut ThreadRng, 
                  min_x: f32, 
                  min_y: f32, 
                  min_radius: f32, 
                  min_dx: f32,
                  min_dy: f32,
                  max_x: f32, 
                  max_y: f32, 
                  max_radius: f32,
                  max_dx: f32,
                  max_dy: f32) -> Circle {
        Circle { 
            x: rng.gen_range(min_x..max_x) as f32,
            y: rng.gen_range(min_y..max_y) as f32,
            radius: rng.gen_range(min_radius..max_radius) as f32,
            dx: (rng.gen_range(min_dx..max_dx) as f32) * DT,
            dy: (rng.gen_range(min_dy..max_dy) as f32) * DT,
            color: rng.gen::<u32>() | 0xff000000,
        }
    }

}

fn main() -> Result<(), ()> {
    let mut rng = rand::thread_rng();
    // Initialize the canvas
    let mut frames: Vec<Frame> = Vec::<Frame>::new();
    let mut canvas: Canvas = gen_canvas(WIDTH, HEIGHT);
    
    let mut objs: Vec<Circle> = Vec::new();

    for i in 0..10 {
        objs.push(Circle::random(&mut rng, 
                                 100f32, 100f32, 10f32, -50f32, 
                                 -50f32, WIDTH as f32 - 100f32, 
                                 HEIGHT as f32 - 100f32, 
                                 100f32, 200f32, 200f32
        ));
    }
    
    // Initialize the simulation
    for t in 0..TIME {
        // Clear the canvas
        fill_canvas(&mut canvas, RgbColor { r: 0x18, g: 0x18, b: 0x18 });

        // TODO: walk the list of objs, update them all, and render them all.
        for circle in objs.iter_mut() {
            // Draw the circle.
            fill_circle(&mut canvas, circle.x as usize, circle.y as usize, 
                        circle.radius as usize,
                        RgbColor::from_u32(circle.color));
            // Bounds check the circle, and update dx & dy accordingly
            if circle.x + circle.radius + circle.dx >= WIDTH as f32 {
                circle.dx = circle.dx * -1f32;
                circle.color = 0xff000000 | rng.gen::<u32>();
            }
            if circle.y + circle.radius + circle.dy >= HEIGHT as f32 {
                circle.dy = circle.dy * -1f32;
                circle.color = 0xff000000 | rng.gen::<u32>();
            }
            if circle.x - circle.radius + circle.dx <= 0f32 {
                circle.dx = circle.dx * -1f32;
                circle.color = 0xff000000 | rng.gen::<u32>();
            }
            if circle.y + circle.dy - circle.radius <= 0f32 {
                circle.dy = circle.dy * -1f32;
                circle.color = 0xff000000 | rng.gen::<u32>();
            }
            // Update the circle's position.
            circle.x += circle.dx;
            circle.y += circle.dy;
        }
        frames.push(Frame { pixels: canvas.data.clone() });
    }
    render_y4m_file("../test.y4m", 
                    WIDTH, 
                    HEIGHT, 
                    frames, 
                    FRAMERATE_N, 
                    FRAMERATE_D
    )
}
