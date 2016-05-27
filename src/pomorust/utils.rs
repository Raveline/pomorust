use std::fs::File;
use std::env;
use std::thread;
use std::time::Duration;
use std::process;
use std::os::unix::process::CommandExt;
use std::io::BufReader;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use chrono;
use rodio;

pub type MaybeLocalDate = Option<chrono::DateTime<chrono::Local>>;

pub fn wait_for(minutes: u16) {
    thread::sleep(Duration::new(minutes as u64 * 60, 0));
}

/// Try to parse a date if a string is not empty. Fail if the date
/// is not correct. Return None if the string is empty.
pub fn parse_maybe_local_date(str: &str, err_str: &str) -> MaybeLocalDate {
    match str.len() {
        0 => None,
        _ => Some(str.parse::<chrono::DateTime<chrono::Local>>()
            .ok()
            .expect(err_str))
    }
}

pub fn parse_maybe_string(str: &str) -> Option<String> {
    match str.len() {
        0 => None,
        _ => Some(str.to_string())
    }
}

pub fn ding() {
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);
    let path = concat!(env!("CARGO_MANIFEST_DIR"),
            "/data/ding.ogg");
    let file = File::open(path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    /* Unfortunately, Rodio sleep until end is not working yet;
    So we will just sleep for one second, time enough for the sound
    to be played.*/
    thread::sleep(Duration::new(1, 0));
}

pub fn notify(title: &str, text: &str) {
    Notification::new().summary(title)
        .body(text)
        .hint(Hint::SuppressSound(true))
        .show().unwrap();
}

/// We want to leave the shell available once the pomodoro has started.
/// So we need to launch the timer as an independant process.
/// This function is the shortcut to do this.
/// Rust process library is relatively unstable at this point,
/// so we want to encapsulate this.
pub fn run_background_process(task_id: String) {
	process::Command::new(env::args().nth(0).expect("Should not happen"))
		.arg("new_pomodoro")
        .arg(task_id)
        .before_exec(|| { Ok(()) }).spawn().unwrap();
		//.session_leader(true).spawn().unwrap();
}

pub fn str_to(str: &str, up_to: usize) -> String {
    if up_to > str.len() - 1 {
        str.to_string()
    } else {
        str[..up_to].to_string()
    }
}
