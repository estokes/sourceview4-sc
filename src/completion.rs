// completion.rs
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

use crate::Completion;
use crate::CompletionContext;
use glib::translate::*;

impl Completion {
    #[allow(unused)]
    fn create_context(
        &self,
        mut position: Option<&mut gtk::TextIter>,
    ) -> Option<CompletionContext> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_create_context(
                self.to_glib_none().0,
                position.to_glib_none_mut().0,
            ))
        }
    }
}
