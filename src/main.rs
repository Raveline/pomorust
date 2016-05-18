#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;

pub mod pomorust;

use pomorust::config::config;
use pomorust::actions::{parse, Command};
use pomorust::model::context::Context;
use pomorust::model::tasks::Task;


fn main() {
    let mut context = config::create_context();
    match parse() {
        Command::TaskNew(Some(t)) => { add_task(context, t); },
        Command::TaskList => { list_task(context); },
        Command::TaskStart(Some(t)) => { start_task(&mut context, t); }
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

fn start_task(context: &mut Context, identifier: String) {
    let started_task = match identify_task(&mut context.tasks, identifier) {
        MatchingResult::NoMatch => panic!("No such task !"),
        MatchingResult::AmbiguousMatch => panic!("Too many possible tasks !"),
        MatchingResult::OneMatch(t) => t
    };
    println!("Starting task : {}", started_task.to_string());
    started_task.start();
    println!("Done one pomodoro on : {}", started_task.to_string());
}

enum MatchingResult<'a> {
    NoMatch,
    OneMatch(&'a mut Task),
    AmbiguousMatch
}

fn identify_task<'a>(tasks : &'a mut Vec<Task>, identifier: String) -> MatchingResult<'a> {
    let mut matching = tasks
        .iter_mut()
        .filter(|x| x.can_be_identified_by(&identifier));
    match matching.next() {
        None => MatchingResult::NoMatch,
        Some(n) => match matching.next() {
            None => MatchingResult::OneMatch(n),
            Some(_) => MatchingResult::AmbiguousMatch
        }
    }
}
