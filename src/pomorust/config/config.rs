use std::fs::File;
use std::io::Read;
use std::string::ToString;
use pomorust::model::tasks::Task;
use std::option;
use uuid;

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

fn write_task_file(tasks: Vec<Task>, file: File) {
    let tasks_as_strings = tasks.iter().map(|x| x.to_csv()).collect::<Vec<String>>();
    let mut file = match File::create("tasks") {
        Ok(file) => file,
        Err(_) => panic!("Could not create tasks file.")
    };
}
