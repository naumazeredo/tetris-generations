use crate::app::*;

mod debug_pieces;
mod main_menu;
mod persistent_data;
mod scene_manager;
mod singleplayer;
//mod multiplayer;
//mod multiplayer_spectate;

pub use debug_pieces::*;
pub use main_menu::*;
pub use persistent_data::*;
pub use scene_manager::*;
pub use singleplayer::*;

/*
pub use multiplayer::*;
pub use multiplayer_spectate::*;
*/

pub trait SceneTrait: ImDraw {
    type Scene: SceneTrait;
    type PersistentData;

    fn on_enter(&mut self, _app: &mut App, _persistent: &mut Self::PersistentData) {}
    fn on_exit (&mut self, _app: &mut App, _persistent: &mut Self::PersistentData) {}

    fn update(&mut self, _app: &mut App, _persistent: &mut Self::PersistentData) {}
    fn render(&mut self, _app: &mut App, _persistent: &mut Self::PersistentData) {}

    fn handle_input(
        &mut self,
        _event: &sdl2::event::Event,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) -> bool
    { false }

    fn transition(
        &mut self,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>>
    { None }
}

#[derive(Debug, ImDraw)]
pub enum Scene {
    MainMenuScene(MainMenuScene),
    SingleplayerScene(SingleplayerScene),
    //MultiPlayerScene,
    //MultiPlayerSpectateScene,
    DebugPiecesScene(DebugPiecesScene),
}

// @TODO proc_macro for this
impl SceneTrait for Scene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    )
    {
        match self {
            Self::MainMenuScene(scene)     => scene.update(app, persistent),
            Self::SingleplayerScene(scene) => scene.update(app, persistent),
            Self::DebugPiecesScene(scene)  => scene.update(app, persistent),
        }
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    )
    {
        match self {
            Self::MainMenuScene(scene)     => scene.render(app, persistent),
            Self::SingleplayerScene(scene) => scene.render(app, persistent),
            Self::DebugPiecesScene(scene)  => scene.render(app, persistent),
        }
    }

    fn handle_input(
        &mut self,
        event: &sdl2::event::Event,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) -> bool
    {
        match self {
            Self::MainMenuScene(scene)     => scene.handle_input(event, app, persistent),
            Self::SingleplayerScene(scene) => scene.handle_input(event, app, persistent),
            Self::DebugPiecesScene(scene)  => scene.handle_input(event, app, persistent),
        }
    }

    fn transition(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>>
    {
        match self {
            Self::MainMenuScene(scene)     => scene.transition(app, persistent),
            Self::SingleplayerScene(scene) => scene.transition(app, persistent),
            Self::DebugPiecesScene(scene)  => scene.transition(app, persistent),
        }
    }

    fn on_enter(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    )
    {
        match self {
            Self::MainMenuScene(scene)     => scene.on_enter(app, persistent),
            Self::SingleplayerScene(scene) => scene.on_enter(app, persistent),
            Self::DebugPiecesScene(scene)  => scene.on_enter(app, persistent),
        }
    }

    fn on_exit(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    )
    {
        match self {
            Self::MainMenuScene(scene)     => scene.on_exit(app, persistent),
            Self::SingleplayerScene(scene) => scene.on_exit(app, persistent),
            Self::DebugPiecesScene(scene)  => scene.on_exit(app, persistent),
        }
    }
}

impl From<MainMenuScene> for Scene {
    fn from(other: MainMenuScene) -> Self {
        Self::MainMenuScene(other)
    }
}

impl From<SingleplayerScene> for Scene {
    fn from(other: SingleplayerScene) -> Self {
        Self::SingleplayerScene(other)
    }
}

impl From<DebugPiecesScene> for Scene {
    fn from(other: DebugPiecesScene) -> Self {
        Self::DebugPiecesScene(other)
    }
}

pub enum SceneTransition<S> {
    Pop,
    Push(S),
    Swap(S),
}
