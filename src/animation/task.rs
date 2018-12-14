use std::collections::{HashMap, VecDeque};
use super::{Easing, Action, AnimationError};
use yoga::StyleUnit;

/// Task properties needed for active task
#[derive(Clone, Debug)]
pub struct TaskProperties {
    /// Started properties of transition
    pub started: HashMap<String, StyleUnit>,
    // Worked properties - for update properties
    pub worked: HashMap<String, StyleUnit>,
}

/// Task is named queue of linear actions - stored and runned in main `Animation` instance
#[derive(Clone, Debug)]
pub struct Task {
    pub properties: TaskProperties,
    pub queue: VecDeque<Action>,
    pub finished: bool,
}

/// Single interpolation task for transition an float value
#[derive(Clone, Debug)]
pub struct SingleTask {
    pub easing: Easing,
    pub finished: bool,
    pub duration: u32,
    pub elapsed: u32,
    pub target: f32,
    pub start: f32,
}

impl Task {
    /// Tick for handle new value of transitions queue action and return finished status after iterate
    pub fn run(&mut self, delta_time: f32) -> Result<bool, AnimationError> {
        let mut pop_action = false;

        if let Some(task) = self.queue.front_mut() {
            if let Ok(status) = task.next(&mut self.properties, delta_time) {
                pop_action = status;
            }
        } else {
            self.finished = true;
            return Ok(true);
        }

        if pop_action {
            let finished = self.queue.pop_front().is_none();
            self.finished = finished;
            Ok(finished)
        } else {
            Ok(false)
        }
    }
}

impl SingleTask {
    /// Tick for run transition for value
    pub fn run(&mut self, delta_time: f32) -> Result<bool, AnimationError> {
        Ok(true)
    }
}
