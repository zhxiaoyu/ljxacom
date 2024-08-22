include!("profiledataconvert.rs");
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};
const MASK_CURRENT_BATCH_COMMITED: u32 = 0x80000000;
pub struct LJX8If {
    stream: TcpStream,
}
pub struct Ljx8ifGetProfileRequest {
    pub by_target_bank: u8,
    pub by_position_mode: u8,
    pub reserve: [u8; 2],
    pub dw_get_profile_no: u32,
    pub by_get_profile_count: u8,
    pub by_erase: u8,
    pub reserve2: [u8; 2],
}
pub struct Ljx8ifGetBatchProfileRequest {
    pub by_target_bank: u8,
    pub by_position_mode: u8,
    pub reserve: [u8; 2],
    pub dw_get_batch_no: u32,
    pub dw_get_profile_no: u32,
    pub by_get_profile_count: u8,
    pub by_erase: u8,
    pub reserve2: [u8; 2],
}
pub struct Ljx8ifGetProfileResponse {
    pub dw_current_profile_no: u32,
    pub dw_oldest_profile_no: u32,
    pub dw_get_top_profile_no: u32,
    pub by_get_profile_count: u8,
    pub reserve: [u8; 3],
}
pub struct Ljx8ifGetBatchProfileResponse {
    pub dw_current_batch_no: u32,
    pub dw_current_batch_profile_count: u32,
    pub dw_oldest_batch_no: u32,
    pub dw_oldest_batch_profile_count: u32,
    pub dw_get_batch_no: u32,
    pub dw_get_batch_profile_count: u32,
    pub dw_get_batch_top_profile_no: u32,
    pub by_get_profile_count: u8,
    pub by_current_batch_commited: u8,
    pub reserve: [u8; 2],
}
pub struct Ljx8ifHighSpeedPreStartReq {
    pub by_send_position: u8,
    pub reserve: [u8; 3],
}
pub struct Ljx8ifProfileInfo {
    pub by_profile_count: u8,
    pub reserve1: u8,
    pub by_luminance_output: u8,
    pub reserve2: u8,
    pub w_profile_data_count: u16,
    pub reserve3: [u8; 2],
    pub l_xstart: i32,
    pub l_xpitch: i32,
}
impl LJX8If {
    pub fn ljx8_if_ethernet_open(ip: &str, port: u16) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect((ip, port))?;
        Ok(Self { stream })
    }
    pub fn ljx8_if_communication_close(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
    pub fn ljx8_if_reboot_controller(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x02)?;
        Ok(())
    }
    pub fn ljx8_if_return_to_factory_setting(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x03)?;
        Ok(())
    }
    pub fn ljx8_if_control_laser(&mut self, by_state: u8) -> Result<(), std::io::Error> {
        let senddata = [0x2A, 0x00, 0x00, 0x00, by_state, 0x00, 0x00, 0x00];
        self.my_any_command(&senddata)?;
        Ok(())
    }
    pub fn ljx8_if_get_error(
        &mut self,
        by_received_max: u8,
    ) -> Result<(u8, Vec<u16>), std::io::Error> {
        let receive_buffer = self.send_single_command(0x04)?;
        let number_of_error = receive_buffer[28];
        let copy_num;
        if number_of_error >= 1 {
            if by_received_max > number_of_error {
                copy_num = number_of_error;
            } else {
                copy_num = by_received_max;
            }
            let mut pw_err_code = vec![0u16; copy_num as usize];
            for i in 0..copy_num as usize {
                pw_err_code[i] =
                    u16::from_le_bytes(receive_buffer[32 + i * 2..34 + i * 2].try_into().unwrap());
            }
            return Ok((number_of_error, pw_err_code));
        }
        Ok((number_of_error, vec![]))
    }
    pub fn ljx8_if_clear_error(&mut self, w_err_code: u16) -> Result<(), std::io::Error> {
        let w_err_code_bytes = w_err_code.to_le_bytes();
        let (low_byte, high_byte) = (w_err_code_bytes[0], w_err_code_bytes[1]);
        let senddata = [0x05, 0x00, 0x00, 0x00, low_byte, high_byte, 0x00, 0x00];
        self.my_any_command(&senddata)?;
        Ok(())
    }
    pub fn ljx8_if_trg_error_reset(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x2B)?;
        Ok(())
    }
    pub fn ljx8_if_get_trigger_and_pulse_count(&mut self) -> Result<(u32, i32), std::io::Error> {
        let receive_buffer = self.send_single_command(0x4B)?;

        let trigger_count = u32::from_le_bytes(receive_buffer[28..32].try_into().unwrap());
        let encoder_count = i32::from_le_bytes(receive_buffer[32..36].try_into().unwrap());

        Ok((trigger_count, encoder_count))
    }
    pub fn ljx8_if_get_head_temperature(&mut self) -> Result<(i16, i16, i16), std::io::Error> {
        let receive_buffer = self.send_single_command(0x4C)?;
        let pn_sensor_temperature = i16::from_le_bytes(receive_buffer[28..30].try_into().unwrap());
        let pn_processor_temperature =
            i16::from_le_bytes(receive_buffer[30..32].try_into().unwrap());
        let pn_case_temperature = i16::from_le_bytes(receive_buffer[32..34].try_into().unwrap());
        Ok((
            pn_sensor_temperature,
            pn_processor_temperature,
            pn_case_temperature,
        ))
    }
    pub fn ljx8_if_get_head_model(&mut self) -> Result<String, std::io::Error> {
        let receive_buffer = self.send_single_command(0x06)?;
        let number_of_unit = receive_buffer[28];

        if number_of_unit >= 2 {
            let head_model = std::str::from_utf8(&receive_buffer[96..128])
                .unwrap_or("")
                .trim_end_matches('\0')
                .to_string();
            Ok(head_model)
        } else {
            Ok(String::new())
        }
    }
    pub fn ljx8_if_get_serial_number(&mut self) -> Result<(String, String), std::io::Error> {
        let receive_buffer = self.send_single_command(0x06)?;
        let number_of_unit = receive_buffer[28];
        let mut controller_serial = String::new();
        let mut head_serial = String::new();

        if number_of_unit >= 1 {
            controller_serial = std::str::from_utf8(&receive_buffer[76..92])
                .unwrap()
                .trim_end_matches('\0')
                .to_string();
            if number_of_unit >= 2 {
                head_serial = std::str::from_utf8(&receive_buffer[128..144])
                    .unwrap()
                    .trim_end_matches('\0')
                    .to_string();
            }
        }

        Ok((controller_serial, head_serial))
    }
    pub fn ljx8_if_get_attention_status(&mut self) -> Result<u16, std::io::Error> {
        let receive_buffer = self.send_single_command(0x65)?;

        let attention_status = u16::from_le_bytes(receive_buffer[28..30].try_into().unwrap());

        Ok(attention_status)
    }
    pub fn ljx8_if_trigger(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x21)?;
        Ok(())
    }
    pub fn ljx8_if_start_measure(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x22)?;
        Ok(())
    }
    pub fn ljx8_if_stop_measure(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x23)?;
        Ok(())
    }
    pub fn ljx8_if_clear_memory(&mut self) -> Result<(), std::io::Error> {
        self.send_single_command(0x27)?;
        Ok(())
    }
    //ToDo: LJX8IF_SetSetting, LJX8IF_GetSetting
    pub fn ljx8_if_initialize_setting(
        &mut self,
        by_depth: u8,
        by_target: u8,
    ) -> Result<(), std::io::Error> {
        let senddata = [
            0x3D, 0x00, 0x00, 0x00, by_depth, 0x00, 0x00, 0x00, 0x03, by_target, 0x00, 0x00,
        ];
        self.my_any_command(&senddata)?;
        Ok(())
    }

    pub fn ljx8_if_reflect_setting(&mut self, by_depth: u8) -> Result<u32, std::io::Error> {
        let senddata = [0x33, 0x00, 0x00, 0x00, by_depth, 0x00, 0x00, 0x00];
        let receive_buffer = self.my_any_command(&senddata)?;
        let error = u32::from_le_bytes(receive_buffer[28..32].try_into().unwrap());
        Ok(error)
    }

    pub fn ljx8_if_rewrite_temporary_setting(
        &mut self,
        by_depth: u8,
    ) -> Result<(), std::io::Error> {
        let by_depth = match by_depth {
            1 | 2 => by_depth - 1,
            _ => 0xFF,
        };
        let senddata = [0x35, 0x00, 0x00, 0x00, by_depth, 0x00, 0x00, 0x00];
        self.my_any_command(&senddata)?;
        Ok(())
    }

    pub fn ljx8_if_check_memory_access(&mut self) -> Result<u8, std::io::Error> {
        let receive_buffer = self.send_single_command(0x34)?;
        Ok(receive_buffer[28])
    }

    pub fn ljx8_if_set_xpitch(&mut self, dw_xpitch: u32) -> Result<(), std::io::Error> {
        let xpitch_bytes = dw_xpitch.to_le_bytes();
        let senddata = [
            0x36,
            0x00,
            0x00,
            0x00,
            xpitch_bytes[0],
            xpitch_bytes[1],
            xpitch_bytes[2],
            xpitch_bytes[3],
        ];
        self.my_any_command(&senddata)?;
        Ok(())
    }

    pub fn ljx8_if_get_xpitch(&mut self) -> Result<u32, std::io::Error> {
        let receive_buffer = self.send_single_command(0x37)?;
        Ok(u32::from_le_bytes(
            receive_buffer[28..32].try_into().unwrap(),
        ))
    }

    pub fn ljx8_if_set_timer_count(&mut self, dw_timer_count: u32) -> Result<(), std::io::Error> {
        let timer_count_bytes = dw_timer_count.to_le_bytes();
        let senddata = [
            0x4E,
            0x00,
            0x00,
            0x00,
            timer_count_bytes[0],
            timer_count_bytes[1],
            timer_count_bytes[2],
            timer_count_bytes[3],
        ];
        self.my_any_command(&senddata)?;
        Ok(())
    }

    pub fn ljx8_if_get_timer_count(&mut self) -> Result<u32, std::io::Error> {
        let receive_buffer = self.send_single_command(0x4F)?;
        Ok(u32::from_le_bytes(
            receive_buffer[28..32].try_into().unwrap(),
        ))
    }

    pub fn ljx8_if_change_active_program(
        &mut self,
        by_program_no: u8,
    ) -> Result<(), std::io::Error> {
        let senddata = [0x39, 0x00, 0x00, 0x00, by_program_no, 0x00, 0x00, 0x00];
        self.my_any_command(&senddata)?;
        Ok(())
    }

    pub fn ljx8_if_get_active_program(&mut self) -> Result<u8, std::io::Error> {
        let receive_buffer = self.send_single_command(0x65)?;
        Ok(receive_buffer[24])
    }
    pub fn ljx8_if_get_profile(
        &mut self,
        p_req: Ljx8ifGetProfileRequest,
    ) -> Result<(Ljx8ifGetProfileResponse, Ljx8ifProfileInfo, Vec<u32>), std::io::Error> {
        let aby_dat = p_req.dw_get_profile_no.to_le_bytes();
        let senddata = [
            0x42,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            p_req.by_target_bank,
            p_req.by_position_mode,
            0x00,
            0x00,
            aby_dat[0],
            aby_dat[1],
            aby_dat[2],
            aby_dat[3],
            p_req.by_get_profile_count,
            p_req.by_erase,
            0x00,
            0x00,
        ];
        let receive_buffer = self.my_any_command(&senddata)?;
        let p_rsp = Ljx8ifGetProfileResponse {
            dw_current_profile_no: u32::from_le_bytes(receive_buffer[28..32].try_into().unwrap()),
            dw_oldest_profile_no: u32::from_le_bytes(receive_buffer[32..36].try_into().unwrap()),
            dw_get_top_profile_no: u32::from_le_bytes(receive_buffer[36..40].try_into().unwrap()),
            by_get_profile_count: receive_buffer[40],
            reserve: [0; 3],
        };
        let kind = receive_buffer[48];
        let profile_unit = u16::from_le_bytes(receive_buffer[54..56].try_into().unwrap());
        let p_profile_info = Ljx8ifProfileInfo {
            by_profile_count: get_profile_count(kind),
            reserve1: 0,
            by_luminance_output: match BRIGHTNESS_VALUE & kind > 0 {
                true => 1,
                false => 0,
            },
            reserve2: 0,
            w_profile_data_count: u16::from_le_bytes(receive_buffer[52..54].try_into().unwrap()),
            reserve3: [0; 2],
            l_xstart: i32::from_le_bytes(receive_buffer[56..60].try_into().unwrap()),
            l_xpitch: i32::from_le_bytes(receive_buffer[60..64].try_into().unwrap()),
        };
        let before_conv_data = &receive_buffer[64..];
        println!("before_conv_data length: {:?}", before_conv_data.len());
        let p_dw_profile_data = convert_profile_data(
            p_rsp.by_get_profile_count,
            kind,
            0,
            p_profile_info.w_profile_data_count,
            profile_unit,
            before_conv_data,
        );
        println!("p_dw_profile_data length: {:?}", p_dw_profile_data.len());
        Ok((p_rsp, p_profile_info, p_dw_profile_data))
    }
    pub fn ljx8_if_get_batch_profile(
        &mut self,
        p_req: Ljx8ifGetBatchProfileRequest,
    ) -> Result<(Ljx8ifGetBatchProfileResponse, Ljx8ifProfileInfo, Vec<u32>), std::io::Error> {
        let aby_batch_no = p_req.dw_get_batch_no.to_le_bytes();
        let aby_prof_no = p_req.dw_get_profile_no.to_le_bytes();
        let senddata = [
            0x43,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            p_req.by_target_bank,
            p_req.by_position_mode,
            0x00,
            0x00,
            aby_batch_no[0],
            aby_batch_no[1],
            aby_batch_no[2],
            aby_batch_no[3],
            aby_prof_no[0],
            aby_prof_no[1],
            aby_prof_no[2],
            aby_prof_no[3],
            p_req.by_get_profile_count,
            p_req.by_erase,
            0x00,
            0x00,
        ];
        let receive_buffer = self.my_any_command(&senddata)?;
        println!("receive_buffer.size: {:?}", receive_buffer.len());
        let p_rsp = Ljx8ifGetBatchProfileResponse {
            dw_current_batch_no: u32::from_le_bytes(receive_buffer[28..32].try_into().unwrap()),
            dw_current_batch_profile_count: u32::from_le_bytes(
                receive_buffer[32..36].try_into().unwrap(),
            ) & !MASK_CURRENT_BATCH_COMMITED,
            dw_oldest_batch_no: u32::from_le_bytes(receive_buffer[36..40].try_into().unwrap()),
            dw_oldest_batch_profile_count: u32::from_le_bytes(
                receive_buffer[40..44].try_into().unwrap(),
            ),
            dw_get_batch_no: u32::from_le_bytes(receive_buffer[44..48].try_into().unwrap()),
            dw_get_batch_profile_count: u32::from_le_bytes(
                receive_buffer[48..52].try_into().unwrap(),
            ),
            dw_get_batch_top_profile_no: u32::from_le_bytes(
                receive_buffer[52..56].try_into().unwrap(),
            ),
            by_get_profile_count: receive_buffer[56],
            by_current_batch_commited: match u32::from_le_bytes(
                receive_buffer[32..36].try_into().unwrap(),
            ) & MASK_CURRENT_BATCH_COMMITED
                > 0
            {
                true => 1,
                false => 0,
            },
            reserve: [0; 2],
        };
        let kind = receive_buffer[48 + 16];
        let profile_unit = u16::from_le_bytes(receive_buffer[54 + 16..56 + 16].try_into().unwrap());
        let p_profile_info = Ljx8ifProfileInfo {
            by_profile_count: get_profile_count(kind),
            reserve1: 0,
            by_luminance_output: match BRIGHTNESS_VALUE & kind > 0 {
                true => 1,
                false => 0,
            },
            reserve2: 0,
            w_profile_data_count: u16::from_le_bytes(
                receive_buffer[52 + 16..54 + 16].try_into().unwrap(),
            ),
            reserve3: [0; 2],
            l_xstart: i32::from_le_bytes(receive_buffer[56 + 16..60 + 16].try_into().unwrap()),
            l_xpitch: i32::from_le_bytes(receive_buffer[60 + 16..64 + 16].try_into().unwrap()),
        };
        let before_conv_data = &receive_buffer[64 + 16..];
        let p_dw_profile_data = convert_profile_data(
            p_rsp.by_get_profile_count,
            kind,
            0,
            p_profile_info.w_profile_data_count,
            profile_unit,
            before_conv_data,
        );
        Ok((p_rsp, p_profile_info, p_dw_profile_data))
    }
    fn send_single_command(&mut self, code: u8) -> Result<Vec<u8>, std::io::Error> {
        let command = [code, 0x00, 0x00, 0x00];
        self.my_any_command(&command)
    }
    fn my_any_command(&mut self, command: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut send_buffer = vec![0; 20 * 1024 + 16];
        let data_offset = 16;
        let tcp_length = data_offset + command.len() as u32 - 4;
        let tcp_length_bytes = tcp_length.to_le_bytes();
        send_buffer[..4].copy_from_slice(&tcp_length_bytes);
        send_buffer[4] = 0x03;
        send_buffer[5] = 0x00;
        send_buffer[6] = 0xF0;
        send_buffer[7] = 0x00;
        let length_bytes = (command.len() as u32).to_le_bytes();
        send_buffer[12..16].copy_from_slice(&length_bytes);
        send_buffer[data_offset as usize..data_offset as usize + command.len()]
            .copy_from_slice(command);
        send_buffer.truncate((tcp_length + 4) as usize);
        println!("send_buffer: {:02X?}", send_buffer);
        self.stream.write_all(&send_buffer)?;
        let mut length_buffer = vec![0u8; 4];
        let timeout = Duration::from_secs(5); // 设置5秒超时
        let start_time = Instant::now();

        while self.stream.peek(&mut length_buffer)? < 4 {
            if start_time.elapsed() > timeout {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Timeout"));
            }
            sleep(Duration::from_millis(100));
        }

        let recv_length = u32::from_le_bytes(length_buffer[..].try_into().unwrap());
        let target_length = recv_length + 4;
        let mut receive_buffer = vec![0u8; target_length as usize];
        let _ = self.stream.read_exact(&mut receive_buffer)?;

        //println!("receive_buffer: {:02X?}", receive_buffer);
        println!("target_length: {:?}", target_length);
        Ok(receive_buffer)
    }
}
