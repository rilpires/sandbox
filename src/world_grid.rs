// This is world_grid module
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::cmp::*;
use std::u32;


#[derive(Clone, PartialEq)]
pub struct ParticleData {
    pub speed: (f32, f32),
    pub color: Color,
}

#[derive(Clone)]
pub struct WorldGrid {
    world_rng : ThreadRng,
    width: u32,
    height: u32,
    grid: Vec<CellType>
}


#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Sand(ParticleData),
    Block(ParticleData),
}

impl WorldGrid {
    pub fn new(width: u32, height: u32) -> WorldGrid {
        let v = vec![CellType::Empty; (width * height) as usize];
        WorldGrid {
            world_rng: rand::thread_rng(),
            width,
            height,
            grid: v.clone()
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> CellType {
        self.grid[(y * self.width + x) as usize].clone()
    }

    pub fn set(&mut self, x: u32, y: u32, cell_type: CellType) {
        self.grid[(y * self.width + x) as usize] = cell_type;
    }

    pub fn process_frame(&mut self) {
        let mut tmp_grid = self.grid.clone();
        let mut xvec : Vec<u32> = (0..self.width).collect();
        let mut yvec : Vec<u32> = (0..self.height).collect();
        xvec.shuffle(&mut self.world_rng);
        // yvec.reverse();
        yvec.shuffle(&mut self.world_rng);
        for x in xvec {
            for &y in &yvec {
                match &tmp_grid[(x+y*self.width) as usize] {
                    CellType::Empty => {},
                    CellType::Sand(data) => {
                        let mut dy = f32::floor(data.speed.1) as u32;
                        while dy >= 1 {
                            if y < self.height - dy {
                                if self.get(x, y + dy ) == CellType::Empty {
                                    let mut new_data = data.clone();
                                    new_data.speed.1 += 0.2;
                                    self.set(x, y, CellType::Empty);
                                    self.set(x, y + dy, CellType::Sand(new_data));
                                    break;
                                } else {
                                    let mut new_data = data.clone();
                                    new_data.speed.1 = 1.0;
                                    let mut fall_right = (x < self.width-1) && (self.get(x + 1, y + 1) == CellType::Empty);
                                    let mut fall_left = (x > 0) && (self.get(x - 1, y + 1) == CellType::Empty);  
                                    if (fall_left && fall_right) {
                                        fall_right = self.world_rng.gen_range(-1..=1) > 0;
                                        fall_left = !fall_right;
                                    }
                                    if fall_right {
                                        self.set(x, y, CellType::Empty);
                                        self.set(x + 1, y + 1, CellType::Sand(new_data));
                                    } else if fall_left  {
                                        self.set(x, y, CellType::Empty);
                                        self.set(x - 1, y + 1, CellType::Sand(new_data));
                                    }
                                    break;
                                }
                            }
                            dy -= 1;
                        }
                    },
                    CellType::Block(_) => {},
                }
            
            }
        }
    }

}
