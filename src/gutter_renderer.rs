// gutter_renderer.rs
//
// Copyright 2019 Christopher Davis <brainblasted@disroot.org>
//
// This file is free software; you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use crate::GutterRenderer;
use crate::GutterRendererState;
use glib::object::IsA;
use glib::translate::*;

pub trait GutterRendererExtManual: 'static {
    fn begin(
        &self,
        cr: &mut cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    );
    fn draw(
        &self,
        cr: &mut cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    );
}

impl<O: IsA<GutterRenderer>> GutterRendererExtManual for O {
    fn begin(
        &self,
        cr: &mut cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    ) {
        unsafe {
            ffi::gtk_source_gutter_renderer_begin(
                self.as_ref().to_glib_none().0,
                cr.to_raw_none(),
                background_area.to_glib_none_mut().0,
                cell_area.to_glib_none_mut().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn draw(
        &self,
        cr: &mut cairo::Context,
        background_area: &mut gdk::Rectangle,
        cell_area: &mut gdk::Rectangle,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        state: GutterRendererState,
    ) {
        unsafe {
            ffi::gtk_source_gutter_renderer_draw(
                self.as_ref().to_glib_none().0,
                cr.to_raw_none(),
                background_area.to_glib_none_mut().0,
                cell_area.to_glib_none_mut().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                state.into_glib(),
            );
        }
    }
}
