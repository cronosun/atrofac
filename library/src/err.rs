use std::borrow::Cow;
use winapi::_core::num::TryFromIntError;

#[derive(Debug)]
pub struct AfErr {
    msg: Cow<'static, str>,
}

impl From<String> for AfErr {
    fn from(string: String) -> Self {
        Self { msg: string.into() }
    }
}

impl From<&'static str> for AfErr {
    fn from(string: &'static str) -> Self {
        Self {
            msg: Cow::Borrowed(string),
        }
    }
}

impl From<std::num::TryFromIntError> for AfErr {
    fn from(_: TryFromIntError) -> Self {
        "Integer overflow/underflow (TryFromIntError).".into()
    }
}
