use crate::enum_dispatch::*;
use crate::app::*;
use crate::State;

mod debug_pieces;
mod persistent_data;
mod scene_manager;
mod singleplayer;

pub use debug_pieces::*;
pub use persistent_data::*;
pub use scene_manager::*;
pub use singleplayer::*;

#[enum_dispatch]
pub trait SceneTrait {
    fn update(
        &mut self,
        _app: &mut App<'_, State>,
        _persistent: &mut PersistentData
    ) {}

    fn render(
        &mut self,
        _app: &mut App<'_, State>,
        _persistent: &mut PersistentData
    ) {}

    fn handle_input(
        &mut self,
        _app: &mut App<'_, State>,
        _persistent: &mut PersistentData,
        _event: &sdl2::event::Event
    ) -> bool {
        false
    }

    fn transition(&mut self) -> Option<SceneTransition> {
        None
    }

    //fn on_enter(&mut self, ...);
    //fn on_exit(&mut self, ...);
}

#[enum_dispatch(SceneTrait)]
#[derive(Clone, Debug)]
pub enum Scene {
    SinglePlayerScene,
    DebugPiecesScene,
}

impl_imdraw_todo!(Scene);

pub enum SceneTransition {
    Pop,
    Push(Scene),
    Swap(Scene),
}
