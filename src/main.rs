#![allow(clippy::all, warnings, unused)]

use datatype::Vector2;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use sdl2::libc::{abs, rand};
use sdl2::pixels::Color;
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
use sdl2::render::{Canvas, TextureCreator};
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
    let font_bytes = include_bytes!("../assets/Courier Prime Code.ttf");
    let bg_color = Color::RGB(255, 255, 255);
    let point_size : u32 = 1;
    
    let mut toolbox = ToolBox::new();
    let mut world = WorldGrid::new(800/point_size, 600/point_size);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut painting = false;
    let mut counter = 0;
    let mut ttf = sdl2::ttf::init().unwrap();
    let window = video_subsystem
        .window("sandbox", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut rng = rand::thread_rng();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_mouse_pos = Vector2::<u32>{x:0, y:0};
    let rwops = RWops::from_bytes(font_bytes).unwrap();
    let mut last_frame_times = VecDeque::<u32>::new();

    let mut context = Context {
        canvas: canvas,
        world,
        toolbox,
        rng,
        point_size,
        tick_counter: 0,
        texture_creator,
        font: ttf.load_font_at_index_from_rwops(
            rwops, 0, 24,
        ).unwrap(),
    };



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
                    last_mouse_pos = Vector2{x: x as u32, y: y as u32};
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
                context.toolbox.mouse_box().x * point_size,
                context.toolbox.mouse_box().y * point_size,
            )
        ).unwrap();

        // Processing the world
        context.world.process_frame();

        // Painting
        if (painting) {
            mouse_tick(&mut context, last_mouse_pos);
        }

        context.draw_cells();
        context.draw_toolbox();

        // FPS calculation
        let delta_t = Duration::from_micros(16666);
        let after = std::time::Instant::now();
        let elapsed = after.duration_since(loop_start);
        last_frame_times.push_back( max(elapsed, delta_t).as_micros() as u32 );
        if (last_frame_times.len() > 60) {
            last_frame_times.pop_front();
        }
        let time_sum : u32 = last_frame_times.iter().sum();
        context.draw_text(
            format!("FPS: {}", (1000000.0 * last_frame_times.len() as f32 / (time_sum as f32)) as i32 ).as_str(),
            Vector2{x:550u32, y:0u32},
        );


        context.canvas.present();
        if (delta_t > elapsed) {
            std::thread::sleep( delta_t - elapsed );
        }
    }
}

fn mouse_tick(context: &mut Context, mouse_pos:Vector2<u32>) {
    let world = &context.world;
    let toolbox = &context.toolbox;
    let canvas = &context.canvas;

    let center_grid = Vector2::<u32> {
        x: mouse_pos.x / context.point_size,
        y: mouse_pos.y / context.point_size,
    };
    let mut center_x : u32 = max(toolbox.mouse_box().x/2, center_grid.x as u32);
    let mut center_y : u32 = max(toolbox.mouse_box().y/2, center_grid.y as u32);
    center_x = min(center_x, world.width() - toolbox.mouse_box().x/2);
    center_y = min(center_y, world.height() - toolbox.mouse_box().y/2);
    for i in (0..toolbox.points_per_paint()) {
        if (i%10==0) {context.tick_counter = (context.tick_counter + 1)%254};
        let x = center_x - toolbox.mouse_box().x/2 + context.rng.gen_range( 0..toolbox.mouse_box().x );
        let y = center_y - toolbox.mouse_box().y/2 + context.rng.gen_range( 0..toolbox.mouse_box().y );
        context.world.set(x, y, CellType::Sand(ParticleData{
            speed: (0.0, 1.0),
            color: Color::RGB(context.tick_counter as u8, 0, 0)
        }));
    }
}

