# ljxacom
![workflow](https://github.com/zhxiaoyu/ljxacom/actions/workflows/build.yml/badge.svg)
![crates.io](https://img.shields.io/crates/v/ljxacom)
### Rust bindings for Keyence LJ-X8000A CommLib
## Usage
### Add ljxacom to your Cargo.toml
```toml
[dependencies]
ljxacom = "0.1.5"
```
### Example
```rust
fn main() {
    let mut ethernet_config = ljxacom::LJX8IF_ETHERNET_CONFIG {
        abyIpAddress: [192, 168, 0, 1],
        wPortNo: 24691,
        reserve: [0, 0],
    };
    let high_speed_port_no = 24692;
    let device_id = 0; // Set "0" if you use only 1 head.
    const X_IMAGE_SIZE: usize = 3200; // Number of X points.
    const Y_IMAGE_SIZE: usize = 1000; // Number of Y lines.
    let y_pitch_um = 20.0; // Data pitch of Y data. (e.g. your encoder setting)
    let timeout_ms = 5000; // Timeout value for the acquiring image (in milisecond).
    let use_external_batch_start = 0;

    let mut height_image = [0u16; X_IMAGE_SIZE * Y_IMAGE_SIZE];
    #[cfg(target_os = "windows")]
    let mut luminance_image = [0u16; X_IMAGE_SIZE * Y_IMAGE_SIZE];
    #[cfg(target_os = "linux")]
    let mut luminance_image = [0u8; X_IMAGE_SIZE * Y_IMAGE_SIZE];

    #[cfg(target_os = "windows")]
    let mut set_param = ljxacom::LJXA_ACQ_SETPARAM {
        y_linenum: Y_IMAGE_SIZE as i32,
        y_pitch_um: y_pitch_um,
        timeout_ms: timeout_ms,
        use_external_batchStart: use_external_batch_start,
    };
    #[cfg(target_os = "linux")]
    let mut set_param = ljxacom::LJXA_ACQ_SETPARAM {
        y_linenum: Y_IMAGE_SIZE as i32,
        interpolateLines: 0,
        y_pitch_um: y_pitch_um,
        timeout_ms: timeout_ms,
        use_external_batchStart: use_external_batch_start,
    };

    let mut get_param = ljxacom::LJXA_ACQ_GETPARAM {
        luminance_enabled: 0,
        x_pointnum: 0,
        y_linenum_acquired: 0,
        x_pitch_um: 0.0,
        y_pitch_um: 0.0,
        z_pitch_um: 0.0,
    };

    let result = ljxacom::initialize();
    println!("initialize result: 0x{:02X}", result);
    let result = ljxacom::open_device(device_id, &mut ethernet_config, high_speed_port_no);
    println!("open_device result: 0x{:02X}", result);
    let result = ljxacom::acquire(
        device_id,
        &mut height_image,
        &mut luminance_image,
        &mut set_param,
        &mut get_param,
    );
    println!("acquire result: 0x{:02X}", result);
    ljxacom::close_device(device_id);
    let result = ljxacom::finalize();
    println!("finalize result: 0x{:02X}", result);
}


```
## License
Distributed under the [MIT License](LICENSE).