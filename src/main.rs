#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;

pub mod pomorust;

use pomorust::config::config;
use pomorust::actions::{parse, Command};
use pomorust::model::context::Context;
use pomorust::model::tasks::Task;


fn main() {
    let context = config::create_context();
    match parse() {
        Command::TaskNew(Some(t)) => { add_task(context, t); },
        Command::TaskList => { list_task(context); },
        Command::TaskStart(Some(t)) => { start_task(context, t); }
        _ => panic!("Invalid command")
    }
}

fn add_task(mut context: Context,  task: Task) {
    println!("New task :\t{}", task.to_string());
    context.add_task(task);
    config::write_task_file(context.tasks)
}

fn list_task(context: Context) {
    for t in context.tasks {
        println!("{}", t.to_string());
    }
}

fn start_task(context: Context, task: Task) {}
