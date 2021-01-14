use sdl2::event::Event;

use crate::app::App;

pub trait GameState {
    fn new(app: &mut App) -> Self;
    fn update(&mut self, app: &mut App);
    fn render(&mut self, app: &mut App);
    fn handle_input(&mut self, app: &mut App, event: &Event) -> bool; // true if handled/can be ignored
}
