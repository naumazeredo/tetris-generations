use super::super::{
    App,
    animations::{AnimationSet, Repetitions},
    game_state::GameState,
    //renderer::Sprite,
    task_system::{
        Task,
        //schedule_task
    },
};

#[derive(Copy, Clone, Debug)]
struct Animator {
    animation_set: AnimationSet,

    // @Refactor make this somehow safe with newtype idiom
    current_animation: usize,
    current_frame: usize,
    current_repetition: Repetitions,

    is_playing: bool,

    task: Task,
    //completion_callback: FnMut(...)
}

impl Animator {
    // @TODO return result
    pub fn play<S: GameState>(&mut self, app: &mut App<S>) {
        self.schedule_play_frame(app);
    }

    // @Refactor play and stop seems to be basically the same code. Maybe we can just use a single
    //           internal function
    // @TODO return result
    pub fn stop<S>(&mut self, app: &mut App<S>) {
        // cancel animation task
        self.task.cancel(&mut app.task_system);

        // @TODO reset current_frame and current_repetition
    }

    // @TODO pause
    // @TODO change animation

    fn next_frame<S>(&mut self, app: &mut App<S>) -> bool {
        let (animation_data, _) = app.animation_system.get_animation_and_frame(
            self.animation_set,
            self.current_animation,
            self.current_frame
        );

        if self.current_frame + 1 < animation_data.frames.len() {
            self.current_frame += 1;
        } else {
            if let Repetitions::Finite(repetition) = self.current_repetition {
                if let Repetitions::Finite(total_repetitions) = animation_data.repetitions {
                    if repetition == total_repetitions {
                        return false;
                    }
                } else {
                    panic!("animation_data is in a loop and current_repetition is finite");
                }

                self.current_repetition = Repetitions::Finite(repetition + 1);
            }

            self.current_frame = 0;
        }

        /*
        // @XXX just get frame_data in the start with the other get_animation_and_frame
        let (_, frame_data) = get_animation_and_frame(...);
        */

        // @TODO update Sprite from Entity

        true
    }

    fn schedule_play_frame<S: GameState>(&mut self, app: &mut App<S>) {
        // schedule animation task
        let (animation_data, frame_data) = app.animation_system.get_animation_and_frame(
            self.animation_set,
            self.current_animation,
            self.current_frame
        );

        // @TODO 
        /*
        if animation_data.frames.len() > 1 {
            let animator = self;

            self.task = schedule_task(
                &mut app.task_system,
                &app.time,
                frame_data.duration,
                move |id, _state, app| {
                    println!("animation task called: {}", id);
                    animator.next_frame(app);
                    animator.schedule_play_frame(app);
                }
            );
        }
        */
    }
}

