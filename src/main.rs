#![feature(process_exec)]
#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;
extern crate notify_rust;
extern crate ini;

pub mod pomorust;

use std::env;

use pomorust::config::config;
use pomorust::actions::{parse, Command};
use pomorust::model::context::Context;
use pomorust::model::tasks::Task;
use pomorust::utils;

fn check_if_background_proc() -> bool {
    match env::args().nth(1) {
        Some(s) => { 
            if s == "new_pomodoro" {
                return true
            }
        },
        _ => ()
    }
    return false
}

fn main() {
    let mut context = config::create_context();
    if check_if_background_proc() {
        start_task(&mut context, env::args().nth(2).expect("Invalid background call"))
    }
    else {
        match parse() {
            Command::TaskNew(Some(t)) => { add_task(&mut context, t); },
            Command::TaskList => { list_task(context); },
            Command::TaskStart(Some(t)) => { utils::run_background_process(t); }
            _ => panic!("Invalid command")
        }
    }
}

fn add_task(context: &mut Context,  task: Task) {
    println!("New task :\t{}", task.to_string());
    context.add_task(task);
    config::write_task_file(&context.tasks).unwrap();
}

fn list_task(context: Context) {
    for t in context.tasks {
        println!("{}", t.to_string());
    }
}

fn start_task(context: &mut Context, identifier: String) {
    {
        let started_task = match identify_task(&mut context.tasks, identifier) {
            MatchingResult::NoMatch => panic!("No such task !"),
            MatchingResult::AmbiguousMatch => panic!("Too many possible tasks !"),
            MatchingResult::OneMatch(t) => t
        };
        println!("Starting task : {}", started_task.to_string());
        started_task.start();
    }
    if context.use_notification {
        utils::notify("Pomodoro done !", "Pomodoro done. Take a 5 minutes break !");
    }
    config::write_task_file(&context.tasks).unwrap();
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
