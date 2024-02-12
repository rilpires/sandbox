#![allow(clippy::all, warnings, unused)]

use datatype::Vector2;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use sdl2::libc::{abs, rand};
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::rwops::RWops;
use sdl2::sys::{random, random_data};
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::VecDeque;
use std::path::Path;
use std::time::Duration;
use std::u32;
use std::cmp::*;
use rand::Rng;
use sdl2::render::{Canvas, TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use crate::world_grid::*;
use crate::toolbox::*;
use crate::context::*;

mod world_grid;
mod toolbox;
mod datatype;
mod context;

pub fn main() {
    let bg_color = Color::RGBA(255, 255, 255, 255);
    let point_size : usize = 2;
    

    let mut world = World::new(800/point_size, 600/point_size);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut painting = false;
    let mut counter = 0;
    let window = video_subsystem
        .window("sandbox", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_mouse_pos = Vector2::<usize>{x:0, y:0};
    let mut last_frame_times = VecDeque::<usize>::new();
    let mut ttf = sdl2::ttf::init().unwrap();
    let mut texture_creator = canvas.texture_creator();

    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    let mut context = Context::new(&mut canvas, world, point_size, &ttf, &texture_creator);

    'running: loop {
        let loop_start = std::time::Instant::now();
        context.canvas.set_draw_color(bg_color);
        context.canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion {
                    timestamp, window_id, which,
                    mousestate, x, y, xrel, yrel,
                } => {
                    last_mouse_pos = Vector2{x: x as usize, y: y as usize};
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    painting = true
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    painting = false
                },
                _ => {}
            }
        }
        
        // Drawing a rectangle on mouse
        context.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.canvas.draw_rect(
            Rect::new(
                last_mouse_pos.clone().x as i32 - (point_size * context.toolbox.mouse_box().x) as i32 / 2 ,
                last_mouse_pos.clone().y as i32 - (point_size * context.toolbox.mouse_box().y) as i32 / 2 ,
                (context.toolbox.mouse_box().x * point_size) as u32,
                (context.toolbox.mouse_box().y * point_size) as u32,
            )
        ).unwrap();

        // Painting
        if (painting) {
            mouse_tick(&mut context, last_mouse_pos);
        }

        // Processing the world
        let updated_cells = context.world.process_frame();
        context.draw_cells(&updated_cells);
        
        context.draw_toolbox();
        
        // FPS calculation
        let delta_t = Duration::from_micros(16666);
        // let delta_t = Duration::from_micros(500000);
        // let delta_t = Duration::from_micros(1);
        let after = std::time::Instant::now();
        let elapsed = after.duration_since(loop_start);
        last_frame_times.push_back( max(elapsed, delta_t).as_micros() as usize );
        if (last_frame_times.len() > 60) {
            last_frame_times.pop_front();
        }
        let time_sum : usize = last_frame_times.iter().sum();
        context.draw_text(
            format!("FPS: {}", (1000000.0 * last_frame_times.len() as f32 / (time_sum as f32)) as i32 ).as_str(),
            Vector2{x:550, y:0},
        );
        context.canvas.present();
        if (delta_t > elapsed) {
            std::thread::sleep( delta_t - elapsed );
        }
    }
}

fn mouse_tick(context: &mut Context, mouse_pos:Vector2<usize>) {
    let width = context.world.width().clone();
    let height = context.world.height().clone();
    let mut world = &mut context.world;
    let toolbox = &context.toolbox;
    let canvas = &context.canvas;

    let center_grid = Vector2::<usize> {
        x: mouse_pos.x / context.point_size,
        y: mouse_pos.y / context.point_size,
    };
    let mut center_x = max(toolbox.mouse_box().x/2, center_grid.x);
    let mut center_y = max(toolbox.mouse_box().y/2, center_grid.y);
    center_x = min(center_x, width - toolbox.mouse_box().x/2);
    center_y = min(center_y, height - toolbox.mouse_box().y/2);
    context.tick_counter += 1;
    for i in (0..toolbox.points_per_paint()) {
        let x = center_x - toolbox.mouse_box().x/2 + context.rng.gen_range( 0..toolbox.mouse_box().x );
        let y = center_y - toolbox.mouse_box().y/2 + context.rng.gen_range( 0..toolbox.mouse_box().y );
        world.set(x, y, CellType::Sand(ParticleData{
            speed: Vector2{x:0.0, y:2.0},
            color: Color::RGBA(
                if (context.tick_counter % 512 >= 256) { 
                    (context.tick_counter % 256) as u8
                } else {
                    255u8 - (context.tick_counter % 256) as u8
                },
                0,
                0,
                255
            )
        }));
    }
}

