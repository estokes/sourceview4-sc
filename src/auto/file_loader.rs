// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buffer;
use crate::CompressionType;
use crate::Encoding;
use crate::File;
use crate::NewlineType;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkSourceFileLoader")]
    pub struct FileLoader(Object<ffi::GtkSourceFileLoader, ffi::GtkSourceFileLoaderClass>);

    match fn {
        type_ => || ffi::gtk_source_file_loader_get_type(),
    }
}

impl FileLoader {
    pub const NONE: Option<&'static FileLoader> = None;

    #[doc(alias = "gtk_source_file_loader_new")]
    pub fn new(buffer: &impl IsA<Buffer>, file: &impl IsA<File>) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_loader_new(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_file_loader_new_from_stream")]
    #[doc(alias = "new_from_stream")]
    pub fn from_stream(
        buffer: &impl IsA<Buffer>,
        file: &impl IsA<File>,
        stream: &impl IsA<gio::InputStream>,
    ) -> FileLoader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_loader_new_from_stream(
                buffer.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileLoader`] objects.
    ///
    /// This method returns an instance of [`FileLoaderBuilder`](crate::builders::FileLoaderBuilder) which can be used to create [`FileLoader`] objects.
    pub fn builder() -> FileLoaderBuilder {
        FileLoaderBuilder::default()
    }
}

impl Default for FileLoader {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct FileLoader object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileLoader`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileLoaderBuilder {
    buffer: Option<Buffer>,
    file: Option<File>,
    input_stream: Option<gio::InputStream>,
    location: Option<gio::File>,
}

impl FileLoaderBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FileLoaderBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileLoader`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileLoader {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref input_stream) = self.input_stream {
            properties.push(("input-stream", input_stream));
        }
        if let Some(ref location) = self.location {
            properties.push(("location", location));
        }
        glib::Object::new::<FileLoader>(&properties)
            .expect("Failed to create an instance of FileLoader")
    }

    pub fn buffer(mut self, buffer: &impl IsA<Buffer>) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn file(mut self, file: &impl IsA<File>) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn input_stream(mut self, input_stream: &impl IsA<gio::InputStream>) -> Self {
        self.input_stream = Some(input_stream.clone().upcast());
        self
    }

    pub fn location(mut self, location: &impl IsA<gio::File>) -> Self {
        self.location = Some(location.clone().upcast());
        self
    }
}

pub trait FileLoaderExt: 'static {
    #[doc(alias = "gtk_source_file_loader_get_buffer")]
    #[doc(alias = "get_buffer")]
    fn buffer(&self) -> Option<Buffer>;

    #[doc(alias = "gtk_source_file_loader_get_compression_type")]
    #[doc(alias = "get_compression_type")]
    fn compression_type(&self) -> CompressionType;

    #[doc(alias = "gtk_source_file_loader_get_encoding")]
    #[doc(alias = "get_encoding")]
    fn encoding(&self) -> Option<Encoding>;

    #[doc(alias = "gtk_source_file_loader_get_file")]
    #[doc(alias = "get_file")]
    fn file(&self) -> Option<File>;

    #[doc(alias = "gtk_source_file_loader_get_input_stream")]
    #[doc(alias = "get_input_stream")]
    fn input_stream(&self) -> Option<gio::InputStream>;

    #[doc(alias = "gtk_source_file_loader_get_location")]
    #[doc(alias = "get_location")]
    fn location(&self) -> Option<gio::File>;

    #[doc(alias = "gtk_source_file_loader_get_newline_type")]
    #[doc(alias = "get_newline_type")]
    fn newline_type(&self) -> NewlineType;

    //#[doc(alias = "gtk_source_file_loader_load_async")]
    //fn load_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, progress_callback_notify: Fn() + 'static, callback: Q);

    //
    //fn load_future<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: P, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;
}

impl<O: IsA<FileLoader>> FileLoaderExt for O {
    fn buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_compression_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_encoding(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn load_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, progress_callback_notify: Fn() + 'static, callback: Q) {
    //    unsafe { TODO: call ffi:gtk_source_file_loader_load_async() }
    //}

    //
    //fn load_future<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, io_priority: glib::Priority, progress_callback: P, progress_callback_notify: Fn() + 'static) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

    //let progress_callback = progress_callback.map(ToOwned::to_owned);
    //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
    //Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.load_async(
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
}

impl fmt::Display for FileLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileLoader")
    }
}
