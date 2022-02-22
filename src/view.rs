// view.rs
//
// Copyright 2019 Christopher Davis <brainblasted@disroot.org>
//
// This file is free software; you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
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

use glib::object::IsA;
use glib::translate::*;

use crate::MarkAttributes;
use crate::View;

pub trait ViewExtManual: 'static {
    fn get_mark_attributes(&self, category: &str, priority: &mut i32) -> Option<MarkAttributes>;
}

impl<O: IsA<View>> ViewExtManual for O {
    fn get_mark_attributes(&self, category: &str, priority: &mut i32) -> Option<MarkAttributes> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_mark_attributes(
                self.as_ref().to_glib_none().0,
                category.to_glib_none().0,
                priority,
            ))
        }
    }
}
