use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use crate::datatype::*;
use crate::toolbox::*;
use crate::world_grid::*;


pub struct Context<'a> {
    pub canvas: Canvas<Window>,
    pub world: WorldGrid,
    pub toolbox: ToolBox,
    pub rng: ThreadRng,
    pub point_size: u32,
    pub tick_counter: u32,
    pub texture_creator: TextureCreator<WindowContext>,
    pub font: Font<'a, 'a>,
}

impl<'a> Context<'a> {

    pub fn draw_toolbox(&mut self) {
        let dy = 30;
        let mut y = 0;
        self.draw_text(format!("Box size: {}", self.toolbox.mouse_box()).as_str() , Vector2{x:0, y: y+dy});
        y+=dy;
        self.draw_text("Fdp!", Vector2{x:0, y:0})
    }

    pub fn draw_text(&mut self, str:&str, pos:Vector2<u32>) {
        let surface = self.font
            .render(&str)
            .blended(Color::RGB(0, 0, 0))
            .unwrap();

        let texture = self.texture_creator
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

    pub fn draw_cells(&mut self) {
        let &point_size = &self.point_size;
        // Drawing points for every cell
        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                match self.world.get(x, y) {
                    CellType::Empty => {},
                    CellType::Sand(data) => {
                        self.canvas.set_draw_color(data.color);
                        self.canvas.draw_point(Point::new(x as i32, y as i32).scale(point_size as i32)).unwrap();
                        self.canvas.fill_rect(Rect::new((x*point_size) as i32, (y*point_size) as i32, point_size, point_size)).unwrap();
                    },
                    CellType::Block(data) => {
                        self.canvas.set_draw_color(data.color);
                        self.canvas.fill_rect(Rect::new((x*point_size) as i32, (y*point_size) as i32, point_size, point_size)).unwrap();
                    },
                }
            }
        }
    }

}
