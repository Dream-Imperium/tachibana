use skulpin_renderer::{CoordinateSystem, LogicalSize, PresentMode, RendererBuilder};

use super::{runner::Runner, Game};

pub struct Builder<'a> {
    inner_size: LogicalSize,
    window_title: &'a str,
    renderer_builder: RendererBuilder,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Builder::new()
    }
}

impl<'a> Builder<'a> {
    /// Construct the app builder initialized with default options
    pub fn new() -> Self {
        Self {
            inner_size: LogicalSize::new(1280, 720).into(),
            window_title: "Tachibana",
            renderer_builder: RendererBuilder::new().use_vulkan_debug_layer(false),
        }
    }

    /// Specifies the inner size of the window. Both physical and logical coordinates are accepted.
    pub fn inner_size(mut self, inner_size: LogicalSize) -> Self {
        self.inner_size = inner_size.into();
        self
    }

    /// Specifies the title that the window will be created with
    pub fn window_title(mut self, title: &'a str) -> Self {
        self.window_title = title.as_ref();
        self
    }

    /// Name of the app. This is passed into the vulkan layer. I believe it can hint things to the
    /// vulkan driver, but it's unlikely this makes a real difference. Still a good idea to set this
    /// to something meaningful though.
    pub fn app_name(mut self, app_name: std::ffi::CString) -> Self {
        self.renderer_builder = self.renderer_builder.app_name(app_name);
        self
    }

    /// Determine the coordinate system to use for the canvas. This can be overridden by using the
    /// canvas sizer passed into the draw callback
    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.renderer_builder = self.renderer_builder.coordinate_system(coordinate_system);
        self
    }

    /// Start the app.
    pub fn run<F, T>(self, game: F)
    where
        F: 'static + Send + FnOnce() -> T,
        T: Game,
    {
        Runner::run(
            game,
            self.inner_size,
            self.window_title,
            self.renderer_builder
                .present_mode_priority(vec![PresentMode::Immediate]),
        )
    }
}
