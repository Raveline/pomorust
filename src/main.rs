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
use pomorust::model::{Context, Task, TaskModification};
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
            Command::TaskStart(Some(t)) => { utils::run_background_process(t); },
            Command::TaskDone(Some(t)) => { mark_as_done(&mut context, t); },
            Command::Status => { context.display_status(); },
            Command::TaskModify(Some((i, m))) => { modify_task(&mut context, i, m); },
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
        false => context.get_all_tasks()
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
    if context.is_valid_identifier(&identifier).is_ok() {
		before_pomodoro(context, &identifier);
    } else {
        panic!("Not a valid identifier !");
    }
    config::write_task_file(context).unwrap();
	do_pomodoro(identifier);
}

fn before_pomodoro(context: &mut Context, identifier: &str) {
    {
        let task = context.get_task(identifier);
        task.before_starting_pomodoro();
        println!("Starting task : {}", task.to_string());
    }
    // ... and save this state.
    config::write_task_file(context).unwrap();
}

fn do_pomodoro(identifier: String) {
    // Only then, do the pomodoro itself.
    utils::wait_for(25);

    // Now that we are here, the context might have changed:
    // user could have added some tasks, for instance. Reload
    // context.
    let mut updated_context = config::create_context();
    {
        let mut worked_upon_task = updated_context.get_task(&identifier);
        worked_upon_task.after_doing_pomodoro();
    }
    updated_context.increment_pomodoro_count();
    updated_context.pause = true;
    updated_context.timer = Some(chrono::Local::now());
    config::write_task_file(&updated_context).unwrap();
    if updated_context.should_be_long_pause() {
        pause(&updated_context, 30);
    } else {
        pause(&updated_context, 5);
    }
    updated_context.pause = false;
    updated_context.timer = None;
    config::write_task_file(&updated_context).unwrap();
}

fn pause(context: &Context, minutes: u16) {
    notify_according_to_context(&context, "Pomodoro done !", &format!("Take a {} minute break !", minutes));
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

fn modify_task(context: &mut Context, identifier: String,
               modification: TaskModification) {
    if context.is_valid_identifier(&identifier).is_ok() {
        let task = context.get_task(&identifier);
        task.modify(modification);
    }
    config::write_task_file(&context).unwrap();
}
