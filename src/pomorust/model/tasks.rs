extern crate uuid;
use self::uuid::Uuid;
use std::thread;

pub struct Task {
    desc:               String,
    uuid:               Uuid,
    pomodori_count:     u16,
    pomodori_estimate:  u16,
    comment:            String
}

impl Task {
    pub fn new(desc: &str) -> Task {
        Task::new_preset(desc, Uuid::new_v4(), 0, 0, "")
    }

    pub fn new_estimate(desc: &str, estimate: u16) -> Task {
        Task::new_preset(desc, Uuid::new_v4(), 0, estimate, "")
    }

    pub fn new_preset(desc: &str, uuid: Uuid,
           pomodori_count: u16, 
           pomodori_estimate: u16,
           comment: &str) -> Task {
        Task {
            desc: desc.to_string(),
            uuid: uuid,
            pomodori_count: pomodori_count,
            pomodori_estimate: pomodori_estimate,
            comment: comment.to_string()
        }
    }

    fn start(&mut self) {
        println!("Started working on : {}", self.to_string());
        wait_for(25);
        self.increment_pomodoro();
        println!("Done working on : {}", self.to_string());
    }

    fn increment_pomodoro(&mut self) {
        self.pomodori_count += 1;
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        self.desc.to_string()
    }
}

fn wait_for(minutes: u32) {
    thread::sleep_ms(minutes * 60 * 1000);
}
