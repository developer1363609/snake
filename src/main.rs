use piston_window::types::Color;
use piston_window::{Button, clear, PistonWindow, PressEvent, UpdateEvent, WindowSettings};
use crate::drawing::to_gui_coord_u32;
use crate::game::Game;

mod drawing;
mod snake;
mod game;

const BLACK_COLOR:Color = [0.204, 0.286, 0.369, 1.0];
fn main() {
    let (width,height) = (20,20);
    let mut window_settings = WindowSettings::new("rust snake",
    [to_gui_coord_u32(width),to_gui_coord_u32(height)]).exit_on_esc(true);
    window_settings.set_vsync(true);
    let mut window:PistonWindow = window_settings.build().unwrap();
    let mut game = Game::new(width,height);
    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }
        window.draw_2d(&event,|c,g,_|{
            clear(BLACK_COLOR,g);
            game.draw(&c,g);
        });
        event.update(|arg|{
            game.update(arg.dt);
        })
    }
}
