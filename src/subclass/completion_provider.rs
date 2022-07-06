use crate::{
    CompletionActivation, CompletionContext, CompletionInfo, CompletionProposal, CompletionProvider,
};
use glib::{ffi::gboolean, prelude::*, subclass::prelude::*, translate::*, Quark};
use libc::c_char;
use once_cell::sync::Lazy;
use std::ptr;

pub trait CompletionProviderImpl: ObjectImpl {
    fn activate_proposal(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        iter: &gtk::TextIter,
    ) -> bool {
        self.parent_activate_proposal(provider, proposal, iter)
    }

    fn activation(&self, provider: &Self::Type) -> CompletionActivation {
        self.parent_activation(provider)
    }

    fn gicon(&self, provider: &Self::Type) -> Option<gio::Icon> {
        self.parent_gicon(provider)
    }

    fn icon(&self, provider: &Self::Type) -> Option<gdk_pixbuf::Pixbuf> {
        self.parent_icon(provider)
    }

    fn icon_name(&self, provider: &Self::Type) -> Option<glib::GString> {
        self.parent_icon_name(provider)
    }

    fn info_widget(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
    ) -> Option<gtk::Widget> {
        self.parent_info_widget(provider, proposal)
    }

    fn interactive_delay(&self, provider: &Self::Type) -> i32 {
        self.parent_interactive_delay(provider)
    }

    fn name(&self, provider: &Self::Type) -> Option<glib::GString> {
        self.parent_name(provider)
    }

    fn priority(&self, provider: &Self::Type) -> i32 {
        self.parent_priority(provider)
    }

    fn start_iter(
        &self,
        provider: &Self::Type,
        context: &CompletionContext,
        proposal: &CompletionProposal,
    ) -> Option<gtk::TextIter> {
        self.parent_start_iter(provider, context, proposal)
    }

    fn match_(&self, provider: &Self::Type, context: &CompletionContext) -> bool {
        self.parent_match_(provider, context)
    }

    fn populate(&self, provider: &Self::Type, context: &CompletionContext) {
        self.parent_populate(provider, context)
    }

    fn update_info(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        info: &CompletionInfo,
    ) {
        self.parent_update_info(provider, proposal, info)
    }
}

pub trait CompletionProviderImplExt: ObjectSubclass {
    fn parent_activate_proposal(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        iter: &gtk::TextIter,
    ) -> bool;
    fn parent_activation(&self, provider: &Self::Type) -> CompletionActivation;
    fn parent_gicon(&self, provider: &Self::Type) -> Option<gio::Icon>;
    fn parent_icon(&self, provider: &Self::Type) -> Option<gdk_pixbuf::Pixbuf>;
    fn parent_icon_name(&self, provider: &Self::Type) -> Option<glib::GString>;
    fn parent_info_widget(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
    ) -> Option<gtk::Widget>;
    fn parent_interactive_delay(&self, provider: &Self::Type) -> i32;
    fn parent_name(&self, provider: &Self::Type) -> Option<glib::GString>;
    fn parent_priority(&self, provider: &Self::Type) -> i32;
    fn parent_start_iter(
        &self,
        provider: &Self::Type,
        context: &CompletionContext,
        proposal: &CompletionProposal,
    ) -> Option<gtk::TextIter>;
    fn parent_match_(&self, provider: &Self::Type, context: &CompletionContext) -> bool;
    fn parent_populate(&self, provider: &Self::Type, context: &CompletionContext);
    fn parent_update_info(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        info: &CompletionInfo,
    );
}

unsafe fn get_parent_iface<T: CompletionProviderImpl>(
) -> *const ffi::GtkSourceCompletionProviderIface {
    let type_data = T::type_data();
    type_data.as_ref().parent_interface::<CompletionProvider>()
        as *const ffi::GtkSourceCompletionProviderIface
}

unsafe fn provider_to_ffi<T: CompletionProviderImpl>(
    provider: &T::Type,
) -> *mut ffi::GtkSourceCompletionProvider {
    provider
        .unsafe_cast_ref::<CompletionProvider>()
        .to_glib_none()
        .0
}

impl<T: CompletionProviderImpl> CompletionProviderImplExt for T {
    fn parent_activate_proposal(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        iter: &gtk::TextIter,
    ) -> bool {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .activate_proposal
                .expect("no parent \"activate_proposal\" implementation");

            from_glib(func(
                provider_to_ffi::<Self>(provider),
                proposal.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    fn parent_activation(&self, provider: &Self::Type) -> CompletionActivation {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_activation
                .expect("no parent \"activation\" implementation");

            from_glib(func(provider_to_ffi::<Self>(provider)))
        }
    }

    fn parent_gicon(&self, provider: &Self::Type) -> Option<gio::Icon> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_gicon
                .expect("no parent \"gicon\" implementation");

            from_glib_none(func(provider_to_ffi::<Self>(provider)))
        }
    }

    fn parent_icon(&self, provider: &Self::Type) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_icon
                .expect("no parent \"icon\" implementation");

            from_glib_none(func(provider_to_ffi::<Self>(provider)))
        }
    }

    fn parent_icon_name(&self, provider: &Self::Type) -> Option<glib::GString> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_icon_name
                .expect("no parent \"icon_name\" implementation");

            from_glib_none(func(provider_to_ffi::<Self>(provider)))
        }
    }

    fn parent_info_widget(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
    ) -> Option<gtk::Widget> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_info_widget
                .expect("no parent \"info_widget\" implementation");

            from_glib_none(func(
                provider_to_ffi::<Self>(provider),
                proposal.to_glib_none().0,
            ))
        }
    }

    fn parent_interactive_delay(&self, provider: &Self::Type) -> i32 {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_interactive_delay
                .expect("no parent \"interactive_delay\" implementation");

            func(provider_to_ffi::<Self>(provider))
        }
    }

    fn parent_name(&self, provider: &Self::Type) -> Option<glib::GString> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_name
                .expect("no parent \"name\" implementation");

            from_glib_full(func(provider_to_ffi::<Self>(provider)))
        }
    }

    fn parent_priority(&self, provider: &Self::Type) -> i32 {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_priority
                .expect("no parent \"priority\" implementation");

            func(provider_to_ffi::<Self>(provider))
        }
    }

    fn parent_start_iter(
        &self,
        provider: &Self::Type,
        context: &CompletionContext,
        proposal: &CompletionProposal,
    ) -> Option<gtk::TextIter> {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .get_start_iter
                .expect("no parent \"start_iter\" implementation");

            let mut iter = gtk::TextIter::uninitialized();
            let res = from_glib(func(
                provider_to_ffi::<Self>(provider),
                context.to_glib_none().0,
                proposal.to_glib_none().0,
                iter.to_glib_none_mut().0,
            ));
            if res {
                Some(iter)
            } else {
                None
            }
        }
    }

    fn parent_match_(&self, provider: &Self::Type, context: &CompletionContext) -> bool {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .match_
                .expect("no parent \"match_\" implementation");

            from_glib(func(
                provider_to_ffi::<Self>(provider),
                context.to_glib_none().0,
            ))
        }
    }

    fn parent_populate(&self, provider: &Self::Type, context: &CompletionContext) {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .populate
                .expect("no parent \"populate\" implementation");

            func(provider_to_ffi::<Self>(provider), context.to_glib_none().0)
        }
    }

    fn parent_update_info(
        &self,
        provider: &Self::Type,
        proposal: &CompletionProposal,
        info: &CompletionInfo,
    ) {
        unsafe {
            let parent_iface = get_parent_iface::<Self>();

            let func = (*parent_iface)
                .update_info
                .expect("no parent \"update_info\" implementation");

            func(
                provider_to_ffi::<Self>(provider),
                proposal.to_glib_none().0,
                info.to_glib_none().0,
            )
        }
    }
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
}

/// Struct to hold a pointer and free it on `Drop::drop`
pub(crate) struct PtrHolder<T, F: Fn(*mut T) + 'static>(*mut T, F);

impl<T, F: Fn(*mut T) + 'static> Drop for PtrHolder<T, F> {
    fn drop(&mut self) {
        (self.1)(self.0)
    }
}

unsafe extern "C" fn completion_provider_activate_proposal<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    iter: *mut gtk::ffi::GtkTextIter,
) -> gboolean {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.activate_proposal(
        from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref(),
        &from_glib_borrow(proposal),
        &from_glib_borrow(iter),
    )
    .into_glib()
}

unsafe extern "C" fn completion_provider_get_activation<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> ffi::GtkSourceCompletionActivation {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.activation(from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref())
        .into_glib()
}

static GICON_KEY: Lazy<Quark> = Lazy::new(|| Quark::from_str("sourceview4-rs-subclass-gicon"));
unsafe extern "C" fn completion_provider_get_gicon<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *mut gio::ffi::GIcon {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, CompletionProvider>(t);

    imp.gicon(wrap.unsafe_cast_ref())
        .map(|icon| {
            let ret = icon.to_glib_full();
            wrap.set_qdata(
                *GICON_KEY,
                PtrHolder(ret, |ptr| glib::gobject_ffi::g_object_unref(ptr as *mut _)),
            );
            ret
        })
        .unwrap_or(ptr::null_mut())
}

static ICON_KEY: Lazy<Quark> = Lazy::new(|| Quark::from_str("sourceview4-rs-subclass-icon"));
unsafe extern "C" fn completion_provider_get_icon<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *mut gdk_pixbuf::ffi::GdkPixbuf {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, CompletionProvider>(t);

    imp.icon(wrap.unsafe_cast_ref())
        .map(|icon| {
            let ret = icon.to_glib_full();
            wrap.set_qdata(
                *ICON_KEY,
                PtrHolder(ret, |ptr| glib::gobject_ffi::g_object_unref(ptr as *mut _)),
            );
            ret
        })
        .unwrap_or(ptr::null_mut())
}

static ICON_NAME_KEY: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("sourceview4-rs-subclass-icon-name"));
unsafe extern "C" fn completion_provider_get_icon_name<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *const c_char {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, CompletionProvider>(t);

    imp.icon_name(wrap.unsafe_cast_ref())
        .map(|name| {
            let ret = name.to_glib_full();
            wrap.set_qdata(
                *ICON_NAME_KEY,
                PtrHolder(ret, |ptr| glib::gobject_ffi::g_object_unref(ptr as *mut _)),
            );
            ret
        })
        .unwrap_or(ptr::null_mut())
}

static INFO_WIDGET_KEY: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("sourceview4-rs-subclass-info-widget"));
unsafe extern "C" fn completion_provider_get_info_widget<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
) -> *mut gtk::ffi::GtkWidget {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, CompletionProvider>(t);

    imp.info_widget(wrap.unsafe_cast_ref(), &from_glib_borrow(proposal))
        .map(|widget| {
            let ret = widget.to_glib_full();
            wrap.set_qdata(
                *INFO_WIDGET_KEY,
                PtrHolder(ret, |ptr| glib::gobject_ffi::g_object_unref(ptr as *mut _)),
            );
            ret
        })
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn completion_provider_get_interactive_delay<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> i32 {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.interactive_delay(from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref())
}

unsafe extern "C" fn completion_provider_get_name<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> *const c_char {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.name(from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn completion_provider_get_priority<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
) -> i32 {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.priority(from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref())
}

unsafe extern "C" fn completion_provider_get_start_iter<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    context: *mut ffi::GtkSourceCompletionContext,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    iter: *mut gtk::ffi::GtkTextIter,
) -> gboolean {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    let r = imp.start_iter(
        from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref(),
        &from_glib_borrow(context),
        &from_glib_borrow(proposal),
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
) -> gboolean {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.match_(
        from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref(),
        &from_glib_borrow(context),
    )
    .into_glib()
}

unsafe extern "C" fn completion_provider_populate<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    context: *mut ffi::GtkSourceCompletionContext,
) {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.populate(
        from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref(),
        &from_glib_borrow(context),
    );
}

unsafe extern "C" fn completion_provider_update_info<T: CompletionProviderImpl>(
    t: *mut ffi::GtkSourceCompletionProvider,
    proposal: *mut ffi::GtkSourceCompletionProposal,
    info: *mut ffi::GtkSourceCompletionInfo,
) {
    let instance = &*(t as *mut T::Instance);
    let imp = instance.imp();

    imp.update_info(
        from_glib_borrow::<_, CompletionProvider>(t).unsafe_cast_ref(),
        &from_glib_borrow(proposal),
        &from_glib_borrow(info),
    )
}
