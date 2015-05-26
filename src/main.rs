extern crate uuid;

pub mod pomorust;

use pomorust::config::config::read_task_file;

fn main() {
    let task_file = read_task_file();
    for t in task_file {
        println!("Task : {}", t.to_string())
    }
}
