//! Animations module for create simple tick based animations for one or more properties
 
use std::collections::HashMap;
use std::hash::Hash;

mod easing;
mod action;
mod errors;
mod task;

pub use interpolation::EaseFunction;
pub use self::easing::*;
pub use self::action::*;
pub use self::errors::*;
pub use self::task::*;

/// Aniumations simple queue with transition state
#[derive(Clone, Debug, Default)]
pub struct Animation<T, S> where T: Eq + Clone + Hash, S: Eq + Clone + Hash {
    /// Single transformation tasks
    singles: HashMap<S, SingleTask>, 
    /// Complex tasks for transition properties
    tasks: HashMap<T, Task>,
}

impl <T, S>Animation<T, S> where T: Eq + Clone + Hash, S: Eq + Clone + Hash {
    pub fn push_task(&mut self, key: T, task: Task) {
        self.tasks.insert(key, task).is_some();
    }

    pub fn push_single(&mut self, key: S, task: SingleTask) {
        self.singles.insert(key, task).is_some();
    }

    pub fn get_task(&mut self, key: &T) -> Option<&mut Task> {
        self.tasks.get_mut(key)
    }

    pub fn get_single(&mut self, key: &S) -> Option<&mut SingleTask> {
        self.singles.get_mut(key)
    }

    pub fn run(&mut self, delta_time: f32) {
        self.singles.retain(|_, value| !value.finished);
        self.tasks.retain(|_, value| !value.finished);

        // Interate and handle all tasks
        for (key, task) in self.tasks.iter_mut() {
            task.run(delta_time).is_ok();
        }

        for (_, task) in self.singles.iter_mut() {
            task.run(delta_time).is_ok();
        }
    }
}