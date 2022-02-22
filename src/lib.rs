// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GtkSourceView may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using GtkSourceView.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub mod prelude;
pub mod subclass;

#[allow(unused_imports)]
mod auto;
pub use auto::*;

mod completion;
mod completion_info;
mod gutter_renderer;
mod search_context;
mod view;
