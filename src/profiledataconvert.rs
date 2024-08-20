pub const PP_ERR_NONE: u32 = 0; /* normal */
pub const PP_ERR_ARGUMENT: u32 = 1; /* parameter error */

pub const PROFILE_HEADER_SIZE_IN_INT: u32 = 6;
pub const BRIGHTNESS_VALUE: u32 = 0x40;

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

pub fn convert_profile_data_20_to_32(
    profile_type: u8,
    w_prof_cnt: u16,
    w_unit: u16,
    profile_data: &[u32],
    converted_z_profile: &mut [u32],
) -> u8 {
    if profile_data.is_empty() || converted_z_profile.is_empty() {
        return PP_ERR_ARGUMENT;
    }

    let mut read_profile_data = unsafe {
        std::slice::from_raw_parts(
            profile_data.as_ptr().add(PROFILE_HEADER_SIZE_IN_INT * 4) as *const u32,
            profile_data.len() / 4 - PROFILE_HEADER_SIZE_IN_INT,
        )
    };

    let profile_size = w_prof_cnt as usize;
    let profile_unit = w_unit as i32;

    let mut brightness_offset: u32 = 0;

    // 将20位数据转换为32位数据
    convert_20_to_32(
        profile_size,
        read_profile_data,
        &mut read_position,
        &mut brightness_offset,
        converted_z_profile,
    );

    for i in 0..profile_size {
        // 转换异常值
        converted_z_profile[i] = match converted_z_profile[i] {
            DEF_PROF_ALARM_20 => DEF_PROF_ALARM,
            DEF_PROF_INVALID_20 => DEF_PROF_INVALID,
            DEF_PROF_BLIND_20 => DEF_PROF_BLIND,
            DEF_PROF_UNPROCESS_20 => DEF_PROF_UNPROCESS,
            _ => {
                // 扩展符号
                let mut value = converted_z_profile[i];
                if value & 0x00080000 != 0 {
                    value |= 0xFFF00000;
                }
                // 转换为0.01μm单位的值
                value * profile_unit
            }
        };
    }

    let is_brightness = (profile_type & BRIGHTNESS_VALUE) == BRIGHTNESS_VALUE;
    if is_brightness {
        // 偏移指针到亮度配置文件数据存储的位置
        read_profile_data = &read_profile_data[brightness_offset..];
        let converted_profile_data = &mut converted_z_profile[profile_size..];

        // 如果配置文件数量不能被8整除（例如300），则先转换余数
        let is_profile_count_300 = profile_size % 8 != 0;
        if is_profile_count_300 {
            let mut read_position = 0;
            let mut write_position = 0;
            
            // 处理剩余的亮度配置文件
            read_profile_data = read_profile_data.offset(-1);
        
            converted_profile_data[write_position + 0] = ((0xFC000000 & read_profile_data[read_position + 0]) >> 26 | (0x0000000F & read_profile_data[read_position + 1]) << 6) as u32;
            converted_profile_data[write_position + 1] = ((0x03FF0000 & read_profile_data[read_position + 0]) >> 16) as u32;
            converted_profile_data[write_position + 2] = ((0x00FFC000 & read_profile_data[read_position + 1]) >> 14) as u32;
            converted_profile_data[write_position + 3] = ((0x00003FF0 & read_profile_data[read_position + 1]) >> 4) as u32;
            converted_profile_data[write_position + 4] = ((0x00000FFC & read_profile_data[read_position + 2]) >> 2) as u32;
            converted_profile_data[write_position + 5] = ((0xFF000000 & read_profile_data[read_position + 1]) >> 24 | (0x00000003 & read_profile_data[read_position + 2]) << 8) as u32;
            converted_profile_data[write_position + 6] = ((0xFFC00000 & read_profile_data[read_position + 2]) >> 22) as u32;
            converted_profile_data[write_position + 7] = ((0x003FF000 & read_profile_data[read_position + 2]) >> 12) as u32;
            
            read_profile_data = read_profile_data.offset(3);
        
            const HALF_PROFILE_DATA: usize = 16 / 2;
            converted_profile_data = &mut converted_profile_data[HALF_PROFILE_DATA..];
            profile_size -= HALF_PROFILE_DATA;
        }

        let mut read_position = 0;
        let mut write_position = 0;

        while write_position < profile_size {
            // ... (这里需要实现主要的亮度数据转换逻辑)
            // 由于Rust的位操作和C++略有不同，可能需要进行一些调整
        }
    }

    PP_ERR_NONE
}
pub fn convert_profile_data_32_to_16_as_simple_array(
    pdw_batch_data: &[i32],
    profile_size: u16,
    with_luminance: bool,
    get_profile_count: u32,
    z_unit: u16,
    p_profile_header_array: &mut [LJX8IF_PROFILE_HEADER],
    p_height_profile_array: &mut [u16],
    p_luminance_profile_array: Option<&mut [u16]>,
) {
    let coefficient = z_unit as i32 * 8;

    let header_size = mem::size_of::<LJX8IF_PROFILE_HEADER>() / mem::size_of::<i32>();
    let footer_size = mem::size_of::<LJX8IF_PROFILE_FOOTER>() / mem::size_of::<i32>();

    let header_byte_size = mem::size_of::<LJX8IF_PROFILE_HEADER>();

    let mut source = pdw_batch_data;
    for i in 0..get_profile_count {
        let profile_index_base = profile_size as usize * i as usize;

        // Header
        let header_slice =
            unsafe { slice::from_raw_parts(source.as_ptr() as *const u8, header_byte_size) };
        p_profile_header_array[i as usize]
            .copy_from_slice(unsafe { &*(header_slice.as_ptr() as *const LJX8IF_PROFILE_HEADER) });
        source = &source[header_size..];

        // Height data
        for k in 0..profile_size as usize {
            let data = source[k] as i64;
            if data < DEF_PROF_VALIDDATA_MIN {
                // Irregular data is converted to zero.
                p_height_profile_array[profile_index_base + k] = 0;
                continue;
            }

            let value = data / coefficient as i64;
            if value < i16::MIN as i64 || value > i16::MAX as i64 {
                // Height data values from long-distance heads can exceed the 16-bit range,
                p_height_profile_array[profile_index_base + k] = 0;
                continue;
            }

            // To convert signed 16-bit data to unsigned data, offset the data by 32768
            p_height_profile_array[profile_index_base + k] = (value + i16::MAX as i64 + 1) as u16;
        }
        source = &source[profile_size as usize..];

        // Luminance data
        if with_luminance {
            if let Some(p_luminance_profile_array) = p_luminance_profile_array {
                for k in 0..profile_size as usize {
                    p_luminance_profile_array[profile_index_base + k] = source[k] as u16;
                }
            }
            source = &source[profile_size as usize..];
        }

        // Footer
        source = &source[footer_size..];
    }
}

pub fn convert_profile_data(
    data_count: u8,
    by_res_prof_type: u8,
    n_profile_data_margin: i32,
    w_prof_cnt: u16,
    w_unit: u16,
    p_in_data: &[u8],
    p_out_z_prof: &mut [u8],
) -> Result<(), &'static str> {
    // 计算配置文件大小
    let is_brightness = (BRIGHTNESS_VALUE & by_res_prof_type) > 0;
    let n_multiple_value = if is_brightness { 2 } else { 1 };
    let n_head_count = Self::get_profile_count(by_res_prof_type);

    let n_profile_unit_size = w_prof_cnt as usize * n_multiple_value * size_of::<u32>();
    let n_profile_data_size = n_profile_unit_size * n_head_count as usize;

    // 由于转换以8为单位执行，数据大小准备为8的倍数（向上取整）
    let n_profile_buffer_size = ((w_prof_cnt as f64 / PROFILE_CONVERT_UNIT).ceil()
        * PROFILE_CONVERT_UNIT
        * n_multiple_value as f64
        * size_of::<u32>() as f64
        * n_head_count as f64) as usize;

    let n_header_size = size_of::<LJX8IF_PROFILE_HEADER>();
    let n_footer_size = size_of::<LJX8IF_PROFILE_FOOTER>();

    // 检查用户缓冲区大小是否足够
    if p_out_z_prof.len()
        < (n_profile_data_size + n_header_size + n_footer_size) * data_count as usize
    {
        return Err("LJX8IF_RC_ERR_BUFFER_SHORT");
    }

    let n_profile_data_raw_size = if is_brightness {
        size_of_bytes(
            (w_prof_cnt as usize * PROFILE_BIT_SIZE) + (w_prof_cnt as usize * BRIGHTNESS_BIT_SIZE),
            PACKING_PROFILE_DATA_SIZE_PER_BYTE,
        ) * n_head_count as usize
            * PACKING_PROFILE_DATA_SIZE_PER_BYTE
    } else {
        size_of_bytes(
            (w_prof_cnt as usize * PROFILE_BIT_SIZE),
            PACKING_PROFILE_DATA_SIZE_PER_BYTE,
        ) * n_head_count as usize
            * PACKING_PROFILE_DATA_SIZE_PER_BYTE
    };

    let mut p_profile_raw_data = p_in_data;
    let mut p_out = p_out_z_prof;

    for _ in 0..data_count {
        // 一个配置文件的临时缓冲区
        let mut a_profile_data = vec![0u8; n_profile_buffer_size];

        // 20位到32位转换
        if Self::convert_profile_data_20to32(
            by_res_prof_type,
            w_prof_cnt,
            w_unit,
            p_profile_raw_data,
            &mut a_profile_data,
        )? != PP_ERR_NONE
        {
            return Err("LJX8IF_RC_ERR_PARAMETER");
        }

        // 头部
        p_out[..n_header_size].copy_from_slice(&p_profile_raw_data[..n_header_size]);
        p_out = &mut p_out[n_header_size..];

        // 配置文件数据
        p_out[..n_profile_data_size].copy_from_slice(&a_profile_data[..n_profile_data_size]);
        p_out = &mut p_out[n_profile_data_size..];

        // 尾部
        let footer_start = n_header_size + n_profile_data_raw_size;
        p_out[..n_footer_size]
            .copy_from_slice(&p_profile_raw_data[footer_start..footer_start + n_footer_size]);
        p_out = &mut p_out[n_footer_size..];

        p_profile_raw_data = &p_profile_raw_data[n_header_size
            + n_profile_data_raw_size
            + n_footer_size
            + n_profile_data_margin as usize..];
    }

    Ok(())
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

fn convert_20_to_32(
    data_size: u32,
    read_data: &[u32],
    read_position: &mut u32,
    brightness_offset: &mut u32,
    converted_data: &mut [u32],
) {
    let mut write_position = 0;

    while write_position < data_size {
        converted_data[write_position as usize] = 0x000FFFFF & read_data[*read_position as usize];
        if data_size <= write_position + 1 {
            *brightness_offset = *read_position + 1;
            break;
        }

        converted_data[(write_position + 1) as usize] =
            ((0xFFF00000 & read_data[*read_position as usize]) >> 20)
                | ((0x000000FF & read_data[(*read_position + 1) as usize]) << 12);
        if data_size <= write_position + 2 {
            *brightness_offset = *read_position + 2;
            break;
        }

        converted_data[(write_position + 2) as usize] =
            (0x0FFFFF00 & read_data[(*read_position + 1) as usize]) >> 8;
        if data_size <= write_position + 3 {
            *brightness_offset = *read_position + 2;
            break;
        }

        converted_data[(write_position + 3) as usize] =
            ((0xF0000000 & read_data[(*read_position + 1) as usize]) >> 28)
                | ((0x0000FFFF & read_data[(*read_position + 2) as usize]) << 4);
        if data_size <= write_position + 4 {
            *brightness_offset = *read_position + 3;
            break;
        }

        converted_data[(write_position + 4) as usize] =
            ((0xFFFF0000 & read_data[(*read_position + 2) as usize]) >> 16)
                | ((0x0000000F & read_data[(*read_position + 3) as usize]) << 16);
        if data_size <= write_position + 5 {
            *brightness_offset = *read_position + 4;
            break;
        }

        converted_data[(write_position + 5) as usize] =
            (0x00FFFFF0 & read_data[(*read_position + 3) as usize]) >> 4;
        if data_size <= write_position + 6 {
            *brightness_offset = *read_position + 4;
            break;
        }

        converted_data[(write_position + 6) as usize] =
            ((0xFF000000 & read_data[(*read_position + 3) as usize]) >> 24)
                | ((0x00000FFF & read_data[(*read_position + 4) as usize]) << 8);
        if data_size <= write_position + 7 {
            *brightness_offset = *read_position + 5;
            break;
        }

        converted_data[(write_position + 7) as usize] =
            (0xFFFFF000 & read_data[(*read_position + 4) as usize]) >> 12;
        if data_size <= write_position + 8 {
            *brightness_offset = *read_position + 5;
            break;
        }

        *read_position += 5;
        write_position += 8;
    }
}
