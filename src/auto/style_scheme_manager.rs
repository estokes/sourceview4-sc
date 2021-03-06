// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleScheme;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSourceStyleSchemeManager")]
    pub struct StyleSchemeManager(Object<ffi::GtkSourceStyleSchemeManager, ffi::GtkSourceStyleSchemeManagerClass>);

    match fn {
        type_ => || ffi::gtk_source_style_scheme_manager_get_type(),
    }
}

impl StyleSchemeManager {
    pub const NONE: Option<&'static StyleSchemeManager> = None;

    #[doc(alias = "gtk_source_style_scheme_manager_new")]
    pub fn new() -> StyleSchemeManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_style_scheme_manager_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StyleSchemeManager`] objects.
    ///
    /// This method returns an instance of [`StyleSchemeManagerBuilder`](crate::builders::StyleSchemeManagerBuilder) which can be used to create [`StyleSchemeManager`] objects.
    pub fn builder() -> StyleSchemeManagerBuilder {
        StyleSchemeManagerBuilder::default()
    }

    #[doc(alias = "gtk_source_style_scheme_manager_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<StyleSchemeManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_style_scheme_manager_get_default()) }
    }
}

impl Default for StyleSchemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StyleSchemeManager`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StyleSchemeManagerBuilder {
    search_path: Option<Vec<String>>,
}

impl StyleSchemeManagerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StyleSchemeManagerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StyleSchemeManager`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StyleSchemeManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref search_path) = self.search_path {
            properties.push(("search-path", search_path));
        }
        glib::Object::new::<StyleSchemeManager>(&properties)
            .expect("Failed to create an instance of StyleSchemeManager")
    }

    pub fn search_path(mut self, search_path: Vec<String>) -> Self {
        self.search_path = Some(search_path);
        self
    }
}

pub trait StyleSchemeManagerExt: 'static {
    #[doc(alias = "gtk_source_style_scheme_manager_append_search_path")]
    fn append_search_path(&self, path: &str);

    #[doc(alias = "gtk_source_style_scheme_manager_force_rescan")]
    fn force_rescan(&self);

    #[doc(alias = "gtk_source_style_scheme_manager_get_scheme")]
    #[doc(alias = "get_scheme")]
    fn scheme(&self, scheme_id: &str) -> Option<StyleScheme>;

    #[doc(alias = "gtk_source_style_scheme_manager_get_scheme_ids")]
    #[doc(alias = "get_scheme_ids")]
    fn scheme_ids(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_manager_get_search_path")]
    #[doc(alias = "get_search_path")]
    fn search_path(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_source_style_scheme_manager_prepend_search_path")]
    fn prepend_search_path(&self, path: &str);

    #[doc(alias = "gtk_source_style_scheme_manager_set_search_path")]
    fn set_search_path(&self, path: &[&str]);

    #[doc(alias = "scheme-ids")]
    fn connect_scheme_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "search-path")]
    fn connect_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleSchemeManager>> StyleSchemeManagerExt for O {
    fn append_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_append_search_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    fn force_rescan(&self) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_force_rescan(self.as_ref().to_glib_none().0);
        }
    }

    fn scheme(&self, scheme_id: &str) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme(
                self.as_ref().to_glib_none().0,
                scheme_id.to_glib_none().0,
            ))
        }
    }

    fn scheme_ids(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gtk_source_style_scheme_manager_get_scheme_ids(self.as_ref().to_glib_none().0),
            )
        }
    }

    fn search_path(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gtk_source_style_scheme_manager_get_search_path(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn prepend_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_prepend_search_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    fn set_search_path(&self, path: &[&str]) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_set_search_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    fn connect_scheme_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scheme_ids_trampoline<
            P: IsA<StyleSchemeManager>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceStyleSchemeManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleSchemeManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scheme-ids\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scheme_ids_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_path_trampoline<
            P: IsA<StyleSchemeManager>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceStyleSchemeManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleSchemeManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleSchemeManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleSchemeManager")
    }
}
