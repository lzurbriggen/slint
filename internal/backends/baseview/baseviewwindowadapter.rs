use std::{cell::Cell, pin::Pin, rc::Rc};

use baseview::{
    gl::GlConfig, Event, EventStatus, MouseEvent, Window, WindowHandle, WindowHandler,
    WindowOpenOptions, WindowScalePolicy,
};
use i_slint_core::{
    api::PhysicalSize,
    graphics::euclid::default,
    platform::PlatformError,
    window::{WindowAdapter, WindowAdapterInternal},
};
use i_slint_renderer_femtovg::FemtoVGRenderer;
use once_cell::unsync::OnceCell;
use rtrb::{Consumer, RingBuffer};

use crate::glcontext::OpenGLContext;

pub struct BaseviewWindowAdapter {
    window: OnceCell<i_slint_core::api::Window>,
    // currently_pressed_key_code: std::cell::Cell<Option<winit::event::VirtualKeyCode>>,
    pending_redraw: Cell<bool>,
    // dark_color_scheme: OnceCell<Pin<Box<Property<bool>>>>,
    constraints: Cell<(i_slint_core::layout::LayoutInfo, i_slint_core::layout::LayoutInfo)>,
    shown: Cell<bool>,

    // winit_window: Rc<winit::window::Window>,
    renderer: Box<FemtoVGRenderer>,
    // We cache the size because winit_window.inner_size() can return different value between calls (eg, on X11)
    // And we wan see the newer value before the Resized event was received, leading to inconsistencies
    size: Cell<PhysicalSize>,

    #[cfg(enable_accesskit)]
    pub accesskit_adapter: crate::accesskit::AccessKitAdapter,
}

// TODO: do i really need this?
#[derive(Debug, Clone)]
enum Message {
    Hello,
}

struct SlintWindowHandler {
    rx: Consumer<Message>,
}

impl WindowHandler for SlintWindowHandler {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        while let Ok(message) = self.rx.pop() {
            println!("Message: {:?}", message);
        }
    }

    fn on_event(&mut self, _window: &mut baseview::Window, event: Event) -> EventStatus {
        match event {
            Event::Mouse(e) => {
                println!("Mouse event: {:?}", e);

                // #[cfg(target_os = "macos")]
                // match e {
                //     MouseEvent::ButtonPressed { button, modifiers } => {
                //         copy_to_clipboard(&"This is a test!")
                //     }
                //     _ => (),
                // }
            }
            Event::Keyboard(e) => println!("Keyboard event: {:?}", e),
            Event::Window(e) => println!("Window event: {:?}", e),
        }

        EventStatus::Captured
    }
}

pub(crate) struct SlintWindow {
    // application: ApplicationRunner,
    // on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
}

impl WindowHandler for SlintWindow {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        // while let Ok(message) = self.rx.pop() {
        //     println!("Message: {:?}", message);
        // }
    }

    fn on_event(&mut self, _window: &mut baseview::Window, event: Event) -> EventStatus {
        match event {
            Event::Mouse(e) => {
                println!("Mouse event: {:?}", e);

                // #[cfg(target_os = "macos")]
                // match e {
                //     MouseEvent::ButtonPressed { button, modifiers } => {
                //         copy_to_clipboard(&"This is a test!")
                //     }
                //     _ => (),
                // }
            }
            Event::Keyboard(e) => println!("Keyboard event: {:?}", e),
            Event::Window(e) => println!("Window event: {:?}", e),
        }

        EventStatus::Captured
    }
}

fn load_renderer(window: &baseview::Window) -> FemtoVGRenderer {
    let context = window.gl_context().expect("Window was created without OpenGL support");

    unsafe { context.make_current() };

    let renderer = unsafe {
        FemtoVGRenderer::new(OpenGLContext::new_context(window).unwrap().1)
        // ::new_from_function(|s| context.get_proc_address(s) as *const _)
        //     .expect("Cannot create renderer")
    };

    unsafe { context.make_not_current() };

    renderer.unwrap()
}

impl SlintWindow {
    fn new(window: &mut baseview::Window) -> Self {
        let context = window.gl_context().expect("Window was created without OpenGL support");
        let renderer = load_renderer(window);

        SlintWindow {}
    }

    pub fn open_as_if_parented() -> WindowHandle {
        let window_open_options = baseview::WindowOpenOptions {
            title: "baseview".into(),
            size: baseview::Size::new(512.0, 512.0),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        baseview::Window::open_as_if_parented(
            window_open_options,
            move |window: &mut baseview::Window<'_>| -> SlintWindow { SlintWindow::new(window) },
        )
    }
}

impl BaseviewWindowAdapter {
    /// Creates a new reference-counted instance.
    pub(crate) fn new() -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        let window_open_options = baseview::WindowOpenOptions {
            title: "baseview".into(),
            size: baseview::Size::new(512.0, 512.0),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(GlConfig::default()),
        };

        let (mut tx, rx) = RingBuffer::new(128);

        let handle = baseview::Window::open_as_if_parented(window_open_options, |window| {
            SlintWindowHandler { rx }
        });

        // let (renderer, winit_window) =
        //     Self::window_builder().and_then(|builder| R::new(builder))?;

        // let winit_window = Rc::new(winit_window);

        // let self_rc = Rc::new_cyclic(|self_weak| Self {
        //     window: OnceCell::with_value(corelib::api::Window::new(self_weak.clone() as _)),
        //     currently_pressed_key_code: Default::default(),
        //     pending_redraw: Default::default(),
        //     dark_color_scheme: Default::default(),
        //     constraints: Default::default(),
        //     shown: Default::default(),
        //     winit_window: winit_window.clone(),
        //     size: Default::default(),
        //     renderer: Box::new(renderer),
        //     #[cfg(enable_accesskit)]
        //     accesskit_adapter: crate::accesskit::AccessKitAdapter::new(
        //         self_weak.clone(),
        //         &*winit_window,
        //     ),
        // });

        // let id = self_rc.winit_window().id();
        // crate::event_loop::register_window(id, (self_rc.clone()) as _);

        // let scale_factor = std::env::var("SLINT_SCALE_FACTOR")
        //     .ok()
        //     .and_then(|x| x.parse::<f32>().ok())
        //     .filter(|f| *f > 0.)
        //     .unwrap_or_else(|| self_rc.winit_window().scale_factor() as f32);
        // self_rc.window().dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor });

        let self_rc = Rc::new_cyclic(|self_weak| Self {
            window: OnceCell::with_value(i_slint_core::api::Window::new(self_weak.clone() as _)),
            // currently_pressed_key_code: Default::default(),
            pending_redraw: Default::default(),
            // dark_color_scheme: Default::default(),
            constraints: Default::default(),
            shown: Default::default(),
            // winit_window: winit_window.clone(),
            size: Default::default(),
            renderer: Box::new(renderer),
            #[cfg(enable_accesskit)]
            accesskit_adapter: crate::accesskit::AccessKitAdapter::new(
                self_weak.clone(),
                &*winit_window,
            ),
        });

        Ok(self_rc as _)
    }

    fn renderer(&self) -> &FemtoVGRenderer {
        self.renderer.as_ref()
    }

    // fn window_builder() -> Result<winit::window::WindowBuilder, PlatformError> {
    //     let mut window_builder =
    //         winit::window::WindowBuilder::new().with_transparent(true).with_visible(false);

    //     if std::env::var("SLINT_FULLSCREEN").is_ok() {
    //         window_builder =
    //             window_builder.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    //     }

    //     window_builder = window_builder.with_title("Slint Window".to_string());

    //     Ok(window_builder)
    // }

    pub fn take_pending_redraw(&self) -> bool {
        self.pending_redraw.take()
    }

    // pub fn currently_pressed_key_code(&self) -> &Cell<Option<winit::event::VirtualKeyCode>> {
    //     &self.currently_pressed_key_code
    // }

    /// Draw the items of the specified `component` in the given window.
    pub fn draw(&self) -> Result<bool, PlatformError> {
        if !self.shown.get() {
            return Ok(false); // caller bug, doesn't make sense to call draw() when not shown
        }

        self.pending_redraw.set(false);

        let renderer = self.renderer();
        renderer.render(self.window())?;

        Ok(self.pending_redraw.get())
    }

    // fn with_window_handle(&self, callback: &mut dyn FnMut(&winit::window::Window)) {
    //     callback(&self.winit_window());
    // }

    // pub fn winit_window(&self) -> Rc<winit::window::Window> {
    //     self.winit_window.clone()
    // }

    pub fn is_shown(&self) -> bool {
        self.shown.get()
    }

    pub fn input_method_focused(&self) -> bool {
        false
    }

    // pub fn resize_event(&self, size: winit::dpi::PhysicalSize<u32>) -> Result<(), PlatformError> {
    //     // When a window is minimized on Windows, we get a move event to an off-screen position
    //     // and a resize even with a zero size. Don't forward that, especially not to the renderer,
    //     // which might panic when trying to create a zero-sized surface.
    //     if size.width > 0 && size.height > 0 {
    //         let physical_size = physical_size_to_slint(&size);
    //         self.size.set(physical_size);
    //         let scale_factor = WindowInner::from_pub(self.window()).scale_factor();
    //         self.window().dispatch_event(WindowEvent::Resized {
    //             size: physical_size.to_logical(scale_factor),
    //         });
    //         self.renderer().resize_event(physical_size)
    //     } else {
    //         Ok(())
    //     }
    // }

    // pub fn set_dark_color_scheme(&self, dark_mode: bool) {
    //     self.dark_color_scheme
    //         .get_or_init(|| Box::pin(Property::new(false)))
    //         .as_ref()
    //         .set(dark_mode)
    // }
}

impl WindowAdapter for BaseviewWindowAdapter {
    fn window(&self) -> &i_slint_core::api::Window {
        self.window.get().unwrap()
    }

    fn renderer(&self) -> &dyn i_slint_core::renderer::Renderer {
        self.renderer().as_core_renderer()
    }

    fn position(&self) -> Option<i_slint_core::api::PhysicalPosition> {
        match self.winit_window().outer_position() {
            Ok(outer_position) => {
                Some(i_slint_core::api::PhysicalPosition::new(outer_position.x, outer_position.y))
            }
            Err(_) => None,
        }
    }

    fn set_position(&self, position: i_slint_core::api::WindowPosition) {
        self.winit_window().set_outer_position(position_to_winit(&position))
    }

    fn set_size(&self, size: i_slint_core::api::WindowSize) {
        self.winit_window().set_inner_size(window_size_to_slint(&size))
    }

    fn size(&self) -> i_slint_core::api::PhysicalSize {
        self.size.get()
    }

    fn request_redraw(&self) {
        self.pending_redraw.set(true);
        self.with_window_handle(&mut |window| window.request_redraw())
    }

    fn internal(&self, _: i_slint_core::InternalToken) -> Option<&dyn WindowAdapterInternal> {
        Some(self)
    }
}
