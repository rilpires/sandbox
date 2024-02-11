use sdl2::pixels::Color;
use sdl2::event::Event;
use crate::datatype::*;


pub struct ToolBox {
    current_color_index: u32,
    available_colors: Vec<Color>,
    mouse_box: Vector2<u32>,
    points_per_paint : u32,
}

impl ToolBox {
    pub fn new() -> ToolBox {
        ToolBox {
            current_color_index: 0,
            available_colors: vec![
                Color::RGB(255, 255, 255),
                Color::RGB(0, 0, 0),
                Color::RGB(255, 0, 0),
                Color::RGB(0, 255, 0),
                Color::RGB(0, 0, 255),
            ],
            mouse_box: Vector2{x:15u32, y:15u32},
            points_per_paint: 25,
        }
    }

    pub fn mouse_box(&self) -> Vector2<u32> {
        self.mouse_box
    }

    pub fn points_per_paint(&self) -> u32 {
        self.points_per_paint
    }

    pub fn current_color_index(&self) -> u32 {
        self.current_color_index
    }

    pub fn available_colors(&self) -> &Vec<Color> {
        &self.available_colors
    }

    pub fn get_current_color(&self) -> &Color {
        &self.available_colors[self.current_color_index as usize]
    }

}