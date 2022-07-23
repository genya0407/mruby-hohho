use std::ffi::CString;

mod mrb_sys;

unsafe extern "C" fn hello(
    mrb: *mut mrb_sys::mrb_state,
    _slf: mrb_sys::mrb_value,
) -> mrb_sys::mrb_value {
    let cstr = CString::new("hello").unwrap();
    mrb_sys::mrb_str_new_cstr(mrb, cstr.as_ptr())
}

unsafe extern "C" fn hi(
    mrb: *mut mrb_sys::mrb_state,
    _slf: mrb_sys::mrb_value,
) -> mrb_sys::mrb_value {
    let cstr = CString::new("hi!!").unwrap();
    mrb_sys::mrb_str_new_cstr(mrb, cstr.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn mrb_mruby_hohho_gem_init(mrb: *mut mrb_sys::mrb_state) {
    let class_name = CString::new("Hohho").unwrap();
    let class: *mut mrb_sys::RClass =
        mrb_sys::mrb_define_class(mrb, class_name.as_ptr() as *const _, (*mrb).object_class);
    let method_name = CString::new("hello").unwrap();
    mrb_sys::mrb_define_method(
        mrb,
        class,
        method_name.as_ptr() as *const _,
        Some(hello),
        mrb_sys::MRB_ARGS_NONE(),
    );
    let method_name = CString::new("hi").unwrap();
    mrb_sys::mrb_define_class_method(
        mrb,
        class,
        method_name.as_ptr() as *const _,
        Some(hi),
        mrb_sys::MRB_ARGS_NONE(),
    );
}

#[no_mangle]
pub extern "C" fn mrb_mruby_hohho_gem_final(_mrb: *mut mrb_sys::mrb_state) {}
