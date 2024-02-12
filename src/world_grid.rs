// This is world_grid module
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::cell::{Cell, RefCell};
use std::cmp::*;
use std::ops::Div;
use std::os::unix::thread;
use std::usize;
use std::collections::HashSet;

use crate::datatype::{GridMap, Vector2};


#[derive(Clone, PartialEq)]
pub struct ParticleData {
    pub speed: Vector2<f32>,
    pub color: Color,
}

#[derive(Clone)]
pub struct World {
    world_rng : ThreadRng,
    grid_rooms_hotness: GridMap<usize>,
    room_size: Vector2<usize>,
    grid: GridMap<CellType>,
}


#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Sand(ParticleData),
    Block(ParticleData),
}


impl World {
    
    // static single thread rng


    pub fn new(width: usize, height: usize) -> World {
        let original_grid = GridMap::new(width, height, CellType::Empty);
        let mut ret = World {
            world_rng: rand::thread_rng(),
            grid: original_grid.clone(),
            grid_rooms_hotness: GridMap::new(16, 16, 0),
            room_size: Vector2 { x: 0, y: 0 },
        };
        ret.room_size = Vector2 {
            x: ret.grid.width().div_ceil(ret.grid_rooms_hotness.width()),
            y: ret.grid.height().div_ceil(ret.grid_rooms_hotness.height()),
        };
        return ret;
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn get(&self, x:usize, y:usize) -> &CellType {
        self.grid.get(x, y)
    }

    pub fn get_mut(&mut self, x:usize, y:usize) -> &mut CellType {
        self.grid.get_mut(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, cell_type: CellType ) {
        self.grid.set(x, y, cell_type.clone());
        self.hot(x, y);
    }

    fn hot(&mut self, x: usize, y: usize) {
        self.grid_rooms_hotness.set_neighbor(
            (x/self.room_size.x) as i32,
            (y/self.room_size.y) as i32,
            12,
        );
    }

    pub fn process_frame(&mut self) -> Vec<Vector2<usize>> {

        let mut ret = vec![];
        
        self.grid_rooms_hotness.iter_mut().for_each(|x| {
            if *x > 0 { 
                *x = *x-1;
                // println!("Chilling...");
                // if (*x==0) {
                //     println!("Room hotness becomes 0");
                // }
            }
        });


        for room_x in (0..self.grid_rooms_hotness.width()) {
            for room_y in (0..self.grid_rooms_hotness.height()) {
                if (*self.grid_rooms_hotness.get(room_x, room_y) <= 0 ) {
                    continue;
                } else {
                    ret.extend( self.process_room(room_x, room_y ) );
                }
            }   
        }

        for cell in ret.iter() {
            self.hot(cell.x, cell.y);
        }

        return ret;
    }

    //fn process_room(&mut self, tmp_grid:&mut GridMap<CellType>, room_x:usize, room_y:usize) -> Vec<Vector2<usize>>  {
    fn process_room(&mut self, room_x:usize, room_y:usize) -> Vec<Vector2<usize>>  {
        
        let mut ret = vec![];
        
        // Cloning cheap values
        let room_size = self.room_size.clone();
        let height = self.height();
        let width = self.width();

        let mut xvec : Vec<usize> = (
            (room_x*room_size.x)..min(width, (room_x+1)*room_size.x)
        ).collect();
        let mut yvec : Vec<usize> = (
            (room_y*room_size.y)..min(height-1, (room_y+1)*room_size.y)
        ).collect();
        
        let mut rng = rand::thread_rng();
        xvec.shuffle(&mut rng);
        yvec.shuffle(&mut rng);
        let mut dirty_cells = HashSet::<Vector2<usize>>::new();

        for x in xvec.iter() {
            for y in yvec.iter() {
                let x = *x;
                let y = *y;
                if dirty_cells.contains(&Vector2{x: x, y: y}) {
                    continue;
                }
                match self.get(x, y) {
                    CellType::Empty => {},
                    CellType::Sand(data) => {

                        let mut new_pos : Option<Vector2<usize>> = None;
                        let mut new_data = Option::<ParticleData>::None;
                        let data = ParticleData{
                            speed: Vector2 {
                                x: data.speed.x,
                                y: data.speed.y + 0.15,
                            },
                            color: data.color,
                        };
                        let min_dy = if rng.gen_bool(0.5) {
                            0
                        } else {
                            1
                        };
                        let max_dy = if rng.gen_bool(0.5) {
                            data.speed.y.floor() as usize
                        } else {
                            data.speed.y.ceil() as usize
                        };
                        

                        // Finding the next cell below we can go
                        for dy in (min_dy..=max_dy) {
                            // full speed and still empty? nice
                            if (dy == max_dy) && (y+dy<height) && (*self.get(x, y + dy ) == CellType::Empty) {
                                new_pos = Some(Vector2{x: x, y: y+dy});
                                new_data = Some(data.clone());
                                break;
                            } 
                            // hit something, lets check before
                            else if (y + dy >= height) || (*self.get(x, y + dy ) != CellType::Empty) {
                                // there was a before?
                                if (dy>1) {
                                    new_pos = Some(Vector2{x: x.clone(), y: y.clone()+dy-1});
                                    new_data = Some(ParticleData{
                                        speed: Vector2{
                                            x:data.speed.x,
                                            y:data.speed.y*0.1
                                        },
                                        color: data.color
                                    });
                                    break;
                                } 
                                // ugh, maybe we slide
                                break
                            }
                        }


                        // aggressive slide
                        if new_pos.is_none() && y > 0 && y < height-1 {
                            let rand_dx = rng.gen_range(2..=4);
                            if (x > rand_dx) && (x < width-rand_dx) && *self.get(x,y+1) != CellType::Empty  && *self.get(x,y-1) != CellType::Empty {
                                let mut fall_right =
                                    (*self.get(x + rand_dx, y+1) == CellType::Empty) && 
                                    (*self.get(x - rand_dx, y+1) != CellType::Empty);
                                let mut fall_left =
                                    (*self.get(x - rand_dx, y+1) == CellType::Empty) && 
                                    (*self.get(x + rand_dx, y+1) != CellType::Empty);

                                if fall_right {
                                    new_pos = Some(Vector2{x:x+rand_dx, y: y+1});
                                    new_data = Some(data.clone());
                                    new_data.as_mut().unwrap().speed.y = 1.0;
                                } else if fall_left  {
                                    new_pos = Some(Vector2{x:x-rand_dx, y: y+1});
                                    new_data = Some(data.clone());
                                    new_data.as_mut().unwrap().speed.y = 1.0;
                                }
                            }
                        }

                        // simple slide
                        if new_pos.is_none() && (y + 1 < height) {
                            let mut fall_right =
                                (x < width-1) &&
                                (*self.get(x + 1, y + 1) == CellType::Empty);
                            let mut fall_left =
                                (x > 0) &&
                                (*self.get(x - 1, y + 1) == CellType::Empty);
                            if (fall_left && fall_right) {
                                fall_right = rng.gen_bool(0.5);
                                fall_left = !fall_right;
                            }
                            if fall_right {
                                new_pos = Some(Vector2{x:x+1, y:y+1});
                                new_data = Some(data.clone());
                            } else if fall_left  {
                                new_pos = Some(Vector2{x:x-1, y:y+1});
                                new_data = Some(data.clone());
                            }
                        }

                        
                        match new_pos {
                            None => {},
                            Some(new_pos) => {
                                if dirty_cells.contains(&new_pos) {
                                    continue;
                                } else {
                                    self.grid.set(x, y, CellType::Empty);
                                    self.grid.set(new_pos.x, new_pos.y, CellType::Sand(new_data.unwrap()));
                                    ret.push(Vector2{x: x, y: y});
                                    ret.push(new_pos);
                                    dirty_cells.insert(new_pos);
                                }
                            }
                        }
                    },
                    CellType::Block(_) => {},
                }
            
            }
        }

        return ret;
    }

}
