pub const PP_ERR_NONE: u32 = 0; /* normal */
pub const PP_ERR_ARGUMENT: u32 = 1; /* parameter error */

pub const PROFILE_HEADER_SIZE_IN_INT: u32 = 6;
pub const BRIGHTNESS_VALUE: u8 = 0x40;

/* definition related to raw level packet */
pub const DEF_NOTIFY_MAGIC_NO: u32 = 0xFFFFFFFF;

pub const DEF_PROF_ALARM_20: u32 = 0x00080000; // 20bit : 0x00080000
pub const DEF_PROF_INVALID_20: u32 = 0x00080001; // 20bit : 0x00080001
pub const DEF_PROF_BLIND_20: u32 = 0x00080002; // 20bit : 0x00080002
pub const DEF_PROF_UNPROCESS_20: u32 = 0x00080003; // 20bit : 0x00080003

pub const DEF_PROF_ALARM: u32 = 0x80000000; // 32bit : 0x80000000
pub const DEF_PROF_INVALID: u32 = 0x80000001; // 32bit : 0x80000001
pub const DEF_PROF_BLIND: u32 = 0x80000002; // 32bit : 0x80000002
pub const DEF_PROF_UNPROCESS: u32 = 0x80000003; // 32bit : 0x80000003
pub const DEF_PROF_VALIDDATA_MIN: u32 = 0x80000004; // 32bit : 0x80000004

pub const MASK_PROFILE_COUNT: u8 = 0x60;
pub const PROFILE_CONVERT_UNIT: f64 = 8.0;

pub const BIT_PER_BYTE: i32 = 8;
pub const PROFILE_BIT_SIZE: i32 = 20;
pub const BRIGHTNESS_BIT_SIZE: i32 = 10;
pub const PACKING_PROFILE_DATA_SIZE_PER_BYTE: i32 = 4;

#[repr(C)]
pub struct Ljx8ifProfileHeader {
    reserve: u32,          // Reserved
    dw_trigger_count: u32, // The trigger count when the trigger was issued.
    l_encoder_count: i32,  // The encoder count when the trigger was issued.
    reserve2: [u32; 3],    // Reserved
}
#[repr(C)]
pub struct Ljx8ifProfileFooter {
    reserve: u32, // Reserved
}

pub fn convert_profile_data(
    data_count: u8,
    by_res_prof_type: u8,
    n_profile_data_margin: i32,
    w_prof_cnt: u16,
    w_unit: u16,
    p_in_data: &[u8],
) -> Vec<u32> {
    let is_brightness = BRIGHTNESS_VALUE & by_res_prof_type > 0;
    let n_multiple_value = match is_brightness {
        true => 2,
        false => 1,
    };
    let n_head_count = get_profile_count(by_res_prof_type);
    let n_profile_unit_size = w_prof_cnt * n_multiple_value * 4;
    let n_profile_data_size = n_profile_unit_size * n_head_count as u16;
    let n_profile_buffer_size = (f64::ceil(w_prof_cnt as f64 / PROFILE_CONVERT_UNIT)
        * PROFILE_CONVERT_UNIT
        * n_multiple_value as f64
        * 4.0
        * n_head_count as f64) as i32;
    let n_header_size = std::mem::size_of::<Ljx8ifProfileHeader>();
    let n_footer_size = std::mem::size_of::<Ljx8ifProfileFooter>();
    let n_profile_data_raw_size = match is_brightness {
        true => {
            size_of_bytes(
                (w_prof_cnt as i32 * PROFILE_BIT_SIZE) + (w_prof_cnt as i32 * BRIGHTNESS_BIT_SIZE),
                PACKING_PROFILE_DATA_SIZE_PER_BYTE,
            ) * n_head_count as i32
                * PACKING_PROFILE_DATA_SIZE_PER_BYTE
        }
        false => {
            size_of_bytes(
                w_prof_cnt as i32 * PROFILE_BIT_SIZE,
                PACKING_PROFILE_DATA_SIZE_PER_BYTE,
            ) * n_head_count as i32
                * PACKING_PROFILE_DATA_SIZE_PER_BYTE
        }
    };
    let p_out_size =
        (n_header_size + n_profile_data_size as usize + n_footer_size) * data_count as usize;
    let mut p_out: Vec<u8> = vec![0; p_out_size];
    let mut raw_data_cursor = 0;
    let mut p_out_cursor = 0;
    for _i in 0..data_count {
        let p_profile_raw_data = &p_in_data[raw_data_cursor..];
        println!("p_profile_raw_data length: {:?}", p_profile_raw_data.len());
        let a_profile_data =
            convert_profile_data_20_to_32(by_res_prof_type, w_prof_cnt, w_unit, p_profile_raw_data);
        let a_profile_data_u8 = unsafe {
            std::slice::from_raw_parts(
                a_profile_data.as_ptr() as *const u8,
                a_profile_data.len() * 4,
            )
        };

        // header
        p_out[p_out_cursor..p_out_cursor + n_header_size]
            .copy_from_slice(&p_profile_raw_data[..n_header_size]);
        p_out_cursor += n_header_size;

        // profile data
        p_out[p_out_cursor..p_out_cursor + n_profile_data_size as usize]
            .copy_from_slice(&a_profile_data_u8[..n_profile_data_size as usize]);
        p_out_cursor += n_profile_data_size as usize;

        // footer
        let footer_start = n_header_size + n_profile_data_raw_size as usize;
        p_out[p_out_cursor..p_out_cursor + n_footer_size]
            .copy_from_slice(&p_profile_raw_data[footer_start..footer_start + n_footer_size]);
        p_out_cursor += n_footer_size;

        raw_data_cursor += n_header_size
            + n_profile_data_size as usize
            + n_footer_size
            + n_profile_data_margin as usize;
    }
    p_out.chunks_exact(4)
        .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
        .collect::<Vec<u32>>()
}
pub fn get_profile_count(by_kind: u8) -> u8 {
    let by_temp = by_kind & (!MASK_PROFILE_COUNT);
    let mut by_profile_cnt = 0;

    for i in 0..BIT_PER_BYTE {
        if by_temp & (1 << i) != 0 {
            by_profile_cnt += 1;
        }
    }
    by_profile_cnt
}
fn convert_profile_data_20_to_32(
    profile_type: u8,
    w_prof_cnt: u16,
    w_unit: u16,
    profile_data: &[u8],
) -> Vec<i32> {
    let profile_data_u32: &[u32] = unsafe {
        std::slice::from_raw_parts(profile_data.as_ptr() as *const u32, profile_data.len() / 4)
    };
    let read_profile_data = &profile_data_u32[PROFILE_HEADER_SIZE_IN_INT as usize..];
    let mut profile_size = w_prof_cnt as usize;
    let profile_unit = w_unit as i32;
    let mut write_position;
    let mut read_position;
    let (brightness_offset, mut converted_data) = convert_20_to_32(profile_size, read_profile_data);
    println!("profile_unit: {:?}", profile_unit);
    println!("profile_size: {:?}", profile_size);
    for i in 0..profile_size {
        match converted_data[i] {
            DEF_PROF_ALARM_20 => converted_data[i] = DEF_PROF_ALARM,
            DEF_PROF_INVALID_20 => converted_data[i] = DEF_PROF_INVALID,
            DEF_PROF_BLIND_20 => converted_data[i] = DEF_PROF_BLIND,
            DEF_PROF_UNPROCESS_20 => converted_data[i] = DEF_PROF_UNPROCESS,
            _ => {
                // 扩展符号
                if 0x00080000 & converted_data[i] != 0 {
                    converted_data[i] |= 0xFFF00000;
                }
                // 转换为0.01μm单位的值
                converted_data[i] = converted_data[i].wrapping_mul(profile_unit as u32);
            }
        }
    }
    let is_brightness = (profile_type & BRIGHTNESS_VALUE) == BRIGHTNESS_VALUE;
    if is_brightness {
        converted_data.resize(converted_data.len() * 2, 0);
        let mut read_brightness_data = &read_profile_data[brightness_offset..];
        let mut converted_brightness_data = &mut converted_data[profile_size..];
        let is_profile_count_300 = profile_size % 8 != 0;
        if is_profile_count_300 {
            read_position = 0;
            write_position = 0;
            read_brightness_data = &read_profile_data[brightness_offset - 1..];
            converted_brightness_data[write_position + 0] =
                (0xFC000000 & read_brightness_data[read_position + 0]) >> 26
                    | (0x0000000F & read_brightness_data[read_position + 1]) << 6;
            converted_brightness_data[write_position + 1] =
                (0x03FF0000 & read_brightness_data[read_position + 0]) >> 16;
            converted_brightness_data[write_position + 2] =
                (0x00FFC000 & read_brightness_data[read_position + 1]) >> 14;
            converted_brightness_data[write_position + 3] =
                (0x00003FF0 & read_brightness_data[read_position + 1]) >> 4;
            converted_brightness_data[write_position + 4] =
                (0x00000FFC & read_brightness_data[read_position + 2]) >> 2;
            converted_brightness_data[write_position + 5] =
                (0xFF000000 & read_brightness_data[read_position + 1]) >> 24
                    | (0x00000003 & read_brightness_data[read_position + 2]) << 8;
            converted_brightness_data[write_position + 6] =
                (0xFFC00000 & read_brightness_data[read_position + 2]) >> 22;
            converted_brightness_data[write_position + 7] =
                (0x003FF000 & read_brightness_data[read_position + 2]) >> 12;
            read_brightness_data = &read_profile_data[brightness_offset + 3..];
            let half_profile_data = 16 / 2;
            converted_brightness_data = &mut converted_data[profile_size + half_profile_data..];
            profile_size -= half_profile_data;
        }
        read_position = 0;
        write_position = 0;
        while write_position < profile_size {
            converted_brightness_data[write_position + 0] =
                (0x000FFC00 & read_brightness_data[read_position + 0]) >> 10;
            if profile_size <= write_position + 1 {
                break;
            }
            converted_brightness_data[write_position + 1] =
                0x000003FF & read_brightness_data[read_position + 0];
            if profile_size <= write_position + 2 {
                break;
            }
            converted_brightness_data[write_position + 2] =
                (0xC0000000 & read_brightness_data[read_position + 0]) >> 30
                    | (0x000000FF & read_brightness_data[read_position + 1]) << 2;
            if profile_size <= write_position + 3 {
                break;
            }
            converted_brightness_data[write_position + 3] =
                (0x3FF00000 & read_brightness_data[read_position + 0]) >> 20;
            if profile_size <= write_position + 4 {
                break;
            }
            converted_brightness_data[write_position + 4] =
                (0x0FFC0000 & read_brightness_data[read_position + 1]) >> 18;
            if profile_size <= write_position + 5 {
                break;
            }
            converted_brightness_data[write_position + 5] =
                (0x0003FF00 & read_brightness_data[read_position + 1]) >> 8;
            if profile_size <= write_position + 6 {
                break;
            }
            converted_brightness_data[write_position + 6] =
                (0x0000FFC0 & read_brightness_data[read_position + 2]) >> 6;
            if profile_size <= write_position + 7 {
                break;
            }
            converted_brightness_data[write_position + 7] =
                (0xF0000000 & read_brightness_data[read_position + 1]) >> 28
                    | (0x0000003F & read_brightness_data[read_position + 2]) << 4;
            if profile_size <= write_position + 8 {
                break;
            }
            converted_brightness_data[write_position + 8] =
                (0xFC000000 & read_brightness_data[read_position + 2]) >> 26
                    | (0x0000000F & read_brightness_data[read_position + 3]) << 6;
            if profile_size <= write_position + 9 {
                break;
            }
            converted_brightness_data[write_position + 9] =
                (0x03FF0000 & read_brightness_data[read_position + 2]) >> 16;
            if profile_size <= write_position + 10 {
                break;
            }
            converted_brightness_data[write_position + 10] =
                (0x00FFC000 & read_brightness_data[read_position + 3]) >> 14;
            if profile_size <= write_position + 11 {
                break;
            }
            converted_brightness_data[write_position + 11] =
                (0x00003FF0 & read_brightness_data[read_position + 3]) >> 4;
            if profile_size <= write_position + 12 {
                break;
            }
            converted_brightness_data[write_position + 12] =
                (0x00000FFC & read_brightness_data[read_position + 4]) >> 2;
            if profile_size <= write_position + 13 {
                break;
            }
            converted_brightness_data[write_position + 13] =
                (0xFF000000 & read_brightness_data[read_position + 3]) >> 24
                    | (0x00000003 & read_brightness_data[read_position + 4]) << 8;
            if profile_size <= write_position + 14 {
                break;
            }
            converted_brightness_data[write_position + 14] =
                (0xFFC00000 & read_brightness_data[read_position + 4]) >> 22;
            if profile_size <= write_position + 15 {
                break;
            }
            converted_brightness_data[write_position + 15] =
                (0x003FF000 & read_brightness_data[read_position + 4]) >> 12;
            if profile_size <= write_position + 16 {
                break;
            }
            read_position += 5;
            write_position += 16;
        }
    }
    converted_data.into_iter().map(|x| x as i32).collect()
}
fn convert_20_to_32(data_size: usize, read_data: &[u32]) -> (usize, Vec<u32>) {
    let mut read_position = 0;
    let mut write_position = 0;
    let mut converted_data = vec![0; data_size as usize];
    let mut brightness_offset = 0;
    while write_position < data_size {
        converted_data[write_position as usize] = 0x000FFFFF & read_data[read_position as usize];
        if data_size <= write_position + 1 {
            brightness_offset = read_position + 1;
            break;
        }
        converted_data[(write_position + 1) as usize] =
            ((0xFFF00000 & read_data[read_position as usize]) >> 20)
                | ((0x000000FF & read_data[(read_position + 1) as usize]) << 12);
        if data_size <= write_position + 2 {
            brightness_offset = read_position + 2;
            break;
        }
        converted_data[(write_position + 2) as usize] =
            (0x0FFFFF00 & read_data[(read_position + 1) as usize]) >> 8;
        if data_size <= write_position + 3 {
            brightness_offset = read_position + 2;
            break;
        }
        converted_data[(write_position + 3) as usize] =
            ((0xF0000000 & read_data[(read_position + 1) as usize]) >> 28)
                | ((0x0000FFFF & read_data[(read_position + 2) as usize]) << 4);
        if data_size <= write_position + 4 {
            brightness_offset = read_position + 3;
            break;
        }
        converted_data[(write_position + 4) as usize] =
            ((0xFFFF0000 & read_data[(read_position + 2) as usize]) >> 16)
                | ((0x0000000F & read_data[(read_position + 3) as usize]) << 16);
        if data_size <= write_position + 5 {
            brightness_offset = read_position + 4;
            break;
        }
        converted_data[(write_position + 5) as usize] =
            (0x00FFFFF0 & read_data[(read_position + 3) as usize]) >> 4;
        if data_size <= write_position + 6 {
            brightness_offset = read_position + 4;
            break;
        }
        converted_data[(write_position + 6) as usize] =
            ((0xFF000000 & read_data[(read_position + 3) as usize]) << 24)
                | ((0x00000FFF & read_data[(read_position + 4) as usize]) << 8);
        if data_size <= write_position + 7 {
            brightness_offset = read_position + 5;
            break;
        }
        converted_data[(write_position + 7) as usize] =
            (0xFFFFF000 & read_data[(read_position + 4) as usize]) >> 12;
        if data_size <= write_position + 8 {
            brightness_offset = read_position + 5;
            break;
        }
        read_position += 5;
        write_position += 8;
    }
    return (brightness_offset, converted_data);
}
fn size_of_bytes(size: i32, bytes: i32) -> i32 {
    (size + ((bytes * 8) - 1)) / (bytes * 8)
}
