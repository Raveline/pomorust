use std::process;
use std::str::FromStr;
use std::io::{stdout, stderr};

use argparse::{ArgumentParser, StoreTrue, Store, List};
use pomorust::model::tasks::Task;


#[derive(Debug)]
enum Command {
    TaskStart,
    TaskNew,
    TaskList
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "start" => Ok(Command::TaskStart),
            "new" => Ok(Command::TaskNew),
            "list" => Ok(Command::TaskList),
            _ => Err(())
        }
    }
}

fn start_task(args: Vec<String>) {
    let mut uuid = "".to_string();
    let mut ap = ArgumentParser::new();
    ap.set_description("Starts a task");
    ap.refer(&mut uuid).required().add_argument(
        "Task identifier", Store,
        r#"Uuid or part of the UUID to identify the task"#);
    match ap.parse(args, &mut stdout(), &mut stderr()) {
        Ok(()) => {},
        Err(x) => {
            process::exit(x);
        }
    }
}

fn new_task(args: Vec<String>) {
    let mut description = "".to_string();
    let mut pomodori_estimate = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Add a new task");
        ap.refer(&mut description).required().add_argument(
            "description", Store,
            r#"Short description of the task"#);
        ap.refer(&mut pomodori_estimate).add_argument(
            "estimated_number", Store,
            r#"Number of pomodori you think this task will take"#);
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(()) => {}
            Err(x) => {println!("{}", x)}
        }
    }
    let t = Task::new(&description, pomodori_estimate);
}

fn list_task(args: Vec<String>) {
}

pub fn parse() {
    let mut subcommand = Command::TaskStart;
    let mut args = vec!();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pomodoro utility");
        ap.refer(&mut subcommand).required()
            .add_argument("command", Store,
                          r#"Command to run ("start", "new", "list")"#);
        ap.refer(&mut args).
            add_argument("arguments", List, r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }
    args.insert(0, format!("subcommand {:?}", subcommand));
    match subcommand {
        Command::TaskStart => start_task(args),
        Command::TaskNew => new_task(args),
        Command::TaskList => list_task(args),
    }
}
