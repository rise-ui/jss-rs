use super::{Easing, TaskProperties, AnimationError};
use std::collections::HashMap;
use utils::extract_unit_value;
use yoga::StyleUnit;

/// The basic action of transition properties into the common task queue.
/// There are several types of tasks currently performed in the queue:
/// * Basic - one-time tween "to"
/// * Parallel - multiple one-time tweens
/// * Delay - between actions in task
#[derive(Clone, Debug)]
pub enum Action {
    Parallel(Vec<ActionBasic>),
    Basic(ActionBasic),
    Delay(ActionDelay),
}

/// Basic linar transition action
#[derive(Clone, Debug)]
pub struct ActionBasic {
    target: HashMap<String, StyleUnit>,
    easing: Easing,
    finished: bool,
    forever: bool,
    duration: u32,
    elapsed: f32,
}

/// Delay action for delay between linear actions
#[derive(Clone, Debug)]
pub struct ActionDelay {
    finished: bool,
    duration: u32,
    elapsed: f32,
}

impl Action {
    /// Return current complition status of action
    pub fn finished(&self) -> bool {
        use self::Action::*;

        match self {
            Parallel(actions) => actions.binary_search_by_key(&false, |ref action| action.finished).is_err(),
            Basic(action) => action.finished,
            Delay(delay) => delay.finished,
        }
    }

    /// Iterate for handle new value of transition action and return finished status after iterate
    pub fn next(&mut self, properties: &mut TaskProperties, delta_time: f32) -> Result<bool, AnimationError> {
        use self::Action::*;

        // @todo: add more error handling
        match self {
            Basic(action) => {
                action.next(properties, delta_time);
            }
            Delay(delay) => {
                delay.next(delta_time);
            }
            Parallel(actions) => {
                for action in actions.iter_mut() {
                    action.next(properties, delta_time);
                }
            }
        }

        Ok(self.finished())
    }
}

impl ActionBasic {
    pub fn new(target: HashMap<String, StyleUnit>, duration: u32, easing: Easing) -> ActionBasic {
        ActionBasic {
            finished: false,
            forever: false,
            elapsed: 0.0,
            duration,
            target,
            easing,
        }
    }

    /// Return current complition status of action
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Iterate for handle new value of transition action and return finished status after iterate
    pub fn next(&mut self, properties: &mut TaskProperties, delta_time: f32) -> Result<bool, AnimationError> {
        use self::AnimationError::*;
        use self::StyleUnit::*;

        if self.elapsed < self.duration as f32 {
            let progress = self.elapsed / self.duration as f32;

            for (key, end_unit) in self.target.iter() {
                let start_unit = properties.started.get(key).ok_or(MissingProperty(key.clone()))?;

                let start = extract_unit_value(&start_unit);
                let end = extract_unit_value(&end_unit);

                let updated = self.easing.transition(&start, &end, &progress);
                let updated = match end_unit {
                    Percent(_) => Percent(updated.into()),
                    Point(_) => Point(updated.into()),
                    // Undefined, Auto not supported
                    _ => Point(0.0.into()),
                };

                properties.worked.insert(key.clone(), updated);
            }

            self.elapsed += delta_time;
            Ok(false)
        } else {
            for (key, end_unit) in self.target.iter() {
                properties.worked.insert(key.clone(), end_unit.clone());
            }

            self.elapsed += delta_time;
            self.finished = true;
            Ok(true)
        }
    }
}

impl ActionDelay {
    pub fn new(duration: u32) -> ActionDelay {
        ActionDelay {
            finished: false,
            elapsed: 0.0,
            duration,
        }
    }

    /// Return current complition status of action
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Iterate for handle elapsed of action, return finished status after iterate
    pub fn next(&mut self, delta_time: f32) -> Result<bool, AnimationError> {
        if self.elapsed < self.duration as f32 {
            self.elapsed += delta_time;
            Ok(false)
        } else {
            self.finished = true;
            Ok(true)
        }
    }
}
