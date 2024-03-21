use signal_hook::consts::SIGHUP;
use signal_hook::consts::SIGINT;
use signal_hook::consts::SIGQUIT;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use std::{thread, time::Duration};

pub fn start() -> String {
    use std::collections::HashMap;
    use std::env;
    use std::process::{Command, Stdio};

    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();

    let start_front = if cfg!(target_os = "windows") {
        Command::new("tejmagar-index")
            .args(["/C", ""])
            //.current_dir("/usr/local/bin") //TODO
            .stdout(Stdio::inherit())
            .env_clear()
            .envs(&filtered_env)
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("tejmagar-index")
            .arg("")
            .current_dir("/usr/local/bin")
            .stdout(Stdio::inherit())
            .env_clear()
            .envs(&filtered_env)
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("tejmagar-index")
            .arg("")
            .current_dir("/usr/local/bin")
            .stdout(Stdio::inherit())
            .env_clear()
            .envs(&filtered_env)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("tejmagar-index")
            .arg("")
            .current_dir("/usr/local/bin")
            .stdout(Stdio::inherit())
            .env_clear()
            .envs(&filtered_env)
            .output()
            .expect("failed to execute process")
    };

    let start_front = String::from_utf8(start_front.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    return start_front;
}

pub fn process_daemon() {
    let mut signals = match Signals::new(&[SIGHUP, SIGINT, SIGQUIT, SIGTERM]) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    thread::spawn(move || {
        for _sig in signals.forever() {
            let _res = start();
            //println!("{}:{:?}:{}",std::process::id(), _sig,res);
            //println!("{}:{}", std::process::id(), _res);
        }
    });

    thread::sleep(Duration::from_secs(2));
}

fn main() {
    loop {
        process_daemon();
    }
}
