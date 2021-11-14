use crate::enum_dispatch::*;
use crate::app::*;

mod debug_pieces;
mod main_menu;
mod persistent_data;
mod scene_manager;
mod single_local;
mod multiplayer;
mod multiplayer_spectate;

pub use debug_pieces::*;
pub use main_menu::*;
pub use persistent_data::*;
pub use scene_manager::*;
pub use single_local::*;
pub use multiplayer::*;
pub use multiplayer_spectate::*;

#[enum_dispatch]
pub trait SceneTrait {
    fn update(&mut self, _app: &mut App, _persistent: &mut PersistentData) {}
    fn render(&mut self, _app: &mut App, _persistent: &mut PersistentData) {}

    // @TODO change parameter order (event before app+persistent)
    fn handle_input(&mut self, _app: &mut App, _persistent: &mut PersistentData, _event: &sdl2::event::Event) -> bool { false }

    fn transition(&mut self, _app: &mut App, _persistent: &mut PersistentData) -> Option<SceneTransition> { None }

    fn on_enter(&mut self, _app: &mut App, _persistent: &mut PersistentData) {}
    fn on_exit (&mut self, _app: &mut App, _persistent: &mut PersistentData) {}
}

// @TODO remove capital from Player
// @TODO remove Scene suffix
#[enum_dispatch(SceneTrait)]
#[derive(Debug, ImDraw)]
pub enum Scene {
    MainMenuScene,
    SingleLocalScene,
    MultiPlayerScene,
    MultiPlayerSpectateScene,
    DebugPiecesScene,
}

pub enum SceneTransition {
    Pop,
    Push(Scene),
    Swap(Scene),
}
