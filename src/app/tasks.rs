// @Refactor use a timer wheel instead of a binary heap
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

use super::{
    App,
    game_state::GameState,
    time::Time,
};

//#[feature(trait_alias)]
//trait TaskFn<S> = FnMut(u64, &mut S, &mut App<S>);

// @Rename maybe rename to Tasks (or rename all systems to *System)
pub struct TaskSystem<'a, S> {
    next_id: u64,
    // @Refactor don't store whole structure in heap, only (id, execution_time)
    tasks_scheduled: BinaryHeap<TaskData<'a, S>>,
    _tasks_cancelled: HashSet<u64>,
}

impl<'a, S: GameState> TaskSystem<'a, S> {
    pub fn new() -> Self {
        let tasks_scheduled = BinaryHeap::new();
        let _tasks_cancelled = HashSet::new();
        Self {
            next_id: 1,
            tasks_scheduled,
            _tasks_cancelled,
        }
    }
}

// @Refactor Rc<RefCell<Option<u64>>>. After execution, the task should change the value on all
//           references, so we should have to use this (or we can check if the task is running or
//           exist.
#[derive(Copy, Clone, Debug)]
pub struct Task(Option<u64>);

impl Task {
    pub fn empty() -> Self {
        Self(None)
    }

    #[allow(dead_code)]
    pub fn cancel<S>(&mut self, tasks: &mut TaskSystem<S>) {
        if let None = self.0 {
            panic!("Trying to cancel an empty task");
        }

        let id = self.0.take().unwrap();
        tasks._tasks_cancelled.insert(id);
    }
}

pub fn schedule_task<'a, S: GameState, F>(
    task_system: &mut TaskSystem<'a, S>,
    time_system: &Time,
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

impl<'a, S: GameState> App<'a, S> {
    // @TODO use a type safe duration type
    pub fn schedule_task<F>(&mut self, time_delay: u64, callback: F) -> Task
        where
            F: FnMut(u64, &mut S, &mut App<S>) + 'a,
            //F: TaskFn<S> + 'a,
    {
        schedule_task(&mut self.tasks, &self.time, time_delay, callback)
    }

    pub fn run_tasks(&mut self, state: &mut S) {
        let game_time = self.time.game_time;
        while let Some(task) = self.tasks.tasks_scheduled.peek() {
            if task.execution_time > game_time {
                break;
            }

            let task = self.tasks.tasks_scheduled.pop().unwrap();

            // @TODO check if task was cancelled or not

            // @TODO logger (with envvar? with argv value?)
            println!("executed task: {} at {}", task.id, game_time);

            let mut closure = task.callback.borrow_mut();
            (&mut closure)(task.id, state, self);
        }
    }
}

// @Refactor don't store whole structure in heap, only (id, execution_time)
struct TaskData<'a, S> {
    id: u64,
    execution_time: u64,
    callback: Rc<RefCell<dyn FnMut(u64, &mut S, &mut App<S>) + 'a>>,
    //callback: Rc<RefCell<dyn TaskFn<S> + 'a>>,
}

impl<'a, S: GameState> Ord for TaskData<'a, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.execution_time.cmp(&other.execution_time).then(self.id.cmp(&other.id)).reverse()
    }
}

impl<'a, S: GameState> PartialOrd for TaskData<'a, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, S: GameState> PartialEq for TaskData<'a, S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.execution_time == other.execution_time
    }
}

impl<'a, S: GameState> Eq for TaskData<'a, S> {}
