use std::process::exit;
use std::str::FromStr;
use std::io::{stdout, stderr};

use argparse::{ArgumentParser, Store, StoreOption, List, StoreFalse};
use pomorust::model::Task;


#[derive(Debug)]
pub enum Command {
    TaskStart(Option<String>),
    TaskNew(Option<Task>),
    TaskDone(Option<String>),
    TaskList(Option<ListingOption>),
    Status
}

#[derive(Debug)]
pub struct ListingOption {
    pub only_current: bool
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "start" => Ok(Command::TaskStart(None)),
            "new" => Ok(Command::TaskNew(None)),
            "list" => Ok(Command::TaskList(None)),
            "done" => Ok(Command::TaskDone(None)),
            "status" => Ok(Command::Status),
            _ => Err(())
        }
    }
}


fn new_task(args: Vec<String>) -> Command {
    let mut description = "".to_string();
    let mut pomodori_estimate = 0;
    let mut kind = None;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Add a new task");
        ap.refer(&mut description).required().add_argument(
            "description", Store,
            "Short description of the task");
        ap.refer(&mut pomodori_estimate).add_option(
            &["-e", "--estimated"], Store,
            "Number of pomodori you think this task will take");
        ap.refer(&mut kind).add_option(
            &["--type"], StoreOption,
            "The general category of this task if any");
        parse_or_usage(&ap, ap.parse(args, &mut stdout(), &mut stderr()));
    }
    let t = Task::new(&description, pomodori_estimate, kind);
    Command::TaskNew(Some(t))
}

fn identify(args: Vec<String>) -> Option<String> {
    let mut uuid_begin = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Acts upon an identified command");
        ap.refer(&mut uuid_begin).required().add_argument(
            "identifier", Store,
            r#"Beginning of the UUID of the task"#);
        parse_or_usage(&ap, ap.parse(args, &mut stdout(), &mut stderr()));
    }
    Some(uuid_begin)
}

fn list_task(args: Vec<String>) -> Command {
    let mut listing_option = ListingOption { only_current: true };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Lists all registered tasks");
        ap.refer(&mut listing_option.only_current).add_option(&["-a", "--all"], StoreFalse,
            "Display every tasks, even those who are done");
        parse_or_usage(&ap, ap.parse(args, &mut stdout(), &mut stderr()));
    }
    Command::TaskList(Some(listing_option))
}

pub fn parse_or_usage(parser: &ArgumentParser, res: Result<(), i32>) {
    match res {
        Ok(()) => (),
        Err(_) => {
            println!("Unknown command.");
            parser.print_help(&"Pomorust", &mut stdout()).unwrap();
            exit(1);
        }
    }
}

pub fn parse() -> Command {
    let mut subcommand = Command::Status;
    let mut args = vec!();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pomodoro technique utility");
        ap.refer(&mut subcommand).required()
            .add_argument("command", Store,
                          r#"Command to run ("start", "new", "list", "done")"#);
        ap.refer(&mut args).
            add_argument("arguments", List, r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        parse_or_usage(&ap, ap.parse_args());
    }
    args.insert(0, format!("subcommand {:?}", subcommand));
    match subcommand {
        Command::TaskStart(_) => Command::TaskStart(identify(args)),
        Command::TaskNew(_) => new_task(args),
        Command::TaskDone(_) => Command::TaskDone(identify(args)),
        Command::TaskList(_) => list_task(args),
        _ => subcommand
    }
}
