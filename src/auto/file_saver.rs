// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buffer;
use crate::CompressionType;
use crate::Encoding;
use crate::File;
use crate::FileSaverFlags;
use crate::NewlineType;
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
    #[doc(alias = "GtkSourceFileSaver")]
    pub struct FileSaver(Object<ffi::GtkSourceFileSaver, ffi::GtkSourceFileSaverClass>);

    match fn {
        type_ => || ffi::gtk_source_file_saver_get_type(),
    }
}

impl FileSaver {
    pub const NONE: Option<&'static FileSaver> = None;

    #[doc(alias = "gtk_source_file_saver_new")]
    pub fn new(buffer: &impl IsA<Buffer>, file: &impl IsA<File>) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_file_saver_new_with_target")]
    #[doc(alias = "new_with_target")]
    pub fn with_target(
        buffer: &impl IsA<Buffer>,
        file: &impl IsA<File>,
        target_location: &impl IsA<gio::File>,
    ) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new_with_target(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                target_location.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileSaver`] objects.
    ///
    /// This method returns an instance of [`FileSaverBuilder`](crate::builders::FileSaverBuilder) which can be used to create [`FileSaver`] objects.
    pub fn builder() -> FileSaverBuilder {
        FileSaverBuilder::default()
    }
}

impl Default for FileSaver {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct FileSaver object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileSaver`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileSaverBuilder {
    buffer: Option<Buffer>,
    compression_type: Option<CompressionType>,
    encoding: Option<Encoding>,
    file: Option<File>,
    flags: Option<FileSaverFlags>,
    location: Option<gio::File>,
    newline_type: Option<NewlineType>,
}

impl FileSaverBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FileSaverBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileSaver`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileSaver {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref compression_type) = self.compression_type {
            properties.push(("compression-type", compression_type));
        }
        if let Some(ref encoding) = self.encoding {
            properties.push(("encoding", encoding));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref location) = self.location {
            properties.push(("location", location));
        }
        if let Some(ref newline_type) = self.newline_type {
            properties.push(("newline-type", newline_type));
        }
        glib::Object::new::<FileSaver>(&properties)
            .expect("Failed to create an instance of FileSaver")
    }

    pub fn buffer(mut self, buffer: &impl IsA<Buffer>) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn compression_type(mut self, compression_type: CompressionType) -> Self {
        self.compression_type = Some(compression_type);
        self
    }

    pub fn encoding(mut self, encoding: &Encoding) -> Self {
        self.encoding = Some(encoding.clone());
        self
    }

    pub fn file(mut self, file: &impl IsA<File>) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn flags(mut self, flags: FileSaverFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn location(mut self, location: &impl IsA<gio::File>) -> Self {
        self.location = Some(location.clone().upcast());
        self
    }

    pub fn newline_type(mut self, newline_type: NewlineType) -> Self {
        self.newline_type = Some(newline_type);
        self
    }
}

pub trait FileSaverExt: 'static {
    #[doc(alias = "gtk_source_file_saver_get_buffer")]
    #[doc(alias = "get_buffer")]
    fn buffer(&self) -> Option<Buffer>;

    #[doc(alias = "gtk_source_file_saver_get_compression_type")]
    #[doc(alias = "get_compression_type")]
    fn compression_type(&self) -> CompressionType;

    #[doc(alias = "gtk_source_file_saver_get_encoding")]
    #[doc(alias = "get_encoding")]
    fn encoding(&self) -> Option<Encoding>;

    #[doc(alias = "gtk_source_file_saver_get_file")]
    #[doc(alias = "get_file")]
    fn file(&self) -> Option<File>;

    #[doc(alias = "gtk_source_file_saver_get_flags")]
    #[doc(alias = "get_flags")]
    fn flags(&self) -> FileSaverFlags;

    #[doc(alias = "gtk_source_file_saver_get_location")]
    #[doc(alias = "get_location")]
    fn location(&self) -> Option<gio::File>;

    #[doc(alias = "gtk_source_file_saver_get_newline_type")]
    #[doc(alias = "get_newline_type")]
    fn newline_type(&self) -> NewlineType;

    //#[doc(alias = "gtk_source_file_saver_save_async")]
    //fn save_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, progress_callback_notify: Fn() + 'static, callback: Q);

    //
    //fn save_future<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: P, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "gtk_source_file_saver_set_compression_type")]
    fn set_compression_type(&self, compression_type: CompressionType);

    #[doc(alias = "gtk_source_file_saver_set_encoding")]
    fn set_encoding(&self, encoding: Option<&Encoding>);

    #[doc(alias = "gtk_source_file_saver_set_flags")]
    fn set_flags(&self, flags: FileSaverFlags);

    #[doc(alias = "gtk_source_file_saver_set_newline_type")]
    fn set_newline_type(&self, newline_type: NewlineType);

    #[doc(alias = "compression-type")]
    fn connect_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "encoding")]
    fn connect_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "newline-type")]
    fn connect_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileSaver>> FileSaverExt for O {
    fn buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_compression_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_encoding(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn flags(&self) -> FileSaverFlags {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn save_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, progress_callback_notify: Fn() + 'static, callback: Q) {
    //    unsafe { TODO: call ffi:gtk_source_file_saver_save_async() }
    //}

    //
    //fn save_future<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: P, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

    //let progress_callback = progress_callback.map(ToOwned::to_owned);
    //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
    //Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.save_async(
    //        io_priority,
    //        Some(cancellable),
    //        progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
    //        progress_callback_notify.as_ref().map(::std::borrow::Borrow::borrow),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );
    //}))
    //}

    fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            ffi::gtk_source_file_saver_set_compression_type(
                self.as_ref().to_glib_none().0,
                compression_type.into_glib(),
            );
        }
    }

    fn set_encoding(&self, encoding: Option<&Encoding>) {
        unsafe {
            ffi::gtk_source_file_saver_set_encoding(
                self.as_ref().to_glib_none().0,
                encoding.to_glib_none().0,
            );
        }
    }

    fn set_flags(&self, flags: FileSaverFlags) {
        unsafe {
            ffi::gtk_source_file_saver_set_flags(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            ffi::gtk_source_file_saver_set_newline_type(
                self.as_ref().to_glib_none().0,
                newline_type.into_glib(),
            );
        }
    }

    fn connect_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<
            P: IsA<FileSaver>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceFileSaver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileSaver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_compression_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P: IsA<FileSaver>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileSaver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encoding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P: IsA<FileSaver>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceFileSaver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileSaver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<
            P: IsA<FileSaver>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceFileSaver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileSaver::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileSaver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileSaver")
    }
}