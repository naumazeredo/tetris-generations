use sdl2::event::Event;

use crate::app::App;

pub trait GameState {
    fn new(app: &mut App<'_, Self>) -> Self where Self: Sized;
    fn update(&mut self, app: &mut App<'_, Self>) where Self: Sized;
    fn render(&mut self, app: &mut App<'_, Self>) where Self: Sized;
    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &Event) -> bool where Self: Sized; // true if handled/can be ignored
}
