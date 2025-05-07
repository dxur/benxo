use core::fmt;

pub struct Error {
    pub msg: String,
}

impl From<()> for Error {
    fn from(value: ()) -> Self {
        Error {
            msg: "Unknown".to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error {
            msg: value,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error").field("msg", &self.msg).finish()
    }
}
