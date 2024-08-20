use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub struct LJX8IFAsync {
    stream: tokio::net::TcpStream,
}
impl LJX8IFAsync {
    pub async fn ljx8_if_ethernet_open(ip: &str, port: u32) -> Result<Self, std::io::Error> {
        let stream = tokio::net::TcpStream::connect(format!("{}:{}", ip, port)).await?;
        Ok(Self { stream })
    }
    pub async fn ljx8_if_communication_close(&mut self) -> Result<u32, std::io::Error> {
        self.stream.shutdown().await?;
        Ok(0)
    }
    async fn send_single_command(&mut self, code: u8) -> Result<Vec<u8>, std::io::Error> {
        let command = [code, 0x00, 0x00, 0x00];
        self.my_any_command(&command).await
    }
    async fn my_any_command(&mut self, command: &[u8]) -> Result<Vec<u8>, std::io::Error> {
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
        self.stream.write_all(&send_buffer).await?;
        println!("stream.write_all");
        let mut receive_buffer = vec![0u8; 3000 * 1024];
        while self.stream.peek(&mut receive_buffer).await? < 4 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        println!("stream.peek");
        let recv_length = u32::from_le_bytes(receive_buffer[..4].try_into().unwrap());
        let target_length = recv_length + 4;
        receive_buffer.truncate(target_length as usize);
        let _ = self.stream.read_exact(&mut receive_buffer).await?;

        println!("receive_buffer: {:02X?}", receive_buffer);
        println!("target_length: {:?}", target_length);
        Ok(receive_buffer)
    }
    pub async fn ljx8_if_get_head_temperature(
        &mut self,
    ) -> Result<(i16, i16, i16), std::io::Error> {
        let receive_buffer = self.send_single_command(0x4C).await?;
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
    pub async fn ljx8_if_get_serial_number(&mut self) -> Result<(String, String), std::io::Error> {
        let receive_buffer = self.send_single_command(0x06).await?;
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
}
