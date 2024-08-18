pub const MAX_LJXA_DEVICENUM: u32 = 6;
pub const MAX_LJXA_XDATANUM: u32 = 3200;
pub const MAX_LJXA_YDATANUM: u32 = 42000;

/// 设置值存储级别指定
#[repr(u8)]
pub enum LJX8IF_SETTING_DEPTH {
    WRITE = 0x00,   // 写入设置区域
    RUNNING = 0x01, // 运行设置区域
    SAVE = 0x02,    // 保存区域
}

/// 初始化目标设置项目指定
#[repr(u8)]
pub enum LJX8IF_INIT_SETTING_TARGET {
    PRG0 = 0x00,  // 程序 0
    PRG1 = 0x01,  // 程序 1
    PRG2 = 0x02,  // 程序 2
    PRG3 = 0x03,  // 程序 3
    PRG4 = 0x04,  // 程序 4
    PRG5 = 0x05,  // 程序 5
    PRG6 = 0x06,  // 程序 6
    PRG7 = 0x07,  // 程序 7
    PRG8 = 0x08,  // 程序 8
    PRG9 = 0x09,  // 程序 9
    PRG10 = 0x0A, // 程序 10
    PRG11 = 0x0B, // 程序 11
    PRG12 = 0x0C, // 程序 12
    PRG13 = 0x0D, // 程序 13
    PRG14 = 0x0E, // 程序 14
    PRG15 = 0x0F, // 程序 15
}

/// 获取配置文件目标缓冲区指定
#[repr(u8)]
pub enum LJX8IF_PROFILE_BANK {
    ACTIVE = 0x00,   // 活动表面
    INACTIVE = 0x01, // 非活动表面
}

/// 获取配置文件位置规范方法指定（批量测量：关闭）
#[repr(u8)]
pub enum LJX8IF_PROFILE_POSITION {
    CURRENT = 0x00, // 从当前
    OLDEST = 0x01,  // 从最旧
    SPEC = 0x02,    // 指定位置
}

/// 获取配置文件批次数据位置规范方法指定（批量测量：开启）
#[repr(u8)]
pub enum LJX8IF_BATCH_POSITION {
    CURRENT = 0x00,      // 从当前
    SPEC = 0x02,         // 指定位置
    COMMITED = 0x03,     // 从批次提交后的当前
    CURRENT_ONLY = 0x04, // 仅当前
}

/// 版本信息结构
#[repr(C)]
pub struct LJX8IF_VERSION_INFO {
    pub n_major_number: i32,    // 主版本号
    pub n_minor_number: i32,    // 次版本号
    pub n_revision_number: i32, // 修订号
    pub n_build_number: i32,    // 构建号
}

/// 以太网设置结构
#[repr(C)]
pub struct LJX8IF_ETHERNET_CONFIG {
    pub aby_ip_address: [u8; 4], // 要连接的控制器的IP地址
    pub w_port_no: u16,          // 要连接的控制器的端口号
    pub reserve: [u8; 2],        // 保留
}

/// 设置项目指定结构
#[repr(C)]
pub struct LJX8IF_TARGET_SETTING {
    pub by_type: u8,     // 设置类型
    pub by_category: u8, // 类别
    pub by_item: u8,     // 设置项目
    pub reserve: u8,     // 保留
    pub by_target1: u8,  // 设置目标 1
    pub by_target2: u8,  // 设置目标 2
    pub by_target3: u8,  // 设置目标 3
    pub by_target4: u8,  // 设置目标 4
}

/// 配置文件信息
#[repr(C)]
pub struct LJX8IF_PROFILE_INFO {
    pub by_profile_count: u8,    // 固定为1（保留供将来使用）
    pub reserve1: u8,            // 保留
    pub by_luminance_output: u8, // 亮度输出是否开启
    pub reserve2: u8,            // 保留
    pub w_profile_data_count: u16, // 配置文件数据计数
    pub reserve3: [u8; 2],       // 保留
    pub l_x_start: i32,          // 第一点X坐标
    pub l_x_pitch: i32,          // 配置文件数据X方向间隔
}

/// 配置文件头信息结构
#[repr(C)]
pub struct LJX8IF_PROFILE_HEADER {
    pub reserve: u32,         // 保留
    pub dw_trigger_count: u32, // 触发发生时的触发计数
    pub l_encoder_count: i32, // 触发发生时的编码器计数
    pub reserve2: [u32; 3],   // 保留
}

/// 配置文件尾信息结构
#[repr(C)]
pub struct LJX8IF_PROFILE_FOOTER {
    pub reserve: u32, // 保留
}

/// 获取配置文件请求结构（批量测量：关闭）
#[repr(C)]
pub struct LJX8IF_GET_PROFILE_REQUEST {
    pub by_target_bank: u8,    // 要读取的目标表面
    pub by_position_mode: u8,  // 获取配置文件位置规范方法
    pub reserve: [u8; 2],      // 保留
    pub dw_get_profile_no: u32, // 要获取的配置文件的配置文件号
    pub by_get_profile_count: u8, // 要读取的配置文件数量
    pub by_erase: u8,          // 指定是否擦除已读取的配置文件数据和比其更旧的配置文件数据
    pub reserve2: [u8; 2],     // 保留
}

/// 获取配置文件请求结构（批量测量：开启）
#[repr(C)]
pub struct LJX8IF_GET_BATCH_PROFILE_REQUEST {
    pub by_target_bank: u8,    // 要读取的目标表面
    pub by_position_mode: u8,  // 获取配置文件位置规范方法
    pub reserve: [u8; 2],      // 保留
    pub dw_get_batch_no: u32,  // 要获取的配置文件的批次号
    pub dw_get_profile_no: u32, // 在指定批次号中开始获取配置文件的配置文件号
    pub by_get_profile_count: u8, // 要读取的配置文件数量
    pub by_erase: u8,          // 指定是否擦除已读取的配置文件数据和比其更旧的配置文件数据
    pub reserve2: [u8; 2],     // 保留
}

/// 获取配置文件响应结构（批量测量：关闭）
#[repr(C)]
pub struct LJX8IF_GET_PROFILE_RESPONSE {
    pub dw_current_profile_no: u32, // 当前时间点的配置文件号
    pub dw_oldest_profile_no: u32,  // 控制器持有的最旧配置文件的配置文件号
    pub dw_get_top_profile_no: u32, // 这次读取的最旧配置文件的配置文件号
    pub by_get_profile_count: u8,   // 这次读取的配置文件数量
    pub reserve: [u8; 3],           // 保留
}

/// 获取配置文件响应结构（批量测量：开启）
#[repr(C)]
pub struct LJX8IF_GET_BATCH_PROFILE_RESPONSE {
    pub dw_current_batch_no: u32,           // 当前时间点的批次号
    pub dw_current_batch_profile_count: u32, // 最新批次中的配置文件数量
    pub dw_oldest_batch_no: u32,            // 控制器持有的最旧批次的批次号
    pub dw_oldest_batch_profile_count: u32,  // 控制器持有的最旧批次中的配置文件数量
    pub dw_get_batch_no: u32,               // 这次读取的批次号
    pub dw_get_batch_profile_count: u32,     // 这次读取的批次中的配置文件数量
    pub dw_get_batch_top_profile_no: u32,    // 这次读取的批次中最旧的配置文件号
    pub by_get_profile_count: u8,           // 这次读取的配置文件数量
    pub by_current_batch_commited: u8,      // 最新批次号的批量测量已完成
    pub reserve: [u8; 2],                   // 保留
}

/// 高速通信准备开始请求结构
#[repr(C)]
pub struct LJX8IF_HIGH_SPEED_PRE_START_REQ {
    pub by_send_position: u8, // 发送开始位置
    pub reserve: [u8; 3],     // 保留
}