#![feature(process_exec)]
#![feature(type_ascription)]
extern crate chrono;
extern crate xdg;
extern crate uuid;
extern crate argparse;
extern crate notify_rust;
extern crate ini;
extern crate rodio;

pub mod pomorust;

use std::env;
use std::process;

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
    if context.has_ongoing_task() {
        println!("You are already doing a task ! Mark it as done if you're over before starting a new one.");
        process::exit(0);
    }
    {
        if context.is_valid_identifier(&identifier).is_ok() {
            let started_task = context.get_task(&identifier);
            println!("Starting task : {}", started_task.to_string());
            // Set flags and starting date...
            started_task.start();
        } else {
            panic!("Not a valid identifier !");
        }
    }
    // ... and save this state.
    config::write_task_file(&context.tasks).unwrap();
    {
        let started_task = context.get_task(&identifier);
        // Only then, do the pomodoro itself.
        started_task.do_one_pomodoro();
    }
    after_pomodoro(&context);
}


fn after_pomodoro(context: &Context) {
    if context.use_notification {
        utils::notify("Pomodoro done !", "Pomodoro done. Take a 5 minutes break !");
    } else {
        println!("Done working !");
    }
    if context.use_sound {
        utils::ding();
    }
    config::write_task_file(&context.tasks).unwrap();
}
