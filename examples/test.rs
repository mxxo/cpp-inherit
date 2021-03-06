/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct non_virtual {
    pub value: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}_ZN11non_virtual1xEv"]
    pub fn non_virtual_x(this: *mut non_virtual) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZN11non_virtualC1Ei"]
    pub fn non_virtual_non_virtual(this: *mut non_virtual, v: ::std::os::raw::c_int);
}
impl non_virtual {
    #[inline]
    pub unsafe fn x(&mut self) -> ::std::os::raw::c_int {
        non_virtual_x(self)
    }
    #[inline]
    pub unsafe fn new(v: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        non_virtual_non_virtual(__bindgen_tmp.as_mut_ptr(), v);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
pub struct base__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct base {
    pub vtable_: *const base__bindgen_vtable,
    pub value: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}_ZN4baseC1Ei"]
    pub fn base_base(this: *mut base, v: ::std::os::raw::c_int);
}
impl base {
    #[inline]
    pub unsafe fn new(v: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        base_base(__bindgen_tmp.as_mut_ptr(), v);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN4base1xEv"]
    pub fn base_x(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct derived {
    pub _base: base,
}
extern "C" {
    #[link_name = "\u{1}_ZN7derivedC1Ei"]
    pub fn derived_derived(this: *mut derived, v: ::std::os::raw::c_int);
}
impl derived {
    #[inline]
    pub unsafe fn new(v: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        derived_derived(__bindgen_tmp.as_mut_ptr(), v);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN7derived1xEv"]
    pub fn derived_x(this: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_Z9call_x_onP4base"]
    pub fn call_x_on(x: *mut base) -> ::std::os::raw::c_int;
}
