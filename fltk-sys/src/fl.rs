/* automatically generated by rust-bindgen 0.56.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget_Tracker {
    _unused: [u8; 0],
}
pub type Fl_Awake_Handler =
    ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Fl_run() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_check() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ready() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_release();
}
extern "C" {
    pub fn Fl_reload_scheme() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_menu_linespacing() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_menu_linespacing(H: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_lock() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_unlock();
}
extern "C" {
    pub fn Fl_awake_callback(
        handler: Fl_Awake_Handler,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_awake();
}
extern "C" {
    pub fn Fl_set_scrollbar_size(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_scrollbar_size() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_original_key() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_key_down(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_text() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_event_button() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_clicks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_x() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_y() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_x_root() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_y_root() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_dx() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_dy() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_get_mouse(arg1: *mut ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_event_is_click() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_length() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_state() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_x() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_y() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_h() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_w() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_compose(del: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_compose_reset();
}
extern "C" {
    pub fn Fl_compose_state() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_reset_marked_text();
}
extern "C" {
    pub fn Fl_insertion_point_location(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_copy(
        stuff: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        destination: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_paste_text(arg1: *mut Fl_Widget, src: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_paste_image(widget: *mut Fl_Widget, src: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_set_scheme(scheme: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_scheme() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_scheme_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_visible_focus() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_visible_focus(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_set_box_type(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_box_shadow_width() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_box_shadow_width(W: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_box_border_radius_max() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_box_border_radius_max(R: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_get_rgb_color(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_set_color(
        c: ::std::os::raw::c_uint,
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_font(idx: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_get_font_name(idx: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_get_font_sizes(
        font: ::std::os::raw::c_int,
        sizep: *mut *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_fonts(c: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_font(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_set_font2(arg1: ::std::os::raw::c_int, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_set_font_size(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_font_size() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_add_handler(
        ev_handler: ::std::option::Option<
            unsafe extern "C" fn(ev: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_awake_msg(msg: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_thread_msg() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_wait() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_wait_for(arg1: f64) -> f64;
}
extern "C" {
    pub fn Fl_add_timeout(
        t: f64,
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_repeat_timeout(
        t: f64,
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_timeout(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_has_timeout(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_dnd() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_grab() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_set_grab(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_first_window() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_next_window(arg1: *const ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_should_program_quit() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_program_should_quit(flag: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_event_inside(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_belowmouse() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_delete_widget(w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_Tracker_new(w: *mut Fl_Widget) -> *mut Fl_Widget_Tracker;
}
extern "C" {
    pub fn Fl_Widget_Tracker_deleted(self_: *mut Fl_Widget_Tracker) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Widget_Tracker_delete(self_: *mut Fl_Widget_Tracker);
}
extern "C" {
    pub fn Fl_init_all();
}
extern "C" {
    pub fn Fl_redraw();
}
extern "C" {
    pub fn Fl_event_shift() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_ctrl() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_command() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_alt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_damage(flag: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_damage() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_visual(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_own_colormap();
}
extern "C" {
    pub fn Fl_pushed() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_focus() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_set_focus(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_version() -> f64;
}
extern "C" {
    pub fn Fl_api_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_abi_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_load_font(path: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_unload_font(path: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_foreground(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_background(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_background2(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_selection_color(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_inactive_color(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_system_colors();
}
extern "C" {
    pub fn Fl_handle(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_handle_(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_add_idle(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_has_idle(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_remove_idle(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_flush();
}
extern "C" {
    pub fn Fl_set_screen_scale(n: ::std::os::raw::c_int, val: f32);
}
extern "C" {
    pub fn Fl_screen_scale(n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn Fl_screen_scaling_supported() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_count() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_num(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_screen_xywh(
        X: *mut ::std::os::raw::c_int,
        Y: *mut ::std::os::raw::c_int,
        W: *mut ::std::os::raw::c_int,
        H: *mut ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_dpi(h: *mut f32, v: *mut f32, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_screen_work_area(
        X: *mut ::std::os::raw::c_int,
        Y: *mut ::std::os::raw::c_int,
        W: *mut ::std::os::raw::c_int,
        H: *mut ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_open_display();
}
extern "C" {
    pub fn Fl_close_display();
}
extern "C" {
    pub fn Fl_box_dx(boxtype: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_box_dy(boxtype: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_box_dw(boxtype: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_box_dh(boxtype: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_mac_os_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_clipboard() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_event_clipboard_type() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_clipboard_contains(type_: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_event_dispatch(
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                event: ::std::os::raw::c_int,
                arg1: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_inactive(c: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_lighter(c: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_darker(c: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_set_box_type_cb(
        arg1: ::std::os::raw::c_int,
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_int,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
                arg5: ::std::os::raw::c_uint,
            ),
        >,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box_active() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_gray_ramp(i: ::std::os::raw::c_int) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_color_average(
        arg1: ::std::os::raw::c_uint,
        arg2: ::std::os::raw::c_uint,
        f: f32,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_contrast(
        c1: ::std::os::raw::c_uint,
        c2: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color2(g: ::std::os::raw::c_uchar) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_cmap(c: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_box_color(c: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_set_box_color(c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_add_system_handler(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_system_handler(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_gl_visual(mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_add_clipboard_notify(
        cb: ::std::option::Option<
            unsafe extern "C" fn(source: ::std::os::raw::c_int, data: *mut ::std::os::raw::c_void),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_clipboard_notify(
        cb: ::std::option::Option<
            unsafe extern "C" fn(source: ::std::os::raw::c_int, data: *mut ::std::os::raw::c_void),
        >,
    );
}
