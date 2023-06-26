use std::rc::Rc;

use i_slint_core::{platform::PlatformError, window::WindowAdapter};

mod baseviewwindowadapter;
mod event_loop;
mod glcontext;
mod renderer;

use baseviewwindowadapter::BaseviewWindowAdapter;

pub struct Backend {
    window_factory_fn: fn() -> Result<Rc<dyn WindowAdapter>, PlatformError>,
}

fn window_factory_fn() -> Result<Rc<dyn WindowAdapter>, PlatformError> {
    BaseviewWindowAdapter::new()
}

impl Backend {
    #[doc = concat!("Creates a new baseview backend with the femtovg renderer that's compiled in. See the [backend documentation](https://slint.dev/releases/", env!("CARGO_PKG_VERSION"), "/docs/rust/slint/index.html#backends) for")]
    pub fn new() -> Self {
        Self { window_factory_fn: window_factory_fn }
    }
}

impl i_slint_core::platform::Platform for Backend {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        (self.window_factory_fn)().or_else(|e| {
            if let Ok(window) = window_factory_fn() {
                return Ok(window);
            }
            Err(format!(
                "Baseview backend failed to find a suitable renderer. Last failure was: {e}"
            )
            .into())
        })
    }

    // TODO
    // fn run_event_loop(&self) -> Result<(), PlatformError> {
    // crate::event_loop::run()
    // }
}
