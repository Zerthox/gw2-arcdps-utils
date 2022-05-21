//! Window component.

mod menu;
mod options;
mod render;

#[cfg(feature = "settings")]
mod settings;

use crate::ui::{
    render::{small_padding, window_context_menu},
    Component, Hideable, Ui, Windowable,
};
use std::ops::{Deref, DerefMut};

pub use menu::*;
pub use options::*;
pub use render::*;

#[cfg(feature = "settings")]
pub use settings::*;

/// Window component.
#[derive(Debug, Clone)]
pub struct Window<T>
where
    T: Windowable,
{
    pub options: WindowOptions,
    pub inner: T,
}

impl<T> Window<T>
where
    T: Windowable,
{
    /// Creates a new window with [`WindowOptions`] and a given inner [`Windowable`] component.
    pub fn new(options: WindowOptions, inner: T) -> Self {
        Self { inner, options }
    }
}

impl<T> Deref for Window<T>
where
    T: Windowable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Window<T>
where
    T: Windowable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Component for Window<T>
where
    T: Windowable,
{
    type Props = T::Props;

    fn render(&mut self, ui: &Ui, props: &Self::Props) {
        if let Some(_window) = render_window(ui, &mut self.options) {
            // read current window size values
            [self.options.width, self.options.height] = ui.window_size();

            if T::CONTEXT_MENU {
                // render context menu
                window_context_menu(&format!("Options##{}", self.options.name), || {
                    let _style = small_padding(ui);

                    self.inner.render_menu(ui, props);
                    if T::DEFAULT_OPTIONS {
                        window_options_menus(ui, &mut self.options);
                    }
                });
            }

            // render window content
            self.inner.render(ui, props);
        }
    }
}

impl<T> Hideable for Window<T>
where
    T: Windowable,
{
    fn is_visible(&self) -> bool {
        self.options.visible
    }

    fn visible_mut(&mut self) -> &mut bool {
        &mut self.options.visible
    }
}
