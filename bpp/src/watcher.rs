use crate::utils::is_bpp;
use log::{error, info, warn};
use notify::{recommended_watcher, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{env, process, thread, time::Duration};

pub fn main() {
    info!("Listening for file changes");

    let handle = thread::spawn(|| loop {
        let mut watcher: RecommendedWatcher =
            match recommended_watcher(|res: Result<Event, Error>| match res {
                Ok(event) => {
                    let first_path = match event.paths.first() {
                        Some(first_path) => first_path,
                        None => {
                            error!("I cannot find the first path");
                            process::exit(0)
                        }
                    };

                    let path_str = match first_path.to_str() {
                        Some(path_str) => path_str,
                        None => {
                            error!("I can't turn the first path into a 'str'");
                            process::exit(0)
                        }
                    };

                    if is_bpp(path_str) {
                        warn!("A change in a bpp file has been detected: {:?}", event)
                    }
                }
                Err(e) => info!("watch error: {:?}", e),
            }) {
                Ok(watcher) => watcher,
                Err(error) => {
                    error!("{}", error);
                    process::exit(0)
                }
            };

        let current_dir = match env::current_dir() {
            Ok(current_dir) => current_dir,
            Err(error) => {
                error!("{}", error);
                process::exit(0)
            }
        };

        match watcher.watch(&current_dir, RecursiveMode::Recursive) {
            Ok(_) => {}
            Err(error) => {
                error!("{}", error);
                process::exit(0)
            }
        };

        thread::sleep(Duration::from_secs(10));
    });

    match handle.join() {
        Ok(_) => {}
        Err(error) => {
            error!("{:?}", error);
            process::exit(0)
        }
    }
}
