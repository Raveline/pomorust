use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use ini::Ini;
use xdg;
use pomorust::model::Task;
use pomorust::model::Context;
use pomorust::utils;

const CONF_FILE_NAME: &'static str = ".pomorust.ini";
const TASK_FILE_NAME: &'static str = "task";

/// Technically, we should put the config file in XDG_CONFIG_HOME.
/// But I find this a bit bothersome for a few lines.
/// So we will put everything in XDG_DATA_HOME.
fn get_path_for(path: &Path) -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("pomorust").unwrap();
    xdg_dirs.place_data_file(path).expect("Could not create rust in local config directory")
}

pub fn create_context() -> Context {
    let ini = read_ini_file();
    let main_sec = ini.general_section();
    let ref with_sound = main_sec["use_sound"];
    let ref with_notification = main_sec["use_notification"];
    let (pomo_count, last_pomo_date, task_list) = match read_task_file() {
        Some((pc, lpd, tl)) => (pc, lpd, tl),
        None => (0, None, Vec::new())
    };
    Context { tasks: task_list,
              use_notification: with_notification == "true",
              use_sound: with_sound == "true",
              pomodori_count: pomo_count,
              last_pomodoro: last_pomo_date
    }
}

pub fn read_ini_file() -> Ini {
    match Ini::load_from_file(CONF_FILE_NAME) {
        Ok(i) => i,
        Err(_) => create_ini_file()
    }
}

pub fn create_ini_file() -> Ini {
    let mut conf = Ini::new();
    conf.with_section(None::<String>)
        .set("use_notification", "true")
        .set("use_sound", "true");
    conf.write_to_file(get_path_for(Path::new(CONF_FILE_NAME)).to_str().expect("Invalid path")).unwrap();
    conf
}

pub fn read_task_file() -> Option<(u16, utils::MaybeLocalDate, Vec<Task>)> {
    let mut file = match File::open(&get_path_for(Path::new(TASK_FILE_NAME))) {
        Ok(file) => file,
        Err(_) => return None
    };
    let mut file_txt = String::new();
    match file.read_to_string(&mut file_txt) {
        Ok(_) => {},
        Err(_) => return None
    };
    let mut tasks: Vec<Task> = Vec::new();
    let lines = file_txt.split("\n").collect::<Vec<&str>>();
    if lines.len() <= 2 {
        return None
    }
    let last_pomo_line = lines[0];
    let pomo_count_line = lines[1];
    for l in lines.into_iter().skip(2) {
        if l.len() > 0 {
            tasks.push(Task::from_csv(l));
        }
    }
    let last_pomo = utils::parse_maybe_local_date(last_pomo_line, "Last pomodoro date is invalid");
    let pomo_count : u16 = pomo_count_line.parse().ok().expect("Could not parse pomodoro count");
    Some((pomo_count, last_pomo, tasks))
}

pub fn write_task_file(context: &Context) -> Result<(), Error> {
    let tasks_as_strings = context.tasks.iter().map(|x| x.to_csv()).collect::<Vec<String>>();
    let mut file = File::create(&get_path_for(Path::new(TASK_FILE_NAME))).unwrap();

    let last_pomodoro_string = context.last_pomodoro.map_or(String::new(), |x|x.to_rfc3339());
    try!(file.write(&last_pomodoro_string.into_bytes()));
    try!(file.write(b"\n"));
    try!(file.write(&context.pomodori_count.to_string().into_bytes()));
    try!(file.write(b"\n"));

    for s in tasks_as_strings {
        try!(file.write(&s.into_bytes()));
    }
    try!(file.sync_all());
    Ok(())
}
