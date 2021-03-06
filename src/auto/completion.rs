// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CompletionContext;
use crate::CompletionInfo;
use crate::CompletionProvider;
use crate::View;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GtkSourceCompletion")]
    pub struct Completion(Object<ffi::GtkSourceCompletion, ffi::GtkSourceCompletionClass>) @implements gtk::Buildable;

    match fn {
        type_ => || ffi::gtk_source_completion_get_type(),
    }
}

impl Completion {
    pub const NONE: Option<&'static Completion> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Completion`] objects.
    ///
    /// This method returns an instance of [`CompletionBuilder`](crate::builders::CompletionBuilder) which can be used to create [`Completion`] objects.
    pub fn builder() -> CompletionBuilder {
        CompletionBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Completion`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CompletionBuilder {
    accelerators: Option<u32>,
    auto_complete_delay: Option<u32>,
    proposal_page_size: Option<u32>,
    provider_page_size: Option<u32>,
    remember_info_visibility: Option<bool>,
    select_on_show: Option<bool>,
    show_headers: Option<bool>,
    show_icons: Option<bool>,
    view: Option<View>,
}

impl CompletionBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CompletionBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Completion`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Completion {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accelerators) = self.accelerators {
            properties.push(("accelerators", accelerators));
        }
        if let Some(ref auto_complete_delay) = self.auto_complete_delay {
            properties.push(("auto-complete-delay", auto_complete_delay));
        }
        if let Some(ref proposal_page_size) = self.proposal_page_size {
            properties.push(("proposal-page-size", proposal_page_size));
        }
        if let Some(ref provider_page_size) = self.provider_page_size {
            properties.push(("provider-page-size", provider_page_size));
        }
        if let Some(ref remember_info_visibility) = self.remember_info_visibility {
            properties.push(("remember-info-visibility", remember_info_visibility));
        }
        if let Some(ref select_on_show) = self.select_on_show {
            properties.push(("select-on-show", select_on_show));
        }
        if let Some(ref show_headers) = self.show_headers {
            properties.push(("show-headers", show_headers));
        }
        if let Some(ref show_icons) = self.show_icons {
            properties.push(("show-icons", show_icons));
        }
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        glib::Object::new::<Completion>(&properties)
            .expect("Failed to create an instance of Completion")
    }

    pub fn accelerators(mut self, accelerators: u32) -> Self {
        self.accelerators = Some(accelerators);
        self
    }

    pub fn auto_complete_delay(mut self, auto_complete_delay: u32) -> Self {
        self.auto_complete_delay = Some(auto_complete_delay);
        self
    }

    pub fn proposal_page_size(mut self, proposal_page_size: u32) -> Self {
        self.proposal_page_size = Some(proposal_page_size);
        self
    }

    pub fn provider_page_size(mut self, provider_page_size: u32) -> Self {
        self.provider_page_size = Some(provider_page_size);
        self
    }

    pub fn remember_info_visibility(mut self, remember_info_visibility: bool) -> Self {
        self.remember_info_visibility = Some(remember_info_visibility);
        self
    }

    pub fn select_on_show(mut self, select_on_show: bool) -> Self {
        self.select_on_show = Some(select_on_show);
        self
    }

    pub fn show_headers(mut self, show_headers: bool) -> Self {
        self.show_headers = Some(show_headers);
        self
    }

    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = Some(show_icons);
        self
    }

    pub fn view(mut self, view: &impl IsA<View>) -> Self {
        self.view = Some(view.clone().upcast());
        self
    }
}

pub trait CompletionExt: 'static {
    #[doc(alias = "gtk_source_completion_add_provider")]
    fn add_provider(&self, provider: &impl IsA<CompletionProvider>) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_source_completion_block_interactive")]
    fn block_interactive(&self);

    #[doc(alias = "gtk_source_completion_get_info_window")]
    #[doc(alias = "get_info_window")]
    fn info_window(&self) -> Option<CompletionInfo>;

    #[doc(alias = "gtk_source_completion_get_providers")]
    #[doc(alias = "get_providers")]
    fn providers(&self) -> Vec<CompletionProvider>;

    #[doc(alias = "gtk_source_completion_get_view")]
    #[doc(alias = "get_view")]
    fn view(&self) -> Option<View>;

    #[doc(alias = "gtk_source_completion_hide")]
    fn hide(&self);

    #[doc(alias = "gtk_source_completion_remove_provider")]
    fn remove_provider(&self, provider: &impl IsA<CompletionProvider>) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_source_completion_start")]
    fn start(
        &self,
        providers: &[CompletionProvider],
        context: &impl IsA<CompletionContext>,
    ) -> bool;

    #[doc(alias = "gtk_source_completion_unblock_interactive")]
    fn unblock_interactive(&self);

    fn accelerators(&self) -> u32;

    fn set_accelerators(&self, accelerators: u32);

    #[doc(alias = "auto-complete-delay")]
    fn auto_complete_delay(&self) -> u32;

    #[doc(alias = "auto-complete-delay")]
    fn set_auto_complete_delay(&self, auto_complete_delay: u32);

    #[doc(alias = "proposal-page-size")]
    fn proposal_page_size(&self) -> u32;

    #[doc(alias = "proposal-page-size")]
    fn set_proposal_page_size(&self, proposal_page_size: u32);

    #[doc(alias = "provider-page-size")]
    fn provider_page_size(&self) -> u32;

    #[doc(alias = "provider-page-size")]
    fn set_provider_page_size(&self, provider_page_size: u32);

    #[doc(alias = "remember-info-visibility")]
    fn is_remember_info_visibility(&self) -> bool;

    #[doc(alias = "remember-info-visibility")]
    fn set_remember_info_visibility(&self, remember_info_visibility: bool);

    #[doc(alias = "select-on-show")]
    fn selects_on_show(&self) -> bool;

    #[doc(alias = "select-on-show")]
    fn set_select_on_show(&self, select_on_show: bool);

    #[doc(alias = "show-headers")]
    fn shows_headers(&self) -> bool;

    #[doc(alias = "show-headers")]
    fn set_show_headers(&self, show_headers: bool);

    #[doc(alias = "show-icons")]
    fn shows_icons(&self) -> bool;

    #[doc(alias = "show-icons")]
    fn set_show_icons(&self, show_icons: bool);

    #[doc(alias = "activate-proposal")]
    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_proposal(&self);

    #[doc(alias = "hide")]
    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_hide(&self);

    #[doc(alias = "move-cursor")]
    fn connect_move_cursor<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_cursor(&self, step: gtk::ScrollStep, num: i32);

    #[doc(alias = "move-page")]
    fn connect_move_page<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_page(&self, step: gtk::ScrollStep, num: i32);

    #[doc(alias = "populate-context")]
    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_populate_context(&self, context: &CompletionContext);

    #[doc(alias = "show")]
    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show(&self);

    #[doc(alias = "accelerators")]
    fn connect_accelerators_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "auto-complete-delay")]
    fn connect_auto_complete_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "proposal-page-size")]
    fn connect_proposal_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "provider-page-size")]
    fn connect_provider_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "remember-info-visibility")]
    fn connect_remember_info_visibility_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "select-on-show")]
    fn connect_select_on_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-headers")]
    fn connect_show_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-icons")]
    fn connect_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Completion>> CompletionExt for O {
    fn add_provider(&self, provider: &impl IsA<CompletionProvider>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_source_completion_add_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn block_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_block_interactive(self.as_ref().to_glib_none().0);
        }
    }

    fn info_window(&self) -> Option<CompletionInfo> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_get_info_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn providers(&self) -> Vec<CompletionProvider> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_completion_get_providers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide(&self) {
        unsafe {
            ffi::gtk_source_completion_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_provider(&self, provider: &impl IsA<CompletionProvider>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_source_completion_remove_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn start(
        &self,
        providers: &[CompletionProvider],
        context: &impl IsA<CompletionContext>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_start(
                self.as_ref().to_glib_none().0,
                providers.to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    fn unblock_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_unblock_interactive(self.as_ref().to_glib_none().0);
        }
    }

    fn accelerators(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "accelerators")
    }

    fn set_accelerators(&self, accelerators: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "accelerators", &accelerators)
    }

    fn auto_complete_delay(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "auto-complete-delay")
    }

    fn set_auto_complete_delay(&self, auto_complete_delay: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "auto-complete-delay", &auto_complete_delay)
    }

    fn proposal_page_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "proposal-page-size")
    }

    fn set_proposal_page_size(&self, proposal_page_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "proposal-page-size", &proposal_page_size)
    }

    fn provider_page_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "provider-page-size")
    }

    fn set_provider_page_size(&self, provider_page_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "provider-page-size", &provider_page_size)
    }

    fn is_remember_info_visibility(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "remember-info-visibility")
    }

    fn set_remember_info_visibility(&self, remember_info_visibility: bool) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "remember-info-visibility",
            &remember_info_visibility,
        )
    }

    fn selects_on_show(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "select-on-show")
    }

    fn set_select_on_show(&self, select_on_show: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "select-on-show", &select_on_show)
    }

    fn shows_headers(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-headers")
    }

    fn set_show_headers(&self, show_headers: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-headers", &show_headers)
    }

    fn shows_icons(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-icons")
    }

    fn set_show_icons(&self, show_icons: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-icons", &show_icons)
    }

    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_proposal_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-proposal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_proposal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_proposal(&self) {
        self.emit_by_name::<()>("activate-proposal", &[]);
    }

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hide_trampoline<P: IsA<Completion>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"hide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    hide_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_hide(&self) {
        self.emit_by_name::<()>("hide", &[]);
    }

    fn connect_move_cursor<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            P: IsA<Completion>,
            F: Fn(&P, gtk::ScrollStep, i32) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            step: gtk::ffi::GtkScrollStep,
            num: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Completion::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(step),
                num,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_cursor(&self, step: gtk::ScrollStep, num: i32) {
        self.emit_by_name::<()>("move-cursor", &[&step, &num]);
    }

    fn connect_move_page<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_page_trampoline<
            P: IsA<Completion>,
            F: Fn(&P, gtk::ScrollStep, i32) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            step: gtk::ffi::GtkScrollStep,
            num: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Completion::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(step),
                num,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_page_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_page(&self, step: gtk::ScrollStep, num: i32) {
        self.emit_by_name::<()>("move-page", &[&step, &num]);
    }

    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn populate_context_trampoline<
            P: IsA<Completion>,
            F: Fn(&P, &CompletionContext) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            context: *mut ffi::GtkSourceCompletionContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Completion::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"populate-context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    populate_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_populate_context(&self, context: &CompletionContext) {
        self.emit_by_name::<()>("populate-context", &[&context]);
    }

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_trampoline<P: IsA<Completion>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_show(&self) {
        self.emit_by_name::<()>("show", &[]);
    }

    fn connect_accelerators_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerators_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accelerators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accelerators_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_auto_complete_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_complete_delay_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-complete-delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_complete_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_proposal_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proposal_page_size_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proposal-page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proposal_page_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_provider_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_provider_page_size_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::provider-page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_provider_page_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remember_info_visibility_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remember_info_visibility_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::remember-info-visibility\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remember_info_visibility_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_select_on_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_on_show_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-on-show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_on_show_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_headers_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-headers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_headers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_icons_trampoline<
            P: IsA<Completion>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Completion::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-icons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_icons_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Completion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Completion")
    }
}
