extern crate notify;

use notify::watcher;
use std::time::Duration;
use std::sync::mpsc::channel;

pub struct WatchArgs<'a> {
	path: &'a str,
	delay: Duration,
	callback: Box<dyn FnMut(notify::DebouncedEvent)>,
	on_watch_error: Box<dyn Fn(std::sync::mpsc::RecvError)>,
}

impl WatchArgs<'_> {
	pub fn default() -> Self {
		WatchArgs {
			path: ".",
			delay: Duration::from_millis(100),
			callback: Box::new(|_| {}),
			on_watch_error: Box::new(|_| {}),
		}
	}
	
	pub fn new(path: &str, delay: Duration, callback: Box<dyn FnMut(notify::DebouncedEvent)>, on_watch_error: Box<dyn Fn(std::sync::mpsc::RecvError)>) -> WatchArgs<'_> {
		WatchArgs {
			path,
			delay,
			callback,
			on_watch_error,
		}
	}
	
	pub fn with_path(self, path: &str) -> WatchArgs {
		WatchArgs {
			path,
			delay: self.delay,
			callback: self.callback,
			on_watch_error: self.on_watch_error,
		}
	}
	
	pub fn with_delay(mut self, delay: Duration) -> Self {
		self.delay = delay;
		self
	}
	
	pub fn with_callback<F>(mut self, callback: F) -> Self
	where F: FnMut(notify::DebouncedEvent) + 'static {
		self.callback = Box::new(callback);
		self
	}
	
	pub fn with_on_watch_error<F>(mut self, on_watch_error: F) -> Self
	where F: Fn(std::sync::mpsc::RecvError) + 'static {
		self.on_watch_error = Box::new(on_watch_error);
		self
	}
}

pub fn watch(mut args: WatchArgs) -> std::result::Result<(), notify::Error> {
	use notify::{RecursiveMode, Watcher};

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, args.delay).unwrap();
	let result = watcher.watch(args.path, RecursiveMode::Recursive);
	
	if result.is_err() {
		return result;
	}

	loop {
		match rx.recv() {
			Ok(event) => &(args.callback)(event),
			Err(e) => &(args.on_watch_error)(e),
		};
	}
}