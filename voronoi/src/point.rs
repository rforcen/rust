/*
	x,y point
*/

#[derive(Debug)]
pub struct Point {
	pub x		:u32,
	pub y		:u32,
	pub color	:u32,
}

impl Point {
	pub fn new(x:u32, y:u32, color:u32) -> Self { Point{x, y, color} }
}