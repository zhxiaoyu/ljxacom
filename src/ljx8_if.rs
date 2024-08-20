use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Instant, Duration};
pub struct LJX8If {
    stream: TcpStream,
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

        println!("receive_buffer: {:02X?}", receive_buffer);
        println!("target_length: {:?}", target_length);
        Ok(receive_buffer)
    }
}