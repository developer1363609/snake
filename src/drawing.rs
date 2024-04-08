use piston_window::{Context, G2d, rectangle};
use piston_window::types::Color;

const BLOCK_SIZE:f64 = 25.0;

pub fn to_gui_coord(coord:i32) -> f64 {
    (coord as f64) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(coord:i32) -> u32{
    to_gui_coord_u32(coord)
}

pub fn draw_block(color:Color,x:i32,y:i32,context:&Context,g:&mut G2d) {
    let x = to_gui_coord(x);
    let y = to_gui_coord(y);
    rectangle(color,[x,y,BLOCK_SIZE,BLOCK_SIZE],context.transform,g)
}

pub fn draw_rectangle(color:Color,x:i32,y:i32,width:i32,height:i32,context:&Context,g:&mut G2d) {
    let start_x = to_gui_coord(x);
    let start_y = to_gui_coord(y);
    rectangle(color,[start_x,start_y,
              BLOCK_SIZE * (width as f64),BLOCK_SIZE * (height as f64)],context.transform,g);
}