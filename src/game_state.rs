use crate::app::App;

pub trait GameState {
    fn new() -> Self;
    fn update(&mut self, app: &mut App);
    fn render(&mut self, app: &mut App);

    // true if handled/can be ignored
    //fn handle_input(&mut self, app: &mut App, event: sdl2::event::Event) -> bool;
}
