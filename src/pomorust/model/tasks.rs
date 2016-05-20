use uuid::Uuid;
use chrono;
use pomorust::utils::wait_for;

type MaybeLocalDate = Option<chrono::DateTime<chrono::Local>>;

#[derive(Debug)]
pub struct Task {
    /// Main description for the task
    pub description: String,
    /// Single generated identifier for the task
    pub uuid: Uuid,
    /// Number of pomodori spent on this task
    pomodori_count: u16,
    /// Number of pomodori needed according to user
    pomodori_estimate: u16,
    /// Is the user currently doing a pomodoro over this task ?
    pub is_ongoing: bool,
    /// Comment
    comment: String,
    /// When (and if) this task was started for the first time
    start_date: MaybeLocalDate,
    /// When (and if) this task was finished
    end_date: MaybeLocalDate
}

impl Task {
    pub fn new(desc: &str, estimate: u16) -> Task {
        Task {
            description: desc.to_string(),
            uuid: Uuid::new_v4(),
            pomodori_count: 0,
            pomodori_estimate: estimate,
            comment: "".to_string(),
            is_ongoing: false,
            start_date: None,
            end_date: None
        }
    }

    pub fn start(&mut self) {
        if self.start_date.is_none() {
            self.start_date = Some(chrono::Local::now())
        }
        self.is_ongoing = true;
    }

    pub fn do_one_pomodoro(&mut self) {
        wait_for(25);
        self.increment_pomodoro();
        self.is_ongoing = false;
    }

    pub fn finish(&mut self) {
        self.end_date = Some(chrono::Local::now())
    }

    fn increment_pomodoro(&mut self) {
        self.pomodori_count += 1;
    }

    pub fn can_be_identified_by(&self, identifier: &str) -> bool {
        self.uuid.to_string().starts_with(identifier)
    }

    pub fn to_csv(&self) -> String {
        let start_date_string = self.start_date.map_or(String::new(), |x|x.to_rfc3339());
        let end_date_string = self.end_date.map_or(String::new(), |x| x.to_rfc3339());
        format!("{};{};{};{};{};{};{};{}\n", self.description, self.uuid,
                self.pomodori_count, self.pomodori_estimate,
                self.comment, self.is_ongoing,
                start_date_string,
                end_date_string
        )
    }

    pub fn from_csv(line: &str) -> Task {
        let task_elements = line.split(";").collect::<Vec<&str>>();
        let desc = task_elements[0];
        let uuid : Uuid = Uuid::parse_str(task_elements[1])
            .ok()
            .expect("Error in the task file : uuid not parsable.");
        let pomodori_count: u16 = task_elements[2].parse()
            .ok()
            .expect("Error in the task file : pomodori count not parsable.");
        let pomodori_estimate: u16 = task_elements[3].parse()
            .ok()
            .expect("Error in the task file : pomodori count not parsable");
        let comment = task_elements[4];
        let is_ongoing : bool = task_elements[5] == "true";
        let start_date : MaybeLocalDate = match task_elements[6].len() {
            0 => None,
            _ => Some(task_elements[6].parse::<chrono::DateTime<chrono::Local>>()
                .ok()
                .expect("Error in the task file : start date not parsable."))
        };
        let end_date : MaybeLocalDate = match task_elements[7].len() {
            0 => None,
            _ => Some(task_elements[7].parse::<chrono::DateTime<chrono::Local>>()
                .ok()
                .expect("Error in the task file : start date not parsable."))
        };
        Task {
            description: desc.to_string(),
            uuid: uuid,
            pomodori_count: pomodori_count,
            pomodori_estimate: pomodori_estimate,
            comment: comment.to_string(),
            is_ongoing: is_ongoing,
            start_date: start_date,
            end_date: end_date
        }
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{0: <38} {1:.<80} {2} / {3}",
                self.uuid.to_string(), self.description,
                self.pomodori_count, self.pomodori_estimate)
    }
}
