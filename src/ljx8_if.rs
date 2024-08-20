include!("ljxa_errorcode.rs");

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::mem;
use std::net::{SocketAddr, TcpStream};
use std::sync::RwLock;
use std::time::Duration;

pub const MAX_LJXA_DEVICE_NUM: usize = 6;
pub const MAX_LJXA_X_DATA_NUM: usize = 3200;
pub const MAX_LJXA_Y_DATA_NUM: usize = 42000;

const LJXA_CONNECT_TIMEOUT_SEC: u64 = 3;
const WRITE_DATA_SIZE: usize = 20 * 1024;
const SEND_BUF_SIZE: usize = WRITE_DATA_SIZE + 16;
const RECEIVE_BUF_SIZE: usize = 3000 * 1024;
const FAST_RECEIVE_BUF_SIZE: usize = 64 * 1024;
const PARITY_ERROR: u32 = 0x40000000;

const RES_ADRESS_OFFSET_ERROR: usize = 17;
const PREFIX_ERROR_CONTROLLER: u32 = 0x8000;

/// 设置值存储级别指定
pub enum Ljx8ifSettingDepth {
    Write = 0x00,   // 写入设置区域
    Running = 0x01, // 运行设置区域
    Save = 0x02,    // 保存区域
}

/// 初始化目标设置项目指定
pub enum Ljx8ifInitSettingTarget {
    Prg0 = 0x00,  // 程序 0
    Prg1 = 0x01,  // 程序 1
    Prg2 = 0x02,  // 程序 2
    Prg3 = 0x03,  // 程序 3
    Prg4 = 0x04,  // 程序 4
    Prg5 = 0x05,  // 程序 5
    Prg6 = 0x06,  // 程序 6
    Prg7 = 0x07,  // 程序 7
    Prg8 = 0x08,  // 程序 8
    Prg9 = 0x09,  // 程序 9
    Prg10 = 0x0A, // 程序 10
    Prg11 = 0x0B, // 程序 11
    Prg12 = 0x0C, // 程序 12
    Prg13 = 0x0D, // 程序 13
    Prg14 = 0x0E, // 程序 14
    Prg15 = 0x0F, // 程序 15
}

/// 获取配置文件目标缓冲区指定
pub enum Ljx8ifProfileBank {
    Active = 0x00,   // 活动表面
    Inactive = 0x01, // 非活动表面
}

/// 获取配置文件位置规范方法指定（批量测量：关闭）
pub enum Ljx8ifProfilePosition {
    Current = 0x00, // 从当前
    Oldest = 0x01,  // 从最旧
    Spec = 0x02,    // 指定位置
}

/// 获取配置文件批次数据位置规范方法指定（批量测量：开启）
pub enum Ljx8ifBatchPosition {
    Current = 0x00,     // 从当前
    Spec = 0x02,        // 指定位置
    Commited = 0x03,    // 从批次提交后的当前
    CurrentOnly = 0x04, // 仅当前
}

/// 版本信息结构
pub struct Ljx8ifVersionInfo {
    pub major_number: i32,    // 主版本号
    pub minor_number: i32,    // 次版本号
    pub revision_number: i32, // 修订号
    pub build_number: i32,    // 构建号
}

/// 以太网设置结构
pub struct Ljx8ifEthernetConfig {
    pub ip_address: [u8; 4], // 要连接的控制器的IP地址
    pub port_no: u16,        // 要连接的控制器的端口号
    pub reserve: [u8; 2],    // 保留
}

/// 设置项目指定结构
pub struct Ljx8ifTargetSetting {
    pub type_: u8,    // 设置类型
    pub category: u8, // 类别
    pub item: u8,     // 设置项目
    pub reserve: u8,  // 保留
    pub target1: u8,  // 设置目标 1
    pub target2: u8,  // 设置目标 2
    pub target3: u8,  // 设置目标 3
    pub target4: u8,  // 设置目标 4
}

/// 配置文件信息
pub struct Ljx8ifProfileInfo {
    pub profile_count: u8,       // 固定为1（保留供将来使用）
    pub reserve1: u8,            // 保留
    pub luminance_output: u8,    // 亮度输出是否开启
    pub reserve2: u8,            // 保留
    pub profile_data_count: u16, // 配置文件数据计数
    pub reserve3: [u8; 2],       // 保留
    pub x_start: i32,            // 第一点X坐标
    pub x_pitch: i32,            // 配置文件数据X方向间隔
}

/// 配置文件头信息结构
pub struct Ljx8ifProfileHeader {
    pub reserve: u32,       // 保留
    pub trigger_count: u32, // 触发发生时的触发计数
    pub encoder_count: i32, // 触发发生时的编码器计数
    pub reserve2: [u32; 3], // 保留
}

/// 配置文件尾信息结构
pub struct Ljx8ifProfileFooter {
    pub reserve: u32, // 保留
}

/// 获取配置文件请求结构（批量测量：关闭）
pub struct Ljx8ifGetProfileRequest {
    pub target_bank: u8,       // 要读取的目标表面
    pub position_mode: u8,     // 获取配置文件位置规范方法
    pub reserve: [u8; 2],      // 保留
    pub get_profile_no: u32,   // 要获取的配置文件的配置文件号
    pub get_profile_count: u8, // 要读取的配置文件数量
    pub erase: u8,             // 指定是否擦除已读取的配置文件数据和比其更旧的配置文件数据
    pub reserve2: [u8; 2],     // 保留
}

/// 获取配置文件请求结构（批量测量：开启）
pub struct Ljx8ifGetBatchProfileRequest {
    pub target_bank: u8,       // 要读取的目标表面
    pub position_mode: u8,     // 获取配置文件位置规范方法
    pub reserve: [u8; 2],      // 保留
    pub get_batch_no: u32,     // 要获取的配置文件的批次号
    pub get_profile_no: u32,   // 在指定批次号中开始获取配置文件的配置文件号
    pub get_profile_count: u8, // 要读取的配置文件数量
    pub erase: u8,             // 指定是否擦除已读取的配置文件数据和比其更旧的配置文件数据
    pub reserve2: [u8; 2],     // 保留
}

/// 获取配置文件响应结构（批量测量：关闭）
pub struct Ljx8ifGetProfileResponse {
    pub current_profile_no: u32, // 当前时间点的配置文件号
    pub oldest_profile_no: u32,  // 控制器持有的最旧配置文件的配置文件号
    pub get_top_profile_no: u32, // 这次读取的最旧配置文件的配置文件号
    pub get_profile_count: u8,   // 这次读取的配置文件数量
    pub reserve: [u8; 3],        // 保留
}

/// 获取配置文件响应结构（批量测量：开启）
pub struct Ljx8ifGetBatchProfileResponse {
    pub current_batch_no: u32,            // 当前时间点的批次号
    pub current_batch_profile_count: u32, // 最新批次中的配置文件数量
    pub oldest_batch_no: u32,             // 控制器持有的最旧批次的批次号
    pub oldest_batch_profile_count: u32,  // 控制器持有的最旧批次中的配置文件数量
    pub get_batch_no: u32,                // 这次读取的批次号
    pub get_batch_profile_count: u32,     // 这次读取的批次中的配置文件数量
    pub get_batch_top_profile_no: u32,    // 这次读取的批次中最旧的配置文件号
    pub get_profile_count: u8,            // 这次读取的配置文件数量
    pub current_batch_commited: u8,       // 最新批次号的批量测量已完成
    pub reserve: [u8; 2],                 // 保留
}

/// 高速通信准备开始请求结构
pub struct Ljx8ifHighSpeedPreStartReq {
    pub send_position: u8, // 发送开始位置
    pub reserve: [u8; 3],  // 保留
}

lazy_static! {
    static ref DEVICE_STREAMS: RwLock<HashMap<i32, TcpStream>> = RwLock::new(HashMap::new());
}
pub fn ljx8_if_initialize() -> u32 {
    LJX8IF_RC_OK
}
pub fn ljx8_if_finalize() -> u32 {
    LJX8IF_RC_OK
}
pub fn ljx8_if_get_version() -> Ljx8ifVersionInfo {
    Ljx8ifVersionInfo {
        major_number: 1,
        minor_number: 0,
        revision_number: 0,
        build_number: 0,
    }
}
pub fn ljx8_if_ethernet_open(l_device_id: i32, p_ethernet_config: &Ljx8ifEthernetConfig) -> u32 {
    // 检查输入参数
    if p_ethernet_config.ip_address.len() != 4 {
        return LJX8IF_RC_ERR_PARAMETER;
    }

    // 构造IP地址字符串
    let ip = format!(
        "{}.{}.{}.{}",
        p_ethernet_config.ip_address[0],
        p_ethernet_config.ip_address[1],
        p_ethernet_config.ip_address[2],
        p_ethernet_config.ip_address[3]
    );

    // 构造SocketAddr
    let addr = SocketAddr::new(ip.parse().unwrap(), p_ethernet_config.port_no);

    // 尝试连接，设置超时
    match TcpStream::connect_timeout(&addr, Duration::from_secs(LJXA_CONNECT_TIMEOUT_SEC)) {
        Ok(stream) => {
            // 将stream存储在一个全局的HashMap中，以l_device_id为键
            DEVICE_STREAMS.write().unwrap().insert(l_device_id, stream);
            LJX8IF_RC_OK
        }
        Err(e) => match e.kind() {
            ErrorKind::TimedOut => LJX8IF_RC_ERR_OPEN,
            _ => LJX8IF_RC_ERR_OPEN,
        },
    }
}
pub fn ljx8_if_communication_close(l_device_id: i32) -> u32 {
    let mut streams = DEVICE_STREAMS.write().unwrap();
    if let Some(stream) = streams.remove(&l_device_id) {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
    LJX8IF_RC_OK
}

fn my_any_command(l_device_id: i32, send_data: &[u8], data_length: i32) -> u32 {
    let data_offset = 16;
    let mut send_buffer = [0; SEND_BUF_SIZE];

    let buf_slice = send_buffer.as_mut_slice();
    buf_slice[4] = 0x03;
    buf_slice[5] = 0x00;
    buf_slice[6] = 0xF0;
    buf_slice[7] = 0x00;

    let length_bytes = (data_length as u32).to_le_bytes();
    buf_slice[12..16].copy_from_slice(&length_bytes);

    buf_slice[data_offset..data_offset + data_length as usize].copy_from_slice(send_data);
    let tcp_length = data_offset + data_length as usize - 4;
    LittleEndian::write_u32(&mut send_buffer[0..4], tcp_length as u32);

    let streams = DEVICE_STREAMS.read().unwrap();
    if let Some(mut stream) = streams.get(&l_device_id).and_then(|s| s.try_clone().ok()) {
        match stream.write_all(&send_buffer[..tcp_length + 4]) {
            Ok(_) => {
                // 发送成功后，尝试查看接收的数据
                let mut peek_buffer = [0u8; RECEIVE_BUF_SIZE];
                let mut n = 0;
                while n < 4 {
                    match stream.peek(&mut peek_buffer[..]) {
                        Ok(m) => n = m,
                        Err(_) => return LJX8IF_RC_ERR_RECEIVE,
                    }
                }
                let mut receive_buffer = vec![0u8; RECEIVE_BUF_SIZE];
                // 读取前4个字节以获取总长度
                let _ = stream.read_exact(&mut receive_buffer[..4]);
                let recv_length =
                    (&receive_buffer[..4]).read_u32::<LittleEndian>().unwrap() as usize;
                let target_length = recv_length + 4;
                // 确保buffer大小足够
                if receive_buffer.len() < target_length {
                    receive_buffer.resize(target_length, 0);
                }
                let mut received_bytes = 4;
                while received_bytes < target_length {
                    let try_bytes = std::cmp::min(target_length - received_bytes, 65535);

                    // 使用peek来检查可用数据
                    let mut n = 0;
                    while n < try_bytes {
                        n = stream
                            .peek(&mut receive_buffer[received_bytes..received_bytes + try_bytes])
                            .unwrap();
                    }

                    // 实际读取数据
                    stream
                        .read_exact(&mut receive_buffer[received_bytes..received_bytes + try_bytes])
                        .unwrap();
                    received_bytes += try_bytes;
                }
                receive_buffer.truncate(target_length);
                let err_code = receive_buffer[RES_ADRESS_OFFSET_ERROR];

                if err_code != 0 {
                    return PREFIX_ERROR_CONTROLLER + err_code as u32;
                }
                LJX8IF_RC_OK
            }
            Err(_) => LJX8IF_RC_ERR_SEND,
        }
    } else {
        LJX8IF_RC_ERR_NOT_OPEN
    }
}
