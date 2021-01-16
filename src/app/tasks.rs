// @Refactor use a timer wheel instead of a binary heap
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

use super::App;
use super::game_state::GameState;

//#[feature(trait_alias)]
//trait TaskFn<S> = FnMut(u64, &mut S, &mut App<S>);

// @Rename maybe rename to Tasks (or rename all systems to *System)
pub struct TaskSystem<'a, S: GameState + ?Sized> {
    next_id: u64,
    // @Refactor don't store whole structure in heap, only (id, execution_time)
    tasks_scheduled: BinaryHeap<TaskInner<'a, S>>,
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

pub struct Task(u64);

impl<'a, S: GameState> App<'a, S> {
    // @TODO use a type safe duration type
    pub fn schedule_task<F>(&mut self, time_delay: u64, callback: F) -> Task
        where
            F: FnMut(u64, &mut S, &mut App<S>) + 'a,
            //F: TaskFn<S> + 'a,
    {

        let id = self.tasks.next_id;
        self.tasks.next_id += 1;

        let game_time = self.time.game_time;
        let execution_time = if time_delay == 0 { game_time + 1 } else { game_time + time_delay };
        self.tasks.tasks_scheduled.push(TaskInner {
            id,
            execution_time,
            callback: Rc::new(RefCell::new(callback)),
        });

        Task ( id )
    }

    pub fn run_tasks(&mut self, state: &mut S) {
        let game_time = self.time.game_time;
        while let Some(task) = self.tasks.tasks_scheduled.peek() {
            if task.execution_time > game_time {
                break;
            }

            let task = self.tasks.tasks_scheduled.pop().unwrap();

            // @TODO check if task was cancelled or not

            let mut closure = task.callback.borrow_mut();
            (&mut closure)(task.id, state, self);
        }
    }
}

impl Task {
    #[allow(dead_code)]
    pub fn cancel<S: GameState>(&mut self, tasks: &mut TaskSystem<S>) {
        assert!(self.0 != 0);
        tasks._tasks_cancelled.insert(self.0);
        self.0 = 0;
    }
}

// @Refactor don't store whole structure in heap, only (id, execution_time)
struct TaskInner<'a, S: GameState + ?Sized> {
    id: u64,
    execution_time: u64,
    callback: Rc<RefCell<dyn FnMut(u64, &mut S, &mut App<S>) + 'a>>,
    //callback: Rc<RefCell<dyn TaskFn<S> + 'a>>,
}

impl<'a, S: GameState> Ord for TaskInner<'a, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.execution_time.cmp(&other.execution_time).then(self.id.cmp(&other.id)).reverse()
    }
}

impl<'a, S: GameState> PartialOrd for TaskInner<'a, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, S: GameState> PartialEq for TaskInner<'a, S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.execution_time == other.execution_time
    }
}

impl<'a, S: GameState> Eq for TaskInner<'a, S> {}
