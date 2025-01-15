#[allow(unused)]
pub mod cb {
    use std::{
        fmt::Display,
        io::{Cursor, Read, Write},
        string::FromUtf8Error,
    };

    // struct, enum, or trait ???
    // pub enum ClipboardOp { Get, Set(String), Append(String), Prepend(String) } // ?

    pub trait ClipboardRead {
        fn get_cb(&self, reg: char) -> Result<String, FromUtf8Error>;
    }

    pub trait ClipboardWrite {
        fn set_cb(&self, reg: char, message: String) -> usize;
        // fn append_cb(&self, reg: char, message: String) -> usize;
        // fn clear_cb(&self, reg: char) -> usize;
    }

    // fn append_to_cb<T: Read + Write>(&self, reg: char, message: String) -> usize;
    // fn prepend_to_cb<T: Read + Write>(&self, reg: char, message: String) -> usize;

    impl<T: Read> ClipboardRead for T {
        fn get_cb(&self, reg: char) -> Result<String, FromUtf8Error> {
            let mut buffer = Vec::new();
            {
                let mut writer = Cursor::new(&mut buffer);
                write!(writer, "\x1b]52;{reg};?m\0");
            }

            // TODO : decode contents from base64
            // let contents: String = if let Ok(i) = String::from_utf8(buffer) { ... }
            String::from_utf8(buffer)
        }
    }
    impl<T: Write> ClipboardWrite for T {
        fn set_cb(&self, reg: char, message: String) -> usize {
            let encoded_message = message; // TODO? : encode to base64

            let mut buffer = Vec::new();
            {
                let mut writer = Cursor::new(&mut buffer);
                write!(writer, "\x1b]52;{reg}m{encoded_message}\0");
            }

            match String::from_utf8(buffer) {
                Ok(_) => return 0,
                _ => return 1,
            }
        }
    }

    /*
    pub fn get_cb(reg: Option<char>) -> Result<String, FromUtf8Error> {
        return send_osc_52(reg, "")?;

        // self.write("\x1b]52;p;?\0")
        // self.read(b"\x1b]52;p;?").unwrap_or(1)
    }
    */
}
