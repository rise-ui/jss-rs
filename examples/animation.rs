#![feature(duration_as_u128)]

extern crate ordered_float;
extern crate failure;
extern crate yoga;
extern crate rand;
extern crate jss;

#[macro_use]
extern crate maplit;

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use std::thread::sleep;
use yoga::StyleUnit;

use jss::animation::*;

fn main() -> Result<(), failure::Error> {
    use self::StyleUnit::*;

    // Declare new animation
    let mut animation: Animation<String, String> = Animation::default();
    let mut rng = thread_rng();

    // Properties for transitions by action(-s) 
    let properties: HashMap<String, StyleUnit> = hashmap!{
        "border_top".to_string() => Point(50.0.into()),
        "height".to_string() => Point(20.0.into()),
        "width".to_string() => Point(20.0.into()),
    };

    // Elapsed by frame (fps)
    let mut frame_elapsed = Instant::now();
    let task_name = "my_task".to_string();
    
    let task_one = Task {
        properties: TaskProperties {
            // initial state of transitions
            started: properties.clone(),
            // values after transitions
            worked: properties.clone(),
        },

        // Queue of transition actions
        queue: VecDeque::from(vec![
            // Basic linear transition with target properties, with 10sec duration and easing
            Action::Basic(ActionBasic::new(hashmap!{
                "border_top".to_string() => Point(150.0.into()),
                "height".to_string() => Point(100.0.into()),
                "width".to_string() => Point(100.0.into()),
            }, 10000, Easing::Basic(EaseFunction::BounceIn)))
        ]),

        // Status of task
        finished: false,
    };

    // Push new task with actions to animation eventloop queue
    animation.push_task(task_name.clone(), task_one);
    
    loop {
        // Generate random wait fps rate
        let simulate_frame_rate: u64 = rng.gen_range(14000, 20000);
        sleep(Duration::from_micros(simulate_frame_rate));

        // Calc elapsed
        let elapsed_time = frame_elapsed.elapsed();
        let elapsed_ms = elapsed_time.as_micros() as f32 / 1000.0;
        // Run animation tick with fps delta_time
        animation.run(elapsed_ms);

        // Get task if exists and print updated values
        if let Some(task) = animation.get_task(&task_name) {
            print!("Updated (fps: {}ms): {:?}\r", elapsed_ms, task.properties.worked);
        } else {
            break;
        }

        frame_elapsed = Instant::now();
    }

    print!("Transition end!");
    Ok(())
}
