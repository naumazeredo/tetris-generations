use crate::app::*;
use crate::State;

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

    pub fn update(&mut self, app: &mut App<'_, State>, persistent: &mut PersistentData) {
        self.try_transition();

        if let Some(scene) = self.scenes.last_mut() {
            scene.update(app, persistent);
        } else {
            panic!("[scene_manager update] empty scene being updated!");
        }
    }

    pub fn render(&mut self, app: &mut App<'_, State>, persistent: &mut PersistentData) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.render(app, persistent);
        } else {
            panic!("[scene_manager render] empty scene being updated!");
        }
    }

    pub fn handle_input(
        &mut self,
        app: &mut App<'_, State>,
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

    fn try_transition(&mut self) {
        let transition;

        if let Some(scene) = self.scenes.last_mut() {
            if let Some(t) = scene.transition() {
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
                    _ => {}
                }
            }

            SceneTransition::Push(scene) => self.scenes.push(scene),
            SceneTransition::Swap(scene) => {
                match self.scenes.pop() {
                    None => panic!("[scene_manager apply_transition pop] empty scene being popped!"),
                    _ => {}
                }

                self.scenes.push(scene);
            }
        }
    }

}
