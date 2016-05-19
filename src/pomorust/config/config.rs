use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::Write;
use pomorust::model::tasks::Task;
use pomorust::model::context::Context;
use ini::Ini;

const CONF_FILE_NAME: &'static str = ".pomorust.ini";

pub fn create_context() -> Context {
    let ini = read_ini_file();
    let main_sec = ini.general_section();
    let ref with_sound = main_sec["use_sound"];
    let ref with_notification = main_sec["use_notification"];
    let task_list = match read_task_file() {
        Some(ts) => ts,
        None => Vec::new()
    };
    Context { tasks: task_list,
              use_notification: with_notification == "true",
              use_sound: with_sound == "true" }
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
    conf.write_to_file(CONF_FILE_NAME).unwrap();
    conf
}

pub fn read_task_file() -> Option<Vec<Task>> {
    let mut file = match File::open("tasks") {
        Ok(file) => file,
        Err(_) => return None
    };
    let mut file_txt = String::new();
    match file.read_to_string(&mut file_txt) {
        Ok(_) => {},
        Err(_) => return None
    };
    let mut data: Vec<Task> = Vec::new();
    for l in file_txt.split("\n").collect::<Vec<&str>>() {
        if l.len() > 0 {
            data.push(Task::from_csv(l));
        }
    }
    Some(data)
}

pub fn write_task_file(tasks: &Vec<Task>) -> Result<(), Error> {
    let tasks_as_strings = tasks.iter().map(|x| x.to_csv()).collect::<Vec<String>>();
    let mut file = File::create("tasks").unwrap();
    for s in tasks_as_strings {
        try!(file.write(&s.into_bytes()));
    }
    try!(file.sync_all());
    Ok(())
}
