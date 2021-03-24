// Animation System

// [ ] make a module (animations/{animator, animation_container})
// [ ] rename to animation_container.rs
// [ ] rename to AnimationContainer

use super::{
    App,
    GameState,
    renderer::Sprite,
    imgui::ImDraw,
    task_system::Task,
};

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Animator {
    animation_set: AnimationSet,

    // @Refactor make this somehow safe with newtype idiom
    current_animation: usize,
    current_frame: usize,
    current_repetition: Repetitions,

    task: Option<Task>,
}

impl Animator {
    pub fn new(animation_set: AnimationSet) -> Self {
        Self {
            animation_set,
            current_animation: 0usize,
            current_frame: 0usize,
            current_repetition: Repetitions::Finite(0),
            task: None,
        }
    }

    // @Maybe animators shouldn't be allowed to change the animation set, only the animations
    pub fn change_animation_set<S: GameState>(&mut self, animation_set: AnimationSet, app: &mut App<S>) {
        self.animation_set = animation_set;
        self.current_animation = 0usize;
        self.current_frame = 0usize;
        self.current_repetition = Repetitions::Finite(0);

        if let Some(mut task) = self.task.take() {
            task.cancel(&mut app.task_system);
        }
    }

    pub fn next_frame<S: GameState>(&mut self, app: &App<S>) -> Option<()> {
        assert!(self.task.is_some());

        let (animation_data, _) = app.animation_system.get_animation_and_frame(self);

        if self.current_frame + 1 < animation_data.frames.len() {
            self.current_frame += 1;
        } else {
            if let Repetitions::Finite(total_repetitions) = animation_data.repetitions {
                if let Repetitions::Finite(repetition) = self.current_repetition {
                    if repetition == total_repetitions {
                        return None;
                    }

                    self.current_repetition = Repetitions::Finite(repetition + 1);
                }
            }

            self.current_frame = 0;
        }

        Some(())
    }

    pub fn get_current_sprite<S: GameState>(&self, app: &App<S>) -> Sprite {
        let (_, frame_data) = app.animation_system.get_animation_and_frame(self);
        frame_data.sprite
    }

    pub fn is_playing(&self) -> bool {
        self.task.is_some()
    }

    pub fn stop<S: GameState>(&mut self, app: &mut App<S>) {
        let mut task = self.task.take().unwrap();
        task.cancel(&mut app.task_system);
    }

    pub fn play<'a, S: GameState, F>(&mut self, app: &mut App<'a, S>, callback: F)
    where F: FnMut(u64, &mut S, &mut App<S>) + 'a,
    {
        let (_, frame_data) = app.animation_system.get_animation_and_frame(self);
        let duration = frame_data.duration;
        let task = app.schedule_task(duration, callback);
        self.task.replace(task).expect_none("[animation] trying to play while already playing");
    }
}

#[derive(Default)]
pub struct AnimationSystem {
    pub(super) animation_sets: Vec<AnimationSetData>,
    pub(super) animations: Vec<AnimationData>,
    pub(super) frames: Vec<FrameData>,
}

impl AnimationSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_animation_and_frame<'a>(
        &'a self,
        animator: &Animator
    ) -> (&'a AnimationData, &'a FrameData) {
        let animation_set_data = &self.animation_sets[animator.animation_set.0 as usize];

        let animation = animation_set_data.animations[animator.current_animation];
        let animation_data = &self.animations[animation.0 as usize];

        let frame = animation_data.frames[animator.current_frame];
        let frame_data = &self.frames[frame.0 as usize];

        (animation_data, frame_data)
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct AnimationSet(u64);
pub struct AnimationSetData {
    pub(super) id: AnimationSet,
    pub animations: Vec<Animation>,
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Animation(u64);
pub struct AnimationData {
    pub(super) id: Animation,
    pub repetitions: Repetitions,
    pub frames: Vec<Frame>,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame(u64);
pub struct FrameData {
    pub(super) id: Frame,
    pub sprite: Sprite,
    pub duration: u64, // @Refactor create type-safe time/duration struct
}

#[derive(Copy, Clone, Debug)]
pub enum Repetitions {
    Infinite,
    Finite(u32),
}

impl ImDraw for Repetitions {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        ui.text(format!("{}: (todo)", label));
    }
}

impl<S> App<'_, S> {
    pub fn build_frame(&mut self, sprite: Sprite, duration: u64) -> Frame {
        let frames = &mut self.animation_system.frames;

        let id = frames.len() as u64;
        let frame = Frame(id);

        frames.push(FrameData {
            id: frame,
            sprite,
            duration
        });
        frame
    }

    pub fn build_animation(&mut self, frames: Vec<Frame>, repetitions: Repetitions) -> Animation {
        let animations = &mut self.animation_system.animations;

        let id = animations.len() as u64;
        let animation = Animation(id);

        animations.push(AnimationData {
            id: animation,
            frames,
            repetitions
        });
        animation
    }

    pub fn build_animation_set(&mut self, animations: Vec<Animation>) -> AnimationSet {
        let sets = &mut self.animation_system.animation_sets;

        let id = sets.len() as u64;
        let set = AnimationSet(id);

        sets.push(AnimationSetData {
            id: set,
            animations
        });
        set
    }
}
