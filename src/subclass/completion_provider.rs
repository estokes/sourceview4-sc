use crate::{
    CompletionActivation, CompletionContext, CompletionInfo, CompletionProposal, CompletionProvider,
};
use glib::{prelude::*, subclass::prelude::*, translate::*};
use libc::{c_char, c_int, c_uint};
use std::ptr;

pub trait CompletionProviderImpl: ObjectImpl {
    fn activate_proposal(
        &self,
        proposal: &impl IsA<CompletionProposal>,
        iter: &gtk::TextIter,
    ) -> bool;
    fn activation(&self) -> CompletionActivation;
    fn gicon(&self) -> Option<gio::Icon>;
    fn icon(&self) -> Option<gdk_pixbuf::Pixbuf>;
    fn icon_name(&self) -> Option<glib::GString>;
    fn info_widget(&self, proposal: &impl IsA<CompletionProposal>) -> Option<gtk::Widget>;
    fn interactive_delay(&self) -> i32;
    fn name(&self) -> Option<glib::GString>;
    fn priority(&self) -> i32;
    fn start_iter(
        &self,
        context: &impl IsA<CompletionContext>,
        proposal: &impl IsA<CompletionProposal>,
    ) -> Option<gtk::TextIter>;
    fn match_(&self, context: &impl IsA<CompletionContext>) -> bool;
    fn populate(
        &self,
        provider: &impl IsA<CompletionProvider>,
        context: &impl IsA<CompletionContext>,
    );
    fn update_info(&self, proposal: &impl IsA<CompletionProposal>, info: &impl IsA<CompletionInfo>);
}

unsafe impl<T: CompletionProviderImpl> IsImplementable<T> for CompletionProvider {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.activate_proposal = Some(completion_provider_activate_proposal::<T>);
        iface.get_activation = Some(completion_provider_get_activation::<T>);
        iface.get_gicon = Some(completion_provider_get_gicon::<T>);
        iface.get_icon = Some(completion_provider_get_icon::<T>);
        iface.get_icon_name = Some(completion_provider_get_icon_name::<T>);
        iface.get_info_widget = Some(completion_provider_get_info_widget::<T>);
        iface.get_interactive_delay = Some(completion_provider_get_interactive_delay::<T>);
        iface.get_name = Some(completion_provider_get_name::<T>);
        iface.get_priority = Some(completion_provider_get_priority::<T>);
        iface.get_start_iter = Some(completion_provider_get_start_iter::<T>);
        iface.match_ = Some(completion_provider_match::<T>);
        iface.populate = Some(completion_provider_populate::<T>);
        iface.update_info = Some(completion_provider_update_info::<T>);
    }

    fn instance_init(_inst: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn completion_provider_activate_proposal<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    iter: *mut gtk::ffi::GtkTextIter,
) -> c_int {
    (&*(t as *mut T::Instance))
        .imp()
        .activate_proposal(
            from_glib_borrow::<_, CompletionProposal>(proposal).as_ref(),
            from_glib_borrow::<_, gtk::TextIter>(iter).as_ref(),
        )
        .into_glib()
}

unsafe extern "C" fn completion_provider_get_activation<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> c_uint {
    (&*(t as *mut T::Instance)).imp().activation().bits()
}

unsafe extern "C" fn completion_provider_get_gicon<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *mut gio::ffi::GIcon {
    (&*(t as *mut T::Instance))
        .imp()
        .gicon()
        .map(|o| o.to_glib_full())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_icon<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *mut gdk_pixbuf::ffi::GdkPixbuf {
    (&*(t as *mut T::Instance))
        .imp()
        .icon()
        .map(|o| o.to_glib_full())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_icon_name<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *const c_char {
    (&*(t as *mut T::Instance))
        .imp()
        .icon_name()
        .map(|s| s.to_glib_full())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_info_widget<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
) -> *mut gtk::ffi::GtkWidget {
    (&*(t as *mut T::Instance))
        .imp()
        .info_widget(from_glib_borrow::<_, CompletionProposal>(proposal).as_ref())
        .map(|o| o.to_glib_full())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_interactive_delay<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> c_int {
    (&*(t as *mut T::Instance)).imp().interactive_delay()
}

unsafe extern "C" fn completion_provider_get_name<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *const c_char {
    (&*(t as *mut T::Instance))
        .imp()
        .name()
        .map(|o| o.to_glib_full())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_priority<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> c_int {
    (&*(t as *mut T::Instance)).imp().priority()
}

unsafe extern "C" fn completion_provider_get_start_iter<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    context: *mut ffi::GtkSourceCompletionContext,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    iter: *mut gtk::ffi::GtkTextIter,
) -> c_int {
    let r = (&*(t as *mut T::Instance)).imp().start_iter(
        from_glib_borrow::<_, CompletionContext>(context).as_ref(),
        from_glib_borrow::<_, CompletionProposal>(proposal).as_ref(),
    );
    match r {
        None => false.into_glib(),
        Some(i) => {
            *iter = *i.to_glib_full();
            true.into_glib()
        }
    }
}

unsafe extern "C" fn completion_provider_match<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    context: *mut ffi::GtkSourceCompletionContext,
) -> c_int {
    (&*(t as *mut T::Instance))
        .imp()
        .match_(from_glib_borrow::<_, CompletionContext>(context).as_ref())
        .into_glib()
}

unsafe extern "C" fn completion_provider_populate<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    context: *mut ffi::GtkSourceCompletionContext,
) {
    (&*(t as *mut T::Instance)).imp().populate(
        from_glib_borrow::<_, CompletionProvider>(t).as_ref(),
        from_glib_borrow::<_, CompletionContext>(context).as_ref(),
    );
}

unsafe extern "C" fn completion_provider_update_info<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    info: *mut ffi::GtkSourceCompletionInfo,
) {
    (&*(t as *mut T::Instance)).imp().update_info(
        from_glib_borrow::<_, CompletionProposal>(proposal).as_ref(),
        from_glib_borrow::<_, CompletionInfo>(info).as_ref(),
    )
}
