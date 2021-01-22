// Animation System

// @AfterGame each entity should have its own Animation/Animator data structures to be able to
//            iterate over visible entities+animators in a cache-friendly way.

use std::collections::HashMap;
use super::{
    App,
    entity::transform::Transform,
    imgui::ImDraw,
    game_state::GameState,
    renderer::{
        Sprite,
        color,
        types::*,
    },
    tasks::{Task, schedule_task},
};

// @Refactor type-safe all ids
// @Refactor think how to better structure the relationship between these structs

// @Idea/@Refactor move animator to outside of AnimationSystem and add AnimatorData with Rc<RefCell<>>
//                 Better yet: Animator be only the id (with Rc<RefCell<u64>>), like Task should be,
//                 that is the only interface outside the AnimationSystem. AnimatorData should be
//                 the one stored efficiently

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ImDraw)]
pub struct Animator(u64);

// @Maybe it would be better to store a Weak ptr to the animator system. With this we could add
//        Drop to remove the Animator from the AnimationSystem automatically
impl Animator {
    // @TODO return result
    #[allow(dead_code)]
    pub fn play<S: GameState>(self, app: &mut App<S>) {
        let mut animator_data = app.animation_system.animators_data.get_mut(&self).expect("unknown animator");

        let index = match animator_data.list_index {
            AnimatorListIndex::Invisible(i) => i,
            _ => panic!("animator is already playing when play was called!"),
        };

        let render_data = app.animation_system.invisible_animators[index];

        // add to visible animators
        let index = app.animation_system.visible_animators.len();
        app.animation_system.visible_animators.push(render_data);
        animator_data.list_index = AnimatorListIndex::Visible(index);

        // remove from invisible animators
        let last = app.animation_system.invisible_animators.pop().expect("invisible animators empty");

        if last.id != self {
            app.animation_system.invisible_animators[index] = last;
            let mut moved_data = app.animation_system.animators_data.get_mut(&last.id)
                .expect("corrupted animator data");

            moved_data.list_index = AnimatorListIndex::Invisible(index);
        }

        // schedule animation task
        self.schedule_play_frame(app);
    }

    // @Refactor play and stop seems to be basically the same code. Maybe we can just use a single
    //           internal function
    // @TODO return result
    #[allow(dead_code)]
    pub fn stop<S>(self, app: &mut App<S>) {
        let mut animator_data = app.animation_system.animators_data
            .get_mut(&self)
            .expect("unknown animator");

        let index = match animator_data.list_index {
            AnimatorListIndex::Visible(i) => i,
            _ => panic!("animator is already stopped when stop was called!"),
        };

        let render_data = app.animation_system.visible_animators[index];

        // cancel animation task
        animator_data.task.cancel(&mut app.tasks);

        // add to invisible animators
        let index = app.animation_system.invisible_animators.len();
        app.animation_system.invisible_animators.push(render_data);
        animator_data.list_index = AnimatorListIndex::Invisible(index);

        // remove from visible animators
        let last = app.animation_system.visible_animators.pop().expect("visible animators empty");

        if last.id != self {
            app.animation_system.visible_animators[index] = last;
            let mut moved_data = app.animation_system.animators_data.get_mut(&last.id)
                .expect("corrupted animator data");

            moved_data.list_index = AnimatorListIndex::Visible(index);
        }
    }

    fn next_frame<S>(self, app: &mut App<S>) -> bool {
        let mut animator_data = app.animation_system.animators_data
            .get_mut(&self)
            .expect("unknown animator");

        let (animation_data, _) = get_data(
            &mut app.animation_system.animation_sets,
            &mut app.animation_system.animations,
            &mut app.animation_system.frames,
            animator_data.animation_set,
            animator_data.current_animation,
            animator_data.current_frame
        );

        if animator_data.current_frame + 1 < animation_data.frames.len() {
            animator_data.current_frame += 1;
        } else {
            /*
            // @TODO
            if Repetitions::Finite(rep) = animator_data.current_repetition {
                if rep == 0 { return false; }
            } else {
            }
            */

            animator_data.current_frame = 0;
        }

        let (_, frame_data) = get_data(
            &mut app.animation_system.animation_sets,
            &mut app.animation_system.animations,
            &mut app.animation_system.frames,
            animator_data.animation_set,
            animator_data.current_animation,
            animator_data.current_frame
        );

        match animator_data.list_index {
            AnimatorListIndex::Visible(index) => {
                let mut render_data = &mut app.animation_system.visible_animators[index];
                render_data.sprite = frame_data.sprite;
            },
            AnimatorListIndex::Invisible(index) => {
                let mut render_data = &mut app.animation_system.invisible_animators[index];
                render_data.sprite = frame_data.sprite;
            },
        }

        true
    }

    fn schedule_play_frame<S: GameState>(self, app: &mut App<S>) {
        let mut animator_data = app.animation_system.animators_data
            .get_mut(&self)
            .expect("unknown animator");

        // schedule animation task
        let (animation_data, frame_data) = get_data(
            &mut app.animation_system.animation_sets,
            &mut app.animation_system.animations,
            &mut app.animation_system.frames,
            animator_data.animation_set,
            animator_data.current_animation,
            animator_data.current_frame
        );

        if animation_data.frames.len() > 1 {
            let animator = self;

            animator_data.task = schedule_task(
                &mut app.tasks,
                &app.time,
                frame_data.duration,
                move |id, _state, app| {
                    println!("animation task called: {}", id);
                    animator.next_frame(app);
                    animator.schedule_play_frame(app);
                }
            );
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct AnimatorData {
    id: Animator,
    animation_set: AnimationSet,

    // @Refactor make this somehow safe
    current_animation: usize,
    current_frame: usize,
    current_repetition: Repetitions,

    list_index: AnimatorListIndex,

    is_playing: bool,

    task: Task,
    //completion_callback: FnMut(...)
}

#[derive(Copy, Clone, Debug)]
struct AnimatorRenderData {
    // @AfterGame this is cached data from entities. I could make the AnimationSystem only create
    //            data, but entity containers should be the ones that have render data grouped to
    //            iterate in a cache friendly way
    id: Animator,
    sprite: Sprite,
    transform: Transform,
    //entity: Entity,
    //flip_x: bool,
}

#[derive(Copy, Clone, Debug)]
enum AnimatorListIndex {
    Visible(usize),
    Invisible(usize),
}

#[derive(Default)]
pub struct AnimationSystem {
    animation_sets: Vec<AnimationSetData>,
    animations: Vec<AnimationData>,
    frames: Vec<FrameData>,

    animators_next_id: u64,

    visible_animators: Vec<AnimatorRenderData>,
    invisible_animators: Vec<AnimatorRenderData>,

    animators_data: HashMap<Animator, AnimatorData>,

    // @Speed HashMaps can be removed from data by remapping when animators are freed (by moving the
    //        last element of the array to the one just freed and moving the id of the moved one to
    //        the position. This way the id must store the array position of animators).
}

impl AnimationSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AnimationSet(u64);
struct AnimationSetData {
    id: AnimationSet,
    animations: Vec<Animation>,
}

#[derive(Copy, Clone, Debug)]
pub struct Animation(u64);
struct AnimationData {
    id: Animation,
    repetitions: Repetitions,
    frames: Vec<Frame>,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame(u64);
struct FrameData {
    id: Frame,
    sprite: Sprite,
    duration: u64, // @Refactor create type-safe time/duration struct
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

    pub fn build_animator(
        &mut self,
        animation_set: AnimationSet,
        transform: Transform,
        /*, entity? */
    ) -> Animator {

        let system = &mut self.animation_system;

        let id = system.animators_next_id;
        system.animators_next_id += 1;

        let animator = Animator(id);

        let animation = system.animation_sets[animation_set.0 as usize].animations[0];
        let frame = system.animations[animation.0 as usize].frames[0];
        let sprite = system.frames[frame.0 as usize].sprite;

        let index = system.invisible_animators.len();
        system.invisible_animators.push(AnimatorRenderData {
            id: animator,
            sprite,
            transform,
        });

        system.animators_data.insert(animator, AnimatorData {
            id: animator,
            animation_set,

            current_animation: 0usize,
            current_frame: 0usize,
            current_repetition: Repetitions::Finite(0),

            list_index: AnimatorListIndex::Invisible(index),

            is_playing: false,

            task: Task::empty(),
        });

        animator
    }

    pub fn render_animators(&mut self) {
        let visible_animators = &self.animation_system.visible_animators;

        for animator_render_data in visible_animators.iter() {
            self.renderer.queue_draw_sprite(
                0 as Program,
                color::WHITE,
                &animator_render_data.transform,
                &animator_render_data.sprite
            );
        }
    }
}

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
