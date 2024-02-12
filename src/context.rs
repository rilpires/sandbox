use std::cell;
use std::iter;
use std::rc;
use std::rc::Rc;

use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::TextureCreator;
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use crate::datatype::*;
use crate::toolbox::*;
use crate::world_grid::*;


pub struct Context<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub world: World,
    pub toolbox: ToolBox,
    pub rng: ThreadRng,
    pub point_size: usize,
    pub tick_counter: usize,
    pub font: Font<'a, 'a>,

    texture_creator: &'a TextureCreator<WindowContext>,
    cells_texture: Texture<'a>,
}

impl<'a> Context<'a> {

    pub fn new(
        canvas: &'a mut Canvas<Window>,
        world: World,
        point_size: usize,
        ttf: &'a Sdl2TtfContext,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let mut toolbox = ToolBox::new();
        let mut rng = rand::thread_rng();
        let world_size = Vector2 {
            x: world.width() as i32,
            y: world.height() as i32,
        };
        let font_bytes = include_bytes!("../assets/Courier Prime Code.ttf");
        let rwops = RWops::from_bytes(font_bytes).unwrap();
        let font = ttf.load_font_at_index_from_rwops(
            rwops, 0, 24,
        ).unwrap();
        
        let mut cells_texture = texture_creator.create_texture(
            PixelFormatEnum::RGBA32,
            TextureAccess::Streaming,
            world_size.x as u32,
            world_size.y as u32,
        ).unwrap();
        // set as invisible white
        cells_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for i in 0..buffer.len() {
                buffer[i] = 255;
            }
        }).unwrap();
        cells_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut ret = Context {
            canvas,
            world,
            toolbox,
            rng,
            point_size,
            tick_counter: 0,
            font,
            texture_creator: texture_creator,
            cells_texture: cells_texture,
        };

        return ret;
    }

    pub fn draw_toolbox(&mut self) {
        let dy = 30;
        let mut y = 0;
        self.draw_text(format!("Box size: {}", self.toolbox.mouse_box()).as_str() , Vector2{x:0, y: y+dy});
        y+=dy;
        self.draw_text("Fdp!", Vector2{x:0, y:0})
    }

    pub fn draw_text(&mut self, str:&str, pos:Vector2<usize>) {
        let surface = self.font
            .render(&str)
            .blended(Color::RGB(0, 0, 0))
            .unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        
        let texture_attrs = texture.query();

        &self.canvas.copy(
            &texture,
            None,
            Rect::new(
                pos.x as i32, pos.y as i32,
                texture_attrs.width, texture_attrs.height,
            )
        );
    }

    // Receives an iterator of vector2 to update
    pub fn draw_cells(&mut self, cells: &Vec<Vector2<usize>>) {
        let width = self.world.width().clone();
        let height = self.world.height().clone();
        self.cells_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

            // Drawing points for every cell
            for cell in cells.iter() {
                let x = cell.x;
                let y = cell.y;
                match self.world.get(x, y) {
                    CellType::Sand(data) => {
                        let i = (x + y*width) as usize * 4;
                        buffer[i] = data.color.r;
                        buffer[i+1] = data.color.g;
                        buffer[i+2] = data.color.b;
                        buffer[i+3] = data.color.a;
                    },
                    _ => {
                        let i = (x + y*width) as usize * 4;
                        buffer[i] = 255;
                        buffer[i+1] = 255;
                        buffer[i+2] = 255;
                        buffer[i+3] = 0;
                    }
                }
            }
        }).unwrap();
        self.canvas.copy(
            &self.cells_texture,
            None,
            None //Rect::new(0, 0, self.world.width() as u32, self.world.height() as u32),
        );
    }

}
