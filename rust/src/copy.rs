extern crate cli_clipboard;

use std::thread::sleep;
use std::time::Duration;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

// pub fn sleep_sec(sec: &u64) {
// 	let dur = Duration::from_secs(*sec);
// 	sleep(dur);
// }

pub fn sleep_milli(ms: &u64) {
	let dur = Duration::from_millis(*ms);
	sleep(dur);
}

pub fn copy_to_clipboard(str: String, keep_time: &u64) {
	let mut ctx = ClipboardContext::new().unwrap();
	ctx.set_contents(str.clone()).unwrap();
	//should check the completion and sleep until finish, here I sleep a fixed duration.
	sleep_milli(keep_time)
}