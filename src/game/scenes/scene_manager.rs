use crate::app::*;

use super::{
    Scene,
    PersistentData,
    SceneTrait,
    SceneTransition,
};

#[derive(Clone, Debug, ImDraw)]
pub struct SceneManager {
    scenes: Vec<Scene>,
}

impl SceneManager {
    pub fn new(main_scene: Scene) -> Self {
        Self {
            scenes: vec![main_scene],
        }
    }

    pub fn update(&mut self, app: &mut App, persistent: &mut PersistentData) {
        self.try_transition(app, persistent);

        if let Some(scene) = self.scenes.last_mut() {
            scene.update(app, persistent);
        } else {
            panic!("[scene_manager update] empty scene being updated!");
        }
    }

    pub fn render(&mut self, app: &mut App, persistent: &mut PersistentData) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.render(app, persistent);
        } else {
            panic!("[scene_manager render] empty scene being updated!");
        }
    }

    pub fn handle_input(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        if let Some(scene) = self.scenes.last_mut() {
            scene.handle_input(app, persistent, event)
        } else {
            panic!("[scene_manager render] empty scene being updated!");
        }
    }

    pub fn current_scene(&mut self) -> &mut Scene {
        if let Some(scene) = self.scenes.last_mut() {
            scene
        } else {
            panic!("[scene_manager current_scene] empty scene being get!");
        }
    }

    fn try_transition(&mut self, app: &mut App, persistent: &mut PersistentData) {
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
