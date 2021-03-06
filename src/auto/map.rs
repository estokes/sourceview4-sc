// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BackgroundPatternType;
use crate::SmartHomeEndType;
use crate::View;
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
    #[doc(alias = "GtkSourceMap")]
    pub struct Map(Object<ffi::GtkSourceMap, ffi::GtkSourceMapClass>) @extends View, gtk::TextView, gtk::Container, gtk::Widget, @implements gtk::Buildable, gtk::Scrollable;

    match fn {
        type_ => || ffi::gtk_source_map_get_type(),
    }
}

impl Map {
    pub const NONE: Option<&'static Map> = None;

    #[doc(alias = "gtk_source_map_new")]
    pub fn new() -> Map {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::gtk_source_map_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Map`] objects.
    ///
    /// This method returns an instance of [`MapBuilder`](crate::builders::MapBuilder) which can be used to create [`Map`] objects.
    pub fn builder() -> MapBuilder {
        MapBuilder::default()
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Map`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MapBuilder {
    font_desc: Option<pango::FontDescription>,
    view: Option<View>,
    auto_indent: Option<bool>,
    background_pattern: Option<BackgroundPatternType>,
    highlight_current_line: Option<bool>,
    indent_on_tab: Option<bool>,
    indent_width: Option<i32>,
    insert_spaces_instead_of_tabs: Option<bool>,
    right_margin_position: Option<u32>,
    show_line_marks: Option<bool>,
    show_line_numbers: Option<bool>,
    show_right_margin: Option<bool>,
    smart_backspace: Option<bool>,
    smart_home_end: Option<SmartHomeEndType>,
    tab_width: Option<u32>,
    accepts_tab: Option<bool>,
    bottom_margin: Option<i32>,
    buffer: Option<gtk::TextBuffer>,
    cursor_visible: Option<bool>,
    editable: Option<bool>,
    im_module: Option<String>,
    indent: Option<i32>,
    input_hints: Option<gtk::InputHints>,
    input_purpose: Option<gtk::InputPurpose>,
    justification: Option<gtk::Justification>,
    left_margin: Option<i32>,
    monospace: Option<bool>,
    overwrite: Option<bool>,
    pixels_above_lines: Option<i32>,
    pixels_below_lines: Option<i32>,
    pixels_inside_wrap: Option<i32>,
    populate_all: Option<bool>,
    right_margin: Option<i32>,
    tabs: Option<pango::TabArray>,
    top_margin: Option<i32>,
    wrap_mode: Option<gtk::WrapMode>,
    border_width: Option<u32>,
    child: Option<gtk::Widget>,
    resize_mode: Option<gtk::ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<gtk::Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<gtk::Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    hadjustment: Option<gtk::Adjustment>,
    hscroll_policy: Option<gtk::ScrollablePolicy>,
    vadjustment: Option<gtk::Adjustment>,
    vscroll_policy: Option<gtk::ScrollablePolicy>,
}

impl MapBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`MapBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Map`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Map {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref font_desc) = self.font_desc {
            properties.push(("font-desc", font_desc));
        }
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        if let Some(ref auto_indent) = self.auto_indent {
            properties.push(("auto-indent", auto_indent));
        }
        if let Some(ref background_pattern) = self.background_pattern {
            properties.push(("background-pattern", background_pattern));
        }
        if let Some(ref highlight_current_line) = self.highlight_current_line {
            properties.push(("highlight-current-line", highlight_current_line));
        }
        if let Some(ref indent_on_tab) = self.indent_on_tab {
            properties.push(("indent-on-tab", indent_on_tab));
        }
        if let Some(ref indent_width) = self.indent_width {
            properties.push(("indent-width", indent_width));
        }
        if let Some(ref insert_spaces_instead_of_tabs) = self.insert_spaces_instead_of_tabs {
            properties.push((
                "insert-spaces-instead-of-tabs",
                insert_spaces_instead_of_tabs,
            ));
        }
        if let Some(ref right_margin_position) = self.right_margin_position {
            properties.push(("right-margin-position", right_margin_position));
        }
        if let Some(ref show_line_marks) = self.show_line_marks {
            properties.push(("show-line-marks", show_line_marks));
        }
        if let Some(ref show_line_numbers) = self.show_line_numbers {
            properties.push(("show-line-numbers", show_line_numbers));
        }
        if let Some(ref show_right_margin) = self.show_right_margin {
            properties.push(("show-right-margin", show_right_margin));
        }
        if let Some(ref smart_backspace) = self.smart_backspace {
            properties.push(("smart-backspace", smart_backspace));
        }
        if let Some(ref smart_home_end) = self.smart_home_end {
            properties.push(("smart-home-end", smart_home_end));
        }
        if let Some(ref tab_width) = self.tab_width {
            properties.push(("tab-width", tab_width));
        }
        if let Some(ref accepts_tab) = self.accepts_tab {
            properties.push(("accepts-tab", accepts_tab));
        }
        if let Some(ref bottom_margin) = self.bottom_margin {
            properties.push(("bottom-margin", bottom_margin));
        }
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref cursor_visible) = self.cursor_visible {
            properties.push(("cursor-visible", cursor_visible));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref im_module) = self.im_module {
            properties.push(("im-module", im_module));
        }
        if let Some(ref indent) = self.indent {
            properties.push(("indent", indent));
        }
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        if let Some(ref justification) = self.justification {
            properties.push(("justification", justification));
        }
        if let Some(ref left_margin) = self.left_margin {
            properties.push(("left-margin", left_margin));
        }
        if let Some(ref monospace) = self.monospace {
            properties.push(("monospace", monospace));
        }
        if let Some(ref overwrite) = self.overwrite {
            properties.push(("overwrite", overwrite));
        }
        if let Some(ref pixels_above_lines) = self.pixels_above_lines {
            properties.push(("pixels-above-lines", pixels_above_lines));
        }
        if let Some(ref pixels_below_lines) = self.pixels_below_lines {
            properties.push(("pixels-below-lines", pixels_below_lines));
        }
        if let Some(ref pixels_inside_wrap) = self.pixels_inside_wrap {
            properties.push(("pixels-inside-wrap", pixels_inside_wrap));
        }
        if let Some(ref populate_all) = self.populate_all {
            properties.push(("populate-all", populate_all));
        }
        if let Some(ref right_margin) = self.right_margin {
            properties.push(("right-margin", right_margin));
        }
        if let Some(ref tabs) = self.tabs {
            properties.push(("tabs", tabs));
        }
        if let Some(ref top_margin) = self.top_margin {
            properties.push(("top-margin", top_margin));
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref hadjustment) = self.hadjustment {
            properties.push(("hadjustment", hadjustment));
        }
        if let Some(ref hscroll_policy) = self.hscroll_policy {
            properties.push(("hscroll-policy", hscroll_policy));
        }
        if let Some(ref vadjustment) = self.vadjustment {
            properties.push(("vadjustment", vadjustment));
        }
        if let Some(ref vscroll_policy) = self.vscroll_policy {
            properties.push(("vscroll-policy", vscroll_policy));
        }
        glib::Object::new::<Map>(&properties).expect("Failed to create an instance of Map")
    }

    pub fn font_desc(mut self, font_desc: &pango::FontDescription) -> Self {
        self.font_desc = Some(font_desc.clone());
        self
    }

    pub fn view(mut self, view: &impl IsA<View>) -> Self {
        self.view = Some(view.clone().upcast());
        self
    }

    pub fn auto_indent(mut self, auto_indent: bool) -> Self {
        self.auto_indent = Some(auto_indent);
        self
    }

    pub fn background_pattern(mut self, background_pattern: BackgroundPatternType) -> Self {
        self.background_pattern = Some(background_pattern);
        self
    }

    pub fn highlight_current_line(mut self, highlight_current_line: bool) -> Self {
        self.highlight_current_line = Some(highlight_current_line);
        self
    }

    pub fn indent_on_tab(mut self, indent_on_tab: bool) -> Self {
        self.indent_on_tab = Some(indent_on_tab);
        self
    }

    pub fn indent_width(mut self, indent_width: i32) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn insert_spaces_instead_of_tabs(mut self, insert_spaces_instead_of_tabs: bool) -> Self {
        self.insert_spaces_instead_of_tabs = Some(insert_spaces_instead_of_tabs);
        self
    }

    pub fn right_margin_position(mut self, right_margin_position: u32) -> Self {
        self.right_margin_position = Some(right_margin_position);
        self
    }

    pub fn show_line_marks(mut self, show_line_marks: bool) -> Self {
        self.show_line_marks = Some(show_line_marks);
        self
    }

    pub fn show_line_numbers(mut self, show_line_numbers: bool) -> Self {
        self.show_line_numbers = Some(show_line_numbers);
        self
    }

    pub fn show_right_margin(mut self, show_right_margin: bool) -> Self {
        self.show_right_margin = Some(show_right_margin);
        self
    }

    pub fn smart_backspace(mut self, smart_backspace: bool) -> Self {
        self.smart_backspace = Some(smart_backspace);
        self
    }

    pub fn smart_home_end(mut self, smart_home_end: SmartHomeEndType) -> Self {
        self.smart_home_end = Some(smart_home_end);
        self
    }

    pub fn tab_width(mut self, tab_width: u32) -> Self {
        self.tab_width = Some(tab_width);
        self
    }

    pub fn accepts_tab(mut self, accepts_tab: bool) -> Self {
        self.accepts_tab = Some(accepts_tab);
        self
    }

    pub fn bottom_margin(mut self, bottom_margin: i32) -> Self {
        self.bottom_margin = Some(bottom_margin);
        self
    }

    pub fn buffer(mut self, buffer: &impl IsA<gtk::TextBuffer>) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn cursor_visible(mut self, cursor_visible: bool) -> Self {
        self.cursor_visible = Some(cursor_visible);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn im_module(mut self, im_module: &str) -> Self {
        self.im_module = Some(im_module.to_string());
        self
    }

    pub fn indent(mut self, indent: i32) -> Self {
        self.indent = Some(indent);
        self
    }

    pub fn input_hints(mut self, input_hints: gtk::InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: gtk::InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }

    pub fn justification(mut self, justification: gtk::Justification) -> Self {
        self.justification = Some(justification);
        self
    }

    pub fn left_margin(mut self, left_margin: i32) -> Self {
        self.left_margin = Some(left_margin);
        self
    }

    pub fn monospace(mut self, monospace: bool) -> Self {
        self.monospace = Some(monospace);
        self
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    pub fn pixels_above_lines(mut self, pixels_above_lines: i32) -> Self {
        self.pixels_above_lines = Some(pixels_above_lines);
        self
    }

    pub fn pixels_below_lines(mut self, pixels_below_lines: i32) -> Self {
        self.pixels_below_lines = Some(pixels_below_lines);
        self
    }

    pub fn pixels_inside_wrap(mut self, pixels_inside_wrap: i32) -> Self {
        self.pixels_inside_wrap = Some(pixels_inside_wrap);
        self
    }

    pub fn populate_all(mut self, populate_all: bool) -> Self {
        self.populate_all = Some(populate_all);
        self
    }

    pub fn right_margin(mut self, right_margin: i32) -> Self {
        self.right_margin = Some(right_margin);
        self
    }

    pub fn tabs(mut self, tabs: &pango::TabArray) -> Self {
        self.tabs = Some(tabs.clone());
        self
    }

    pub fn top_margin(mut self, top_margin: i32) -> Self {
        self.top_margin = Some(top_margin);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: gtk::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: gtk::ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<gtk::Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn hadjustment(mut self, hadjustment: &impl IsA<gtk::Adjustment>) -> Self {
        self.hadjustment = Some(hadjustment.clone().upcast());
        self
    }

    pub fn hscroll_policy(mut self, hscroll_policy: gtk::ScrollablePolicy) -> Self {
        self.hscroll_policy = Some(hscroll_policy);
        self
    }

    pub fn vadjustment(mut self, vadjustment: &impl IsA<gtk::Adjustment>) -> Self {
        self.vadjustment = Some(vadjustment.clone().upcast());
        self
    }

    pub fn vscroll_policy(mut self, vscroll_policy: gtk::ScrollablePolicy) -> Self {
        self.vscroll_policy = Some(vscroll_policy);
        self
    }
}

pub trait MapExt: 'static {
    #[doc(alias = "gtk_source_map_get_view")]
    #[doc(alias = "get_view")]
    fn view(&self) -> Option<View>;

    #[doc(alias = "gtk_source_map_set_view")]
    fn set_view(&self, view: &impl IsA<View>);

    #[doc(alias = "font-desc")]
    fn font_desc(&self) -> Option<pango::FontDescription>;

    #[doc(alias = "font-desc")]
    fn set_font_desc(&self, font_desc: Option<&pango::FontDescription>);

    #[doc(alias = "font-desc")]
    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "view")]
    fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Map>> MapExt for O {
    fn view(&self) -> Option<View> {
        unsafe { from_glib_none(ffi::gtk_source_map_get_view(self.as_ref().to_glib_none().0)) }
    }

    fn set_view(&self, view: &impl IsA<View>) {
        unsafe {
            ffi::gtk_source_map_set_view(
                self.as_ref().to_glib_none().0,
                view.as_ref().to_glib_none().0,
            );
        }
    }

    fn font_desc(&self) -> Option<pango::FontDescription> {
        glib::ObjectExt::property(self.as_ref(), "font-desc")
    }

    fn set_font_desc(&self, font_desc: Option<&pango::FontDescription>) {
        glib::ObjectExt::set_property(self.as_ref(), "font-desc", &font_desc)
    }

    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<P: IsA<Map>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceMap,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Map::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P: IsA<Map>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceMap,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Map::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Map")
    }
}
