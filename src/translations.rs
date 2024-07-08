use std::borrow::Cow;

pub fn translate(tag: &str) -> Cow<str> {
    rust_i18n::t!(tag, pwd = std::env::current_dir().unwrap().display())
}
