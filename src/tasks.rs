// @Refactor use a timer wheel instead of a binary heap
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

use crate::game_state::GameState;

pub struct TaskSystem<'a> {
    next_id: u64,
    tasks: BinaryHeap<TaskInner<'a>>,
    _tasks_cancelled: HashSet<u64>,
}

impl<'a> TaskSystem<'a> {
    pub fn new() -> Self {
        let tasks = BinaryHeap::new();
        let _tasks_cancelled = HashSet::new();
        Self {
            next_id: 1,
            tasks,
            _tasks_cancelled,
        }
    }

    pub fn schedule<F: Fn(u64, u64) + 'a>(&mut self, execution_time: u64, callback: F) -> Task {
        let id = self.next_id;
        self.next_id += 1;

        //let execution_time = if millis == 0 { 1 } else { millis * 1_000 };

        self.tasks.push(TaskInner {
            id,
            execution_time,
            callback: Rc::new(RefCell::new(callback)),
        });

        Task ( id )
    }

    pub fn run<S: GameState>(&mut self, _state: &mut S, current_time: u64) {
        while let Some(task) = self.tasks.peek() {
            if task.execution_time > current_time {
                break;
            }

            let task = self.tasks.pop().unwrap();

            // @TODO check if task was cancelled or not

            let mut closure = task.callback.borrow_mut();
            (&mut closure)(task.id, task.execution_time);
        }
    }
}

pub struct Task(u64);

impl Task {
    #[allow(dead_code)]
    pub fn cancel(&mut self, task_system: &mut TaskSystem) {
        assert!(self.0 != 0);
        task_system._tasks_cancelled.insert(self.0);
        self.0 = 0;
    }
}

struct TaskInner<'a> {
    id: u64,
    execution_time: u64,
    callback: Rc<RefCell<dyn Fn(u64, u64) + 'a>>,
}

impl<'a> Ord for TaskInner<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.execution_time.cmp(&other.execution_time).then(self.id.cmp(&other.id)).reverse()
    }
}

impl<'a> PartialOrd for TaskInner<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for TaskInner<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.execution_time == other.execution_time
    }
}

impl<'a> Eq for TaskInner<'a> {}
