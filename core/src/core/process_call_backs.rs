use std::boxed::Box;

/// Simple callback struct that allows a function thats only called once.
///
/// # Example
///
/// ```
/// use core::process_call_backs::SimpleCallback
/// use std::boxed::Box;
///
/// fn sample() {
///     println!("hello!");
/// }
///
/// fn main() {
///     let mut callback: SimpleCallback = SimpleCallback {
///        callback: Box::new(|| sample)
///     };
///
///     callback.process(); // => hello!
/// }
///
/// Functions can take arguments when passed in this way. We defer the value till the process
/// is called.
///
/// This is great for passing functions around that need to be called
/// in different modules or sections of code.
pub struct SimpleCallback {
    pub callback: Box<dyn FnOnce()>
}

/// The implementation of Simple Callback
impl SimpleCallback {

    /// Process the callback. Simply calls the callback.
    pub fn process(self) {
        (self.callback)();
    }
}
