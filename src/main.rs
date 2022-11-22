use notify_rust::Notification;
use std::convert::AsRef;
use std::io::prelude::Write;
use std::{env, fs, fs::File, str::FromStr, error::Error, path::PathBuf, collections::HashSet};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumString, EnumIter};

fn main() -> Result<(), Box<dyn Error>> {
    let posture_cache_file: PathBuf = dirs::cache_dir()
        .expect("Cannot find user's cache_dir")
        .join("posture");

    if !posture_cache_file.exists() {
        File::create(&posture_cache_file)?;
    }

    let mut current_state = fs::read_to_string(&posture_cache_file)?;

    let mut args = env::args();
    // if there are args its for resetting the state
    if args.len() > 1 {
        let arg = args.nth(1).unwrap();
        if arg == "reset" {
            fs::remove_file(posture_cache_file)?;
        } else if arg == "status" {
            println!("Current posture status = {}", current_state);
        } else {
            panic!("unknown argument {:?}", arg)
        }

        return Ok(());
    }

    let mut postures: HashSet<String> = HashSet::new();
    PostureStatuses::iter().for_each(|p| {
        postures.insert(p.as_ref().into());
    });

    // re "create" the exisiting file to blow away existing data
    let mut file = File::create(&posture_cache_file)?;

    // if the file does not contain a valid posture just make the current state
    // PostureStatuses::BREAK so it can advance from there
    if !postures.contains(&current_state) {
        current_state = PostureStatuses::Sitting.as_ref().to_string();
    }

    let next_state = match PostureStatuses::from_str(&current_state)? {
        PostureStatuses::Sitting => PostureStatuses::Standing,
        PostureStatuses::Standing => PostureStatuses::Break,
        PostureStatuses::Break => PostureStatuses::Sitting,
    };
    file.write_all(format!("{:?}", next_state).as_bytes())?;

    Notification::new()
        .summary("Posture Watcher")
        .urgency(notify_rust::Urgency::Critical)
        .body(&format!(
            "Time to change to a {} position",
            next_state.as_ref()
        ))
        .show()?;

    Ok(())
}

/// The progression of Posture statuses to loop through
/// Assumes I start in SITTING
#[derive(AsRefStr, Debug, EnumIter, EnumString, PartialEq)]
enum PostureStatuses {
    #[strum(serialize = "Sitting")]
    Sitting,
    #[strum(serialize = "Standing")]
    Standing,
    #[strum(serialize = "Break")]
    Break,
}
