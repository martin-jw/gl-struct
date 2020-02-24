mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<crate::bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
    where
        F: FnMut(&'static str) -> *const crate::types::GLvoid,
    {
        Gl {
            inner: Rc::new(crate::bindings::Gl::load_with(loadfn)),
        }
    }
}

impl Deref for Gl {
    type Target = crate::bindings::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
