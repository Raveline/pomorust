use std::process;
use std::str::FromStr;
use std::io::{stdout, stderr};

use argparse::{ArgumentParser, StoreTrue, Store, List};
use pomorust::model::tasks::Task;


#[derive(Debug)]
pub enum Command {
    TaskStart(Option<Task>),
    TaskNew(Option<Task>),
    TaskList
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "start" => Ok(Command::TaskStart(None)),
            "new" => Ok(Command::TaskNew(None)),
            "list" => Ok(Command::TaskList),
            _ => Err(())
        }
    }
}

fn start_task(args: Vec<String>) -> Command {
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
    Command::TaskStart(None)
}

fn new_task(args: Vec<String>) -> Command {
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
    Command::TaskNew(Some(t))
}

fn list_task(args: Vec<String>) -> Command {
    // We might want to add some filtering options here one day
    Command::TaskList
}

pub fn parse() -> Command {
    let mut subcommand = Command::TaskList;
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
        Command::TaskStart(_) => start_task(args),
        Command::TaskNew(_) => new_task(args),
        Command::TaskList => list_task(args),
    }
}
