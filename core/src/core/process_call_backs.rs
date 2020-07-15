type Callback = fn();

/// Simple callback structure that contains a callback with no arguments.
///
/// # Example
///
/// ```
/// use core::process_call_backs::SimpleCallback;
///
/// fn test() { println!("hello world"); }
///
/// let callback = SimpleCallback { callback: test };
///
/// callback.process()
/// ```
///
pub struct SimpleCallback {
    pub callback: Callback
}

/// Implementation for the struct
impl SimpleCallback {

    /// Calls the registered callback
    pub fn process(&mut self) {
        (self.callback)();
    }
}
