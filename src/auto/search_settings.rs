// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
    #[doc(alias = "GtkSourceSearchSettings")]
    pub struct SearchSettings(Object<ffi::GtkSourceSearchSettings, ffi::GtkSourceSearchSettingsClass>);

    match fn {
        type_ => || ffi::gtk_source_search_settings_get_type(),
    }
}

impl SearchSettings {
    pub const NONE: Option<&'static SearchSettings> = None;

    #[doc(alias = "gtk_source_search_settings_new")]
    pub fn new() -> SearchSettings {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_search_settings_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SearchSettings`] objects.
    ///
    /// This method returns an instance of [`SearchSettingsBuilder`](crate::builders::SearchSettingsBuilder) which can be used to create [`SearchSettings`] objects.
    pub fn builder() -> SearchSettingsBuilder {
        SearchSettingsBuilder::default()
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SearchSettings`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchSettingsBuilder {
    at_word_boundaries: Option<bool>,
    case_sensitive: Option<bool>,
    regex_enabled: Option<bool>,
    search_text: Option<String>,
    wrap_around: Option<bool>,
}

impl SearchSettingsBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SearchSettingsBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchSettings`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchSettings {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref at_word_boundaries) = self.at_word_boundaries {
            properties.push(("at-word-boundaries", at_word_boundaries));
        }
        if let Some(ref case_sensitive) = self.case_sensitive {
            properties.push(("case-sensitive", case_sensitive));
        }
        if let Some(ref regex_enabled) = self.regex_enabled {
            properties.push(("regex-enabled", regex_enabled));
        }
        if let Some(ref search_text) = self.search_text {
            properties.push(("search-text", search_text));
        }
        if let Some(ref wrap_around) = self.wrap_around {
            properties.push(("wrap-around", wrap_around));
        }
        glib::Object::new::<SearchSettings>(&properties)
            .expect("Failed to create an instance of SearchSettings")
    }

    pub fn at_word_boundaries(mut self, at_word_boundaries: bool) -> Self {
        self.at_word_boundaries = Some(at_word_boundaries);
        self
    }

    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    pub fn regex_enabled(mut self, regex_enabled: bool) -> Self {
        self.regex_enabled = Some(regex_enabled);
        self
    }

    pub fn search_text(mut self, search_text: &str) -> Self {
        self.search_text = Some(search_text.to_string());
        self
    }

    pub fn wrap_around(mut self, wrap_around: bool) -> Self {
        self.wrap_around = Some(wrap_around);
        self
    }
}

pub trait SearchSettingsExt: 'static {
    #[doc(alias = "gtk_source_search_settings_get_at_word_boundaries")]
    #[doc(alias = "get_at_word_boundaries")]
    fn is_at_word_boundaries(&self) -> bool;

    #[doc(alias = "gtk_source_search_settings_get_case_sensitive")]
    #[doc(alias = "get_case_sensitive")]
    fn is_case_sensitive(&self) -> bool;

    #[doc(alias = "gtk_source_search_settings_get_regex_enabled")]
    #[doc(alias = "get_regex_enabled")]
    fn is_regex_enabled(&self) -> bool;

    #[doc(alias = "gtk_source_search_settings_get_search_text")]
    #[doc(alias = "get_search_text")]
    fn search_text(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_search_settings_get_wrap_around")]
    #[doc(alias = "get_wrap_around")]
    fn wraps_around(&self) -> bool;

    #[doc(alias = "gtk_source_search_settings_set_at_word_boundaries")]
    fn set_at_word_boundaries(&self, at_word_boundaries: bool);

    #[doc(alias = "gtk_source_search_settings_set_case_sensitive")]
    fn set_case_sensitive(&self, case_sensitive: bool);

    #[doc(alias = "gtk_source_search_settings_set_regex_enabled")]
    fn set_regex_enabled(&self, regex_enabled: bool);

    #[doc(alias = "gtk_source_search_settings_set_search_text")]
    fn set_search_text(&self, search_text: Option<&str>);

    #[doc(alias = "gtk_source_search_settings_set_wrap_around")]
    fn set_wrap_around(&self, wrap_around: bool);

    #[doc(alias = "at-word-boundaries")]
    fn connect_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "case-sensitive")]
    fn connect_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "regex-enabled")]
    fn connect_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "search-text")]
    fn connect_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "wrap-around")]
    fn connect_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchSettings>> SearchSettingsExt for O {
    fn is_at_word_boundaries(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_at_word_boundaries(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_case_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_case_sensitive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_regex_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_regex_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn search_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_settings_get_search_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn wraps_around(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_wrap_around(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_at_word_boundaries(&self, at_word_boundaries: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_at_word_boundaries(
                self.as_ref().to_glib_none().0,
                at_word_boundaries.into_glib(),
            );
        }
    }

    fn set_case_sensitive(&self, case_sensitive: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_case_sensitive(
                self.as_ref().to_glib_none().0,
                case_sensitive.into_glib(),
            );
        }
    }

    fn set_regex_enabled(&self, regex_enabled: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_regex_enabled(
                self.as_ref().to_glib_none().0,
                regex_enabled.into_glib(),
            );
        }
    }

    fn set_search_text(&self, search_text: Option<&str>) {
        unsafe {
            ffi::gtk_source_search_settings_set_search_text(
                self.as_ref().to_glib_none().0,
                search_text.to_glib_none().0,
            );
        }
    }

    fn set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_wrap_around(
                self.as_ref().to_glib_none().0,
                wrap_around.into_glib(),
            );
        }
    }

    fn connect_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_at_word_boundaries_trampoline<
            P: IsA<SearchSettings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchSettings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::at-word-boundaries\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_at_word_boundaries_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_case_sensitive_trampoline<
            P: IsA<SearchSettings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchSettings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::case-sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_case_sensitive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_regex_enabled_trampoline<
            P: IsA<SearchSettings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchSettings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::regex-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_regex_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_text_trampoline<
            P: IsA<SearchSettings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchSettings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_around_trampoline<
            P: IsA<SearchSettings>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchSettings,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchSettings::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap-around\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_around_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SearchSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SearchSettings")
    }
}
