use std::boxed::Box;

pub struct SimpleCallback {
    pub callback: Box<dyn FnOnce()>
}

impl SimpleCallback {

    pub fn process(self) {
        (self.callback)()
    }
}
