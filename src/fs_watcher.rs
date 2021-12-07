extern crate notify;

use notify::watcher;
use std::time::Duration;
use std::sync::mpsc::channel;

pub fn watch<F, G>(path: &str, delay: Duration, mut callback: F, on_watch_error: G) -> std::result::Result<(), notify::Error> where
	F: FnMut(notify::DebouncedEvent),
	G: Fn(std::sync::mpsc::RecvError) {
	
	use notify::{RecursiveMode, Watcher};

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, delay).unwrap();
	let result = watcher.watch(path, RecursiveMode::Recursive);
	
	if result.is_err() {
		return result;
	}

	loop {
		match rx.recv() {
			Ok(event) => callback(event),
			Err(e) => on_watch_error(e),
		}
	}
}