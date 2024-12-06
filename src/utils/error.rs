// static mut DEBUG: bool = false;

// pub fn enable_debugging() -> () {
// 	unsafe {
// 		DEBUG = true;
// 	}
// }

// pub fn get_debugging() -> bool {
// 	unsafe {
// 		return DEBUG;
// 	}
// }

// macro_rules! debug {
// 	($msg:expr) => {{
// 		if crate::utils::get_debugging() {
// 			println!("{} ({}:{})", $msg, file!(), line!());
// 		}
// 	}};
// }
// pub(crate) use debug;

pub type AnyError = Box<dyn std::error::Error>;

macro_rules! guard {
	($expr:expr, $msg:expr) => {
		match $expr {
			Ok(val) => val,
			Err(e) => {
				eprintln!("[{}:{}] - {}", file!(), line!(), $msg);
				return Err(e.into());
			}
		}
	};

	($expr:expr, $msg:expr, $($arg:tt)*) => {{
		match $expr {
			Ok(val) => val,
			Err(e) => {
				let formatted_msg: String = format!($msg, $($arg)*);
				eprintln!("[{}:{}] - {}", file!(), line!(), formatted_msg);
				return Err(e.into());
			}
		}
	}};
}

// macro_rules! error {
// 	($msg:expr) => {
// 		Err(format!($msg).into())
// 	};
// }

macro_rules! error {
    ($msg:expr) => ({
        Err(($msg).into())
    });
    ($msg:expr, $($arg:tt)*) => {{
        let formatted_msg: String = format!($msg, $($arg)*);
        Err(formatted_msg.into())
    }};
}

pub(crate) use error;
pub(crate) use guard;
