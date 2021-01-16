use sdl2::event::Event;

use crate::app::App;

pub trait GameState {
    fn new(app: &mut App<'_, Self>) -> Self;
    fn update(&mut self, app: &mut App<'_, Self>);
    fn render(&mut self, app: &mut App<'_, Self>);
    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &Event) -> bool; // true if handled/can be ignored
}
