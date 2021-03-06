// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BracketMatchType;
use crate::ChangeCaseType;
use crate::Language;
use crate::Mark;
use crate::SortFlags;
use crate::StyleScheme;
use crate::UndoManager;
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
    #[doc(alias = "GtkSourceBuffer")]
    pub struct Buffer(Object<ffi::GtkSourceBuffer, ffi::GtkSourceBufferClass>) @extends gtk::TextBuffer;

    match fn {
        type_ => || ffi::gtk_source_buffer_get_type(),
    }
}

impl Buffer {
    pub const NONE: Option<&'static Buffer> = None;

    #[doc(alias = "gtk_source_buffer_new")]
    pub fn new(table: Option<&impl IsA<gtk::TextTagTable>>) -> Buffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_buffer_new(
                table.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_buffer_new_with_language")]
    #[doc(alias = "new_with_language")]
    pub fn with_language(language: &impl IsA<Language>) -> Buffer {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_buffer_new_with_language(
                language.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Buffer`] objects.
    ///
    /// This method returns an instance of [`BufferBuilder`](crate::builders::BufferBuilder) which can be used to create [`Buffer`] objects.
    pub fn builder() -> BufferBuilder {
        BufferBuilder::default()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct Buffer object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Buffer`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BufferBuilder {
    highlight_matching_brackets: Option<bool>,
    highlight_syntax: Option<bool>,
    implicit_trailing_newline: Option<bool>,
    language: Option<Language>,
    max_undo_levels: Option<i32>,
    style_scheme: Option<StyleScheme>,
    undo_manager: Option<UndoManager>,
    tag_table: Option<gtk::TextTagTable>,
    text: Option<String>,
}

impl BufferBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`BufferBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Buffer`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Buffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref highlight_matching_brackets) = self.highlight_matching_brackets {
            properties.push(("highlight-matching-brackets", highlight_matching_brackets));
        }
        if let Some(ref highlight_syntax) = self.highlight_syntax {
            properties.push(("highlight-syntax", highlight_syntax));
        }
        if let Some(ref implicit_trailing_newline) = self.implicit_trailing_newline {
            properties.push(("implicit-trailing-newline", implicit_trailing_newline));
        }
        if let Some(ref language) = self.language {
            properties.push(("language", language));
        }
        if let Some(ref max_undo_levels) = self.max_undo_levels {
            properties.push(("max-undo-levels", max_undo_levels));
        }
        if let Some(ref style_scheme) = self.style_scheme {
            properties.push(("style-scheme", style_scheme));
        }
        if let Some(ref undo_manager) = self.undo_manager {
            properties.push(("undo-manager", undo_manager));
        }
        if let Some(ref tag_table) = self.tag_table {
            properties.push(("tag-table", tag_table));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        glib::Object::new::<Buffer>(&properties).expect("Failed to create an instance of Buffer")
    }

    pub fn highlight_matching_brackets(mut self, highlight_matching_brackets: bool) -> Self {
        self.highlight_matching_brackets = Some(highlight_matching_brackets);
        self
    }

    pub fn highlight_syntax(mut self, highlight_syntax: bool) -> Self {
        self.highlight_syntax = Some(highlight_syntax);
        self
    }

    pub fn implicit_trailing_newline(mut self, implicit_trailing_newline: bool) -> Self {
        self.implicit_trailing_newline = Some(implicit_trailing_newline);
        self
    }

    pub fn language(mut self, language: &impl IsA<Language>) -> Self {
        self.language = Some(language.clone().upcast());
        self
    }

    pub fn max_undo_levels(mut self, max_undo_levels: i32) -> Self {
        self.max_undo_levels = Some(max_undo_levels);
        self
    }

    pub fn style_scheme(mut self, style_scheme: &impl IsA<StyleScheme>) -> Self {
        self.style_scheme = Some(style_scheme.clone().upcast());
        self
    }

    pub fn undo_manager(mut self, undo_manager: &impl IsA<UndoManager>) -> Self {
        self.undo_manager = Some(undo_manager.clone().upcast());
        self
    }

    pub fn tag_table(mut self, tag_table: &impl IsA<gtk::TextTagTable>) -> Self {
        self.tag_table = Some(tag_table.clone().upcast());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

pub trait BufferExt: 'static {
    //#[doc(alias = "gtk_source_buffer_backward_iter_to_source_mark")]
    //fn backward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool;

    #[doc(alias = "gtk_source_buffer_begin_not_undoable_action")]
    fn begin_not_undoable_action(&self);

    #[doc(alias = "gtk_source_buffer_can_redo")]
    fn can_redo(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_can_undo")]
    fn can_undo(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_change_case")]
    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    );

    #[doc(alias = "gtk_source_buffer_create_source_mark")]
    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark>;

    //#[doc(alias = "gtk_source_buffer_create_source_tag")]
    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag>;

    #[doc(alias = "gtk_source_buffer_end_not_undoable_action")]
    fn end_not_undoable_action(&self);

    #[doc(alias = "gtk_source_buffer_ensure_highlight")]
    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter);

    //#[doc(alias = "gtk_source_buffer_forward_iter_to_source_mark")]
    //fn forward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool;

    #[doc(alias = "gtk_source_buffer_get_context_classes_at_iter")]
    #[doc(alias = "get_context_classes_at_iter")]
    fn context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<glib::GString>;

    #[doc(alias = "gtk_source_buffer_get_highlight_matching_brackets")]
    #[doc(alias = "get_highlight_matching_brackets")]
    fn is_highlight_matching_brackets(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_highlight_syntax")]
    #[doc(alias = "get_highlight_syntax")]
    fn is_highlight_syntax(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_implicit_trailing_newline")]
    #[doc(alias = "get_implicit_trailing_newline")]
    fn is_implicit_trailing_newline(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_language")]
    #[doc(alias = "get_language")]
    fn language(&self) -> Option<Language>;

    #[doc(alias = "gtk_source_buffer_get_max_undo_levels")]
    #[doc(alias = "get_max_undo_levels")]
    fn max_undo_levels(&self) -> i32;

    #[doc(alias = "gtk_source_buffer_get_source_marks_at_iter")]
    #[doc(alias = "get_source_marks_at_iter")]
    fn source_marks_at_iter(&self, iter: &mut gtk::TextIter, category: Option<&str>) -> Vec<Mark>;

    #[doc(alias = "gtk_source_buffer_get_source_marks_at_line")]
    #[doc(alias = "get_source_marks_at_line")]
    fn source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark>;

    #[doc(alias = "gtk_source_buffer_get_style_scheme")]
    #[doc(alias = "get_style_scheme")]
    fn style_scheme(&self) -> Option<StyleScheme>;

    #[doc(alias = "gtk_source_buffer_get_undo_manager")]
    #[doc(alias = "get_undo_manager")]
    fn undo_manager(&self) -> Option<UndoManager>;

    //#[doc(alias = "gtk_source_buffer_iter_backward_to_context_class_toggle")]
    //fn iter_backward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool;

    //#[doc(alias = "gtk_source_buffer_iter_forward_to_context_class_toggle")]
    //fn iter_forward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool;

    #[doc(alias = "gtk_source_buffer_iter_has_context_class")]
    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool;

    #[doc(alias = "gtk_source_buffer_join_lines")]
    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    #[doc(alias = "gtk_source_buffer_redo")]
    fn redo(&self);

    #[doc(alias = "gtk_source_buffer_remove_source_marks")]
    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    );

    #[doc(alias = "gtk_source_buffer_set_highlight_matching_brackets")]
    fn set_highlight_matching_brackets(&self, highlight: bool);

    #[doc(alias = "gtk_source_buffer_set_highlight_syntax")]
    fn set_highlight_syntax(&self, highlight: bool);

    #[doc(alias = "gtk_source_buffer_set_implicit_trailing_newline")]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool);

    #[doc(alias = "gtk_source_buffer_set_language")]
    fn set_language(&self, language: Option<&impl IsA<Language>>);

    #[doc(alias = "gtk_source_buffer_set_max_undo_levels")]
    fn set_max_undo_levels(&self, max_undo_levels: i32);

    #[doc(alias = "gtk_source_buffer_set_style_scheme")]
    fn set_style_scheme(&self, scheme: Option<&impl IsA<StyleScheme>>);

    #[doc(alias = "gtk_source_buffer_set_undo_manager")]
    fn set_undo_manager(&self, manager: Option<&impl IsA<UndoManager>>);

    #[doc(alias = "gtk_source_buffer_sort_lines")]
    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    );

    #[doc(alias = "gtk_source_buffer_undo")]
    fn undo(&self);

    #[doc(alias = "bracket-matched")]
    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "highlight-updated")]
    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "redo")]
    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "source-mark-updated")]
    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "undo")]
    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-redo")]
    fn connect_can_redo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-undo")]
    fn connect_can_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "highlight-matching-brackets")]
    fn connect_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "highlight-syntax")]
    fn connect_highlight_syntax_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "implicit-trailing-newline")]
    fn connect_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "language")]
    fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "max-undo-levels")]
    fn connect_max_undo_levels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "style-scheme")]
    fn connect_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "undo-manager")]
    fn connect_undo_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Buffer>> BufferExt for O {
    //fn backward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_backward_iter_to_source_mark() }
    //}

    fn begin_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_buffer_begin_not_undoable_action(self.as_ref().to_glib_none().0);
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_can_redo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_can_undo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    ) {
        unsafe {
            ffi::gtk_source_buffer_change_case(
                self.as_ref().to_glib_none().0,
                case_type.into_glib(),
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_create_source_mark(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                category.to_glib_none().0,
                where_.to_glib_none().0,
            ))
        }
    }

    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag> {
    //    unsafe { TODO: call ffi:gtk_source_buffer_create_source_tag() }
    //}

    fn end_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_buffer_end_not_undoable_action(self.as_ref().to_glib_none().0);
        }
    }

    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter) {
        unsafe {
            ffi::gtk_source_buffer_ensure_highlight(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
            );
        }
    }

    //fn forward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_forward_iter_to_source_mark() }
    //}

    fn context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gtk_source_buffer_get_context_classes_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none().0,
                ),
            )
        }
    }

    fn is_highlight_matching_brackets(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_matching_brackets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_syntax(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_implicit_trailing_newline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_implicit_trailing_newline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn language(&self) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn max_undo_levels(&self) -> i32 {
        unsafe { ffi::gtk_source_buffer_get_max_undo_levels(self.as_ref().to_glib_none().0) }
    }

    fn source_marks_at_iter(&self, iter: &mut gtk::TextIter, category: Option<&str>) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::gtk_source_buffer_get_source_marks_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::gtk_source_buffer_get_source_marks_at_line(
                    self.as_ref().to_glib_none().0,
                    line,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_style_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn undo_manager(&self) -> Option<UndoManager> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_undo_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn iter_backward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_iter_backward_to_context_class_toggle() }
    //}

    //fn iter_forward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_iter_forward_to_context_class_toggle() }
    //}

    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_iter_has_context_class(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                context_class.to_glib_none().0,
            ))
        }
    }

    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_buffer_join_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn redo(&self) {
        unsafe {
            ffi::gtk_source_buffer_redo(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    ) {
        unsafe {
            ffi::gtk_source_buffer_remove_source_marks(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                category.to_glib_none().0,
            );
        }
    }

    fn set_highlight_matching_brackets(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_matching_brackets(
                self.as_ref().to_glib_none().0,
                highlight.into_glib(),
            );
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_syntax(
                self.as_ref().to_glib_none().0,
                highlight.into_glib(),
            );
        }
    }

    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_implicit_trailing_newline(
                self.as_ref().to_glib_none().0,
                implicit_trailing_newline.into_glib(),
            );
        }
    }

    fn set_language(&self, language: Option<&impl IsA<Language>>) {
        unsafe {
            ffi::gtk_source_buffer_set_language(
                self.as_ref().to_glib_none().0,
                language.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_max_undo_levels(&self, max_undo_levels: i32) {
        unsafe {
            ffi::gtk_source_buffer_set_max_undo_levels(
                self.as_ref().to_glib_none().0,
                max_undo_levels,
            );
        }
    }

    fn set_style_scheme(&self, scheme: Option<&impl IsA<StyleScheme>>) {
        unsafe {
            ffi::gtk_source_buffer_set_style_scheme(
                self.as_ref().to_glib_none().0,
                scheme.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_undo_manager(&self, manager: Option<&impl IsA<UndoManager>>) {
        unsafe {
            ffi::gtk_source_buffer_set_undo_manager(
                self.as_ref().to_glib_none().0,
                manager.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    ) {
        unsafe {
            ffi::gtk_source_buffer_sort_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                flags.into_glib(),
                column,
            );
        }
    }

    fn undo(&self) {
        unsafe {
            ffi::gtk_source_buffer_undo(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn bracket_matched_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P, Option<&gtk::TextIter>, BracketMatchType) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            iter: *mut gtk::ffi::GtkTextIter,
            state: ffi::GtkSourceBracketMatchType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gtk::TextIter>::from_glib_borrow(iter)
                    .as_ref()
                    .as_ref(),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bracket-matched\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bracket_matched_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn highlight_updated_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P, &gtk::TextIter, &gtk::TextIter) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            start: *mut gtk::ffi::GtkTextIter,
            end: *mut gtk::ffi::GtkTextIter,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(start),
                &from_glib_borrow(end),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"highlight-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    highlight_updated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn redo_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"redo\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    redo_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn source_mark_updated_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P, &gtk::TextMark) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            mark: *mut gtk::ffi::GtkTextMark,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mark),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"source-mark-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    source_mark_updated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn undo_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"undo\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    undo_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_redo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_redo_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-redo\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_redo_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_undo_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-undo\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_undo_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_matching_brackets_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-matching-brackets\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_matching_brackets_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_highlight_syntax_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_syntax_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-syntax\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_syntax_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_implicit_trailing_newline_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::implicit-trailing-newline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_implicit_trailing_newline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_max_undo_levels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_undo_levels_trampoline<
            P: IsA<Buffer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-undo-levels\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_undo_levels_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_scheme_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-scheme\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_style_scheme_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_undo_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_undo_manager_trampoline<P: IsA<Buffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::undo-manager\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_undo_manager_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Buffer")
    }
}
