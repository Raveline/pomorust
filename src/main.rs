#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;

pub mod pomorust;

use pomorust::config::config;
use pomorust::actions;
use pomorust::model::context::Context;
use pomorust::model::tasks::Task;


fn main() {
    let context = config::create_context();
    match actions::parse() {
        actions::Command::TaskNew(Some(t)) => { add_task(context, t); },
        actions::Command::TaskList => { list_task(context); },
        actions::Command::TaskStart(Some(t)) => { start_task(context, t); }
        _ => panic!("Invalid command")
    }
}

fn add_task(mut context: Context,  task: Task) {
    println!("New task : {}", task.to_string());
    context.add_task(task);
    config::write_task_file(context.tasks)
}

fn list_task(context: Context) {}

fn start_task(context: Context, task: Task) {}
