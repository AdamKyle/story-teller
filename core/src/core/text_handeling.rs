/// Unwraps the option possibly containing a string.
///
/// Can return an empty string.
pub fn unwrap_str<'a>(w: Option<&'a String>) -> &'a str {
    match w {
        Some(s) => s.as_str(),
        None    => ""
    }
}
