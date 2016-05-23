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

use pomorust::config;
use pomorust::actions::{parse, Command, ListingOption};
use pomorust::model::Context;
use pomorust::model::Task;
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
            Command::TaskList(Some(o)) => { list_task(context, o); },
            Command::TaskStart(Some(t)) => { utils::run_background_process(t); }
            Command::TaskDone(Some(t)) => { mark_as_done(&mut context, t); }
            Command::Status => { context.display_status(); }
            _ => panic!("Invalid command")
        }
    }
}

fn add_task(context: &mut Context,  task: Task) {
    println!("New task :\t{}", task.to_string());
    context.add_task(task);
    config::write_task_file(&context).unwrap();
}

fn list_task(context: Context, opt: ListingOption) {
    let to_iterate = match opt.only_current {
        true => context.get_current_tasks(),
        false => context.tasks.iter().collect()
    };
    for t in to_iterate {
        println!("{}", t.to_list_line());
    }
}

fn start_task(context: &mut Context, identifier: String) {
    context.timer = Some(chrono::Local::now());
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
    config::write_task_file(&context).unwrap();
    {
        let started_task = context.get_task(&identifier);
        // Only then, do the pomodoro itself.
        started_task.do_one_pomodoro();
    }
    // Now that we are here, the context might have changed:
    // user could have added some tasks, for instance. Reload
    // context.
    let mut updated_context = config::create_context();
    after_pomodoro(&updated_context);
    updated_context.increment_pomodoro_count();
    config::write_task_file(&updated_context).unwrap();
    if updated_context.should_be_long_pause() {
        updated_context.pause = true;
        pause(&updated_context, 30);
        updated_context.pause = false;
    } else {
        updated_context.pause = true;
        pause(&updated_context, 5);
        updated_context.pause = false;
    }
    updated_context.timer = None;
    config::write_task_file(&updated_context).unwrap();
}

fn after_pomodoro(context: &Context) {
    notify_according_to_context(&context, "Pomodoro done !", "Take a 5 minute break !");
}

fn pause(context: &Context, minutes: u16) {
    utils::wait_for(minutes);
    notify_according_to_context(&context, "Break is over !", "Start a new task");
}

fn notify_according_to_context(context: &Context, notif_title: &str, notif_text: &str) {
    if context.use_notification {
        utils::notify(notif_title, notif_text);
    } else {
        println!("{} {}", notif_title, notif_text);
    }
    if context.use_sound {
        utils::ding();
    }
}


fn mark_as_done(context: &mut Context, identifier: String) {
    if context.is_valid_identifier(&identifier).is_ok() {
        {
            let task = context.get_task(&identifier);
            task.finish();
        }
    } else {
        panic!("Could not identify task");
    }
    config::write_task_file(&context).unwrap();
}
