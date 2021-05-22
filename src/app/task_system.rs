// Task system

// [ ] Refactor to timer wheel
// [ ] Add next frame scheduling
// [ ] Add repeating task

///
/// // construction
///
/// let task_system = TaskSystem::new();
///
/// // scheduling
///
/// let task = app.schedule_task(1_000_000, |id, _state, app| {
///    println!("task {} {}", id, app.game_time());
/// });
///
/// // cancel
///
/// app.cancel_task(&mut task);
///

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

use crate::app::{
    App,
    time_system::TimeSystem,
    imgui::ImDraw,
};

//#[feature(trait_alias)]
//trait TaskFn<S> = FnMut(u64, &mut S, &mut App<S>);

pub(in crate::app) struct TaskSystem<'a, S> {
    next_id: u64,
    // @Refactor don't store whole structure in heap, only (id, execution_time)
    tasks_scheduled: BinaryHeap<TaskData<'a, S>>,
    tasks_cancelled: HashSet<u64>,
}

impl<'a, S> TaskSystem<'a, S> {
    pub(in crate::app) fn new() -> Self {
        let tasks_scheduled = BinaryHeap::new();
        let tasks_cancelled = HashSet::new();
        Self {
            next_id: 1,
            tasks_scheduled,
            tasks_cancelled,
        }
    }
}

// @TODO implement a proper ImDraw
// @Refactor maybe we shouldn't implement Copy or Clone
// @Refactor maybe we should implement a method 'new'
#[derive(Copy, Clone, Debug, Default, ImDraw)]
pub struct Task(Option<u64>);

impl Task {
    // @TODO return Option/Result
    fn cancel<S>(&mut self, task_system: &mut TaskSystem<S>) {
        let id = self.0.take().expect("Trying to cancel an empty task");
        task_system.tasks_cancelled.insert(id);
    }
}

fn schedule_task<'a, S, F>(
    task_system: &mut TaskSystem<'a, S>,
    time_system: &TimeSystem,
    time_delay: u64,
    callback: F
) -> Task
    where
        F: FnMut(u64, &mut S, &mut App<S>) + 'a,
        //F: TaskFn<S> + 'a,
{
    let id = task_system.next_id;
    task_system.next_id += 1;

    let game_time = time_system.game_time;
    let execution_time = if time_delay == 0 { game_time + 1 } else { game_time + time_delay };
    task_system.tasks_scheduled.push(TaskData {
        id,
        execution_time,
        callback: Rc::new(RefCell::new(callback)),
    });

    // @TODO logger (with envvar? with argv value?)
    //println!("scheduled task: {} at {}", id, game_time);

    Task ( Some(id) )
}

impl<'a, S> App<'a, S> {
    // @TODO use a type safe duration type
    pub fn schedule_task<F>(&mut self, time_delay: u64, callback: F) -> Task
    where F: FnMut(u64, &mut S, &mut App<S>) + 'a, // @XXX F: TaskFn<S> + 'a,
    {
        schedule_task(&mut self.task_system, &self.time_system, time_delay, callback)
    }

    // @TODO return Option/Result
    pub fn cancel_task(&mut self, task: &mut Task) {
        task.cancel(&mut self.task_system);
    }

    pub fn run_tasks(&mut self, state: &mut S) {
        let game_time = self.time_system.game_time;
        while let Some(task) = self.task_system.tasks_scheduled.peek() {
            if task.execution_time > game_time {
                break;
            }

            let task = self.task_system.tasks_scheduled.pop().unwrap();

            // check if task was cancelled or not
            if self.task_system.tasks_cancelled.remove(&task.id) {
                continue;
            }

            // @TODO logger (with envvar? with argv value?)
            //println!("executed task: {} at {}", task.id, game_time);

            let mut closure = task.callback.borrow_mut();
            (&mut closure)(task.id, state, self);
        }
    }
}

// @Refactor don't store whole structure in heap, only (id, execution_time)
// @Refactor maybe we don't need Rc<RefCell<>>
struct TaskData<'a, S> {
    id: u64,
    execution_time: u64,
    callback: Rc<RefCell<dyn FnMut(u64, &mut S, &mut App<S>) + 'a>>,
    //callback: Rc<RefCell<dyn TaskFn<S> + 'a>>,
}

impl<'a, S> Ord for TaskData<'a, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.execution_time.cmp(&other.execution_time).then(self.id.cmp(&other.id)).reverse()
    }
}

impl<'a, S> PartialOrd for TaskData<'a, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, S> PartialEq for TaskData<'a, S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.execution_time == other.execution_time
    }
}

impl<'a, S> Eq for TaskData<'a, S> {}
