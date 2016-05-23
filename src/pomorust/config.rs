use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use ini::Ini;
use xdg;
use pomorust::model::Task;
use pomorust::model::Context;

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
    let mut context = read_task_file();
    context.use_sound = with_sound == "true";
    context.use_notification = with_notification == "true";
    context
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

pub fn read_task_file() -> Context {
    let mut file = match File::open(&get_path_for(Path::new(TASK_FILE_NAME))) {
        Ok(file) => file,
        Err(_) => return Context::default()
    };
    let mut file_txt = String::new();
    match file.read_to_string(&mut file_txt) {
        Ok(_) => {},
        Err(_) => return Context::default()
    };

    let lines = file_txt.split("\n").collect::<Vec<&str>>();
    let context_line = lines[0];
    let mut context = Context::from_csv_line(&context_line);

    let mut tasks: Vec<Task> = Vec::new();
    for l in lines.into_iter().skip(1) {
        if l.len() > 0 {
            tasks.push(Task::from_csv(l));
        }
    }
    context.tasks = tasks;
    context
}

pub fn write_task_file(context: &Context) -> Result<(), Error> {
    let tasks_as_strings = context.tasks.iter().map(|x| x.to_csv()).collect::<Vec<String>>();
    let mut file = File::create(&get_path_for(Path::new(TASK_FILE_NAME))).unwrap();

    try!(file.write(&context.metadata_to_csv_line().into_bytes()));
    for s in tasks_as_strings {
        try!(file.write(&s.into_bytes()));
    }
    try!(file.sync_all());
    Ok(())
}
