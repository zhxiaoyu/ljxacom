#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn initialize() -> std::os::raw::c_long {
    unsafe { LJX8IF_Initialize() }
}
pub fn finalize() -> std::os::raw::c_long {
    unsafe { LJX8IF_Finalize() }
}
pub fn open_device(
    device_id: i32,
    ethernet_config: &mut LJX8IF_ETHERNET_CONFIG,
    highspeed_port_no: i32,
) -> std::os::raw::c_long {
    unsafe { LJXA_ACQ_OpenDevice(device_id, ethernet_config, highspeed_port_no) }
}
pub fn close_device(device_id: i32) {
    unsafe { LJXA_ACQ_CloseDevice(device_id) }
}
#[cfg(target_os = "windows")]
pub fn acquire(
    device_id: i32,
    height_image: &mut [u16],
    luminance_image: &mut [u16],
    set_param: &mut LJXA_ACQ_SETPARAM,
    get_param: &mut LJXA_ACQ_GETPARAM,
) -> std::os::raw::c_long {
    unsafe {
        LJXA_ACQ_Acquire(
            device_id,
            height_image.as_mut_ptr(),
            luminance_image.as_mut_ptr(),
            set_param,
            get_param,
        )
    }
}
#[cfg(target_os = "linux")]
pub fn acquire(
    device_id: i32,
    height_image: &mut [u16],
    luminance_image: &mut [u8],
    set_param: &mut LJXA_ACQ_SETPARAM,
    get_param: &mut LJXA_ACQ_GETPARAM,
) -> std::os::raw::c_long {
    unsafe {
        LJXA_ACQ_Acquire(
            device_id,
            height_image.as_mut_ptr(),
            luminance_image.as_mut_ptr(),
            set_param,
            get_param,
        )
    }
}
pub fn acquire_start_async(device_id: i32, set_param: &mut LJXA_ACQ_SETPARAM) -> std::os::raw::c_long {
    unsafe { LJXA_ACQ_StartAsync(device_id, set_param) }
}
#[cfg(target_os = "windows")]
pub fn acquire_async(
    device_id: i32,
    height_image: &mut [u16],
    luminance_image: &mut [u16],
    set_param: &mut LJXA_ACQ_SETPARAM,
    get_param: &mut LJXA_ACQ_GETPARAM,
) -> std::os::raw::c_long {
    unsafe {
        LJXA_ACQ_AcquireAsync(
            device_id,
            height_image.as_mut_ptr(),
            luminance_image.as_mut_ptr(),
            set_param,
            get_param,
        )
    }
}
#[cfg(target_os = "linux")]
pub fn acquire_async(
    device_id: i32,
    height_image: &mut [u16],
    luminance_image: &mut [u8],
    set_param: &mut LJXA_ACQ_SETPARAM,
    get_param: &mut LJXA_ACQ_GETPARAM,
) -> std::os::raw::c_long {
    unsafe {
        LJXA_ACQ_AcquireAsync(
            device_id,
            height_image.as_mut_ptr(),
            luminance_image.as_mut_ptr(),
            set_param,
            get_param,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = initialize();
        assert_eq!(result, 0);
        let result = finalize();
        assert_eq!(result, 0);
    }
}
