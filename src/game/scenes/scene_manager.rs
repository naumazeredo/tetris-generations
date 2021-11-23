use crate::app::*;

use super::{
    SceneTrait,
    SceneTransition,
};

#[derive(Debug, ImDraw)]
pub struct SceneManager<S: SceneTrait> {
    scenes: Vec<S>,
}

impl<S: SceneTrait<Scene = S>> SceneManager<S> {
    pub fn new(main_scene: S) -> Self {
        Self {
            scenes: vec![main_scene],
        }
    }

    pub fn update(&mut self, app: &mut App, persistent: &mut <S as SceneTrait>::PersistentData) {
        self.try_transition(app, persistent);

        if let Some(scene) = self.scenes.last_mut() {
            scene.update(app, persistent);
        } else {
            panic!("[scene_manager update] empty scene being updated!");
        }
    }

    pub fn render(&mut self, app: &mut App, persistent: &mut <S as SceneTrait>::PersistentData) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.render(app, persistent);
        } else {
            panic!("[scene_manager render] empty scene being updated!");
        }
    }

    pub fn handle_input(
        &mut self,
        event: &sdl2::event::Event,
        app: &mut App,
        persistent: &mut <S as SceneTrait>::PersistentData,
    ) -> bool {
        if let Some(scene) = self.scenes.last_mut() {
            scene.handle_input(event, app, persistent)
        } else {
            panic!("[scene_manager render] empty scene being updated!");
        }
    }

    pub fn current_scene(&mut self) -> &mut S {
        if let Some(scene) = self.scenes.last_mut() {
            scene
        } else {
            panic!("[scene_manager current_scene] empty scene being get!");
        }
    }

    fn try_transition(&mut self, app: &mut App, persistent: &mut <S as SceneTrait>::PersistentData) {
        let transition;

        if let Some(scene) = self.scenes.last_mut() {
            if let Some(t) = scene.transition(app, persistent) {
                transition = t;
            } else {
                return;
            }
        } else {
            panic!("[scene_manager try_transition] empty scene being get!");
        }

        match transition {
            SceneTransition::Pop => {
                match self.scenes.pop() {
                    None => panic!("[scene_manager apply_transition pop] empty scene being popped!"),
                    Some(mut s) => s.on_exit(app, persistent),
                }
            }

            SceneTransition::Push(mut scene) => {
                scene.on_enter(app, persistent);
                self.scenes.push(scene);
            }

            SceneTransition::Swap(mut scene) => {
                match self.scenes.pop() {
                    None => panic!("[scene_manager apply_transition pop] empty scene being popped!"),
                    Some(mut s) => s.on_exit(app, persistent),
                }

                scene.on_enter(app, persistent);
                self.scenes.push(scene);
            }
        }
    }

}
