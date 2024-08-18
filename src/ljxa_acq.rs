struct LJXA_ACQ_SETPARAM{
	y_linenum: i32,
	y_pitch_um: f32,
	timeout_ms: i32,
	use_external_batchStart: i32,
}

struct LJXA_ACQ_GETPARAM{
	luminance_enabled: i32,
	x_pointnum: i32,
	y_linenum_acquired: i32,
	x_pitch_um: f32,
	y_pitch_um: f32,
	z_pitch_um: f32,
}

const MAX_LJXA_DEVICENUM: i32 = 6;
const MAX_LJXA_XDATANUM: i32 = 3200;
const BUFFER_FULL_COUNT: u32 = 30000;

fn LJXA_ACQ_OpenDevice(lDeviceId: i32, EthernetConfig: LJX8IF_ETHERNET_CONFIG, HighSpeedPortNo: i32) {
    let errCode = LJX8IF_EthernetOpen(lDeviceId, EthernetConfig);

    _ethernetConfig[lDeviceId] = EthernetConfig;
    _highSpeedPortNo[lDeviceId] = HighSpeedPortNo;
    println!("[@(LJXA_ACQ_OpenDevice) Open device](0x%x)\n", errCode);

    return errCode;
}

fn LJXA_ACQ_CloseDevice(lDeviceId: i32) {
    LJX8IF_FinalizeHighSpeedDataCommunication(lDeviceId);
    LJX8IF_CommunicationClose(lDeviceId);
    println!("[@(LJXA_ACQ_CloseDevice) Close device]\n");
}
