/* automatically generated by rust-bindgen */

pub const FPS_UPPER_LIMIT: u32 = 200;
pub const FPS_LOWER_LIMIT: u32 = 1;
pub const FPS_DEFAULT: u32 = 30;
pub type __uint32_t = core::ffi::c_uint;
pub type Uint32 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPSmanager {
    pub framecount: Uint32,
    pub rateticks: f32,
    pub baseticks: Uint32,
    pub lastticks: Uint32,
    pub rate: Uint32,
}
#[test]
fn bindgen_test_layout_FPSmanager() {
    assert_eq!(
        ::core::mem::size_of::<FPSmanager>(),
        20usize,
        concat!("Size of: ", stringify!(FPSmanager))
    );
    assert_eq!(
        ::core::mem::align_of::<FPSmanager>(),
        4usize,
        concat!("Alignment of ", stringify!(FPSmanager))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FPSmanager>())).framecount as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FPSmanager),
            "::",
            stringify!(framecount)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FPSmanager>())).rateticks as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FPSmanager),
            "::",
            stringify!(rateticks)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FPSmanager>())).baseticks as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FPSmanager),
            "::",
            stringify!(baseticks)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FPSmanager>())).lastticks as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(FPSmanager),
            "::",
            stringify!(lastticks)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FPSmanager>())).rate as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FPSmanager),
            "::",
            stringify!(rate)
        )
    );
}
extern "C" {
    pub fn SDL_initFramerate(manager: *mut FPSmanager);
}
extern "C" {
    pub fn SDL_setFramerate(manager: *mut FPSmanager, rate: Uint32) -> core::ffi::c_int;
}
extern "C" {
    pub fn SDL_getFramerate(manager: *mut FPSmanager) -> core::ffi::c_int;
}
extern "C" {
    pub fn SDL_getFramecount(manager: *mut FPSmanager) -> core::ffi::c_int;
}
extern "C" {
    pub fn SDL_framerateDelay(manager: *mut FPSmanager) -> Uint32;
}
