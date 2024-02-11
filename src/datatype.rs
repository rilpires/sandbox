use std::{fmt::Display, ops::{Mul, MulAssign}, process::Output};


#[derive(Clone, Copy)]
pub struct Vector2<T> where T: Mul + MulAssign + Copy + Display {
    pub x: T,
    pub y: T,
}


impl<T> Vector2<T> where T: Mul + MulAssign + Copy + Display {

    // Returns self
    pub fn scale<T2>(&mut self, s: T2) -> &mut Self
    where 
        T: Mul<T2, Output = T> + MulAssign<T2> + Copy,
        T2: Copy
    {
        self.x *= s; self.y *= s;
        self
    }

    // Returns as a tuple of any type
    pub fn convert<T2>(&self) -> Vector2<T2>
    where T2: Mul + MulAssign + From<T> + Copy + Display
    {
        return Vector2 {
            x: T2::from(self.x),
            y: T2::from(self.y),
        };
    }
}

impl<T> Display for Vector2<T> where T : Mul + MulAssign + Copy + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Clone)]
pub struct GridMap<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T> GridMap<T> 
where T: Clone
{
    pub fn new(w:usize, h:usize, default:T) -> Self {
        GridMap {
            width: w as usize,
            height: h as usize,
            grid: vec![default; w*h]
        }
    }

    pub fn get(&self, x:usize, y:usize) -> &T {
        return &(self.grid[x + y*self.width]);
    }

    pub fn set(&mut self, x:usize, y:usize, new_val:T) {
        self.grid[x + y*self.width] = new_val;
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

}
