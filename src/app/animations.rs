// Animation System

// [ ] rename to animation_system.rs
// [ ] refactor to use entity id (from entity_container)

use super::{
    App,
    renderer::Sprite,
    //task_system::Task,
};

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
        animation_set: AnimationSet,
        current_animation: usize,
        current_frame: usize
    ) -> (&'a AnimationData, &'a FrameData) {
        let animation_set_data = &self.animation_sets[animation_set.0 as usize];

        let animation = animation_set_data.animations[current_animation];
        let animation_data = &self.animations[animation.0 as usize];

        let frame = animation_data.frames[current_frame];
        let frame_data = &self.frames[frame.0 as usize];

        (animation_data, frame_data)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AnimationSet(u64);
pub struct AnimationSetData {
    pub(super) id: AnimationSet,
    pub animations: Vec<Animation>,
}

#[derive(Copy, Clone, Debug)]
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

    /*
    pub fn build_animator(
        &mut self,
        animation_set: AnimationSet,
    ) -> Animator {

        let system = &mut self.animation_system;

        let id = system.animators_next_id;
        system.animators_next_id += 1;

        let animator = Animator(id);

        system.animators_data.insert(animator, AnimatorData {
            id: animator,
            animation_set,

            current_animation: 0usize,
            current_frame: 0usize,
            current_repetition: Repetitions::Finite(0),

            is_playing: false,

            task: Task::empty(),
        });

        animator
    }
    */
}

// -----
// utils
// -----

fn get_data<'a>(
    animation_sets: &'a mut Vec<AnimationSetData>,
    animations: &'a mut Vec<AnimationData>,
    frames: &'a mut Vec<FrameData>,
    animation_set: AnimationSet,
    current_animation: usize,
    current_frame: usize
) -> (&'a AnimationData, &'a FrameData) {
    let animation_set_data = &animation_sets[animation_set.0 as usize];

    let animation = animation_set_data.animations[current_animation];
    let animation_data = &animations[animation.0 as usize];

    let frame = animation_data.frames[current_frame];
    let frame_data = &frames[frame.0 as usize];

    (animation_data, frame_data)
}
