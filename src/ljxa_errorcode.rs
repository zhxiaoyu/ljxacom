const LJX8IF_RC_OK: u32 = 0x0000;	// Normal termination
const LJX8IF_RC_ERR_OPEN: u32 = 0x1000;	// Failed to open the communication path
const LJX8IF_RC_ERR_NOT_OPEN: u32 = 0x1001;	// The communication path was not established.
const LJX8IF_RC_ERR_SEND: u32 = 0x1002;	// Failed to send the command.
const LJX8IF_RC_ERR_RECEIVE: u32 = 0x1003;	// Failed to receive a response.
const LJX8IF_RC_ERR_TIMEOUT: u32 = 0x1004;	// A timeout occurred while waiting for the response.
const LJX8IF_RC_ERR_NOMEMORY: u32 = 0x1005;	// Failed to allocate memory.
const LJX8IF_RC_ERR_PARAMETER: u32 = 0x1006;	// An invalid parameter was passed.
const LJX8IF_RC_ERR_RECV_FMT: u32 = 0x1007;	// The received response data was invalid

const LJX8IF_RC_ERR_HISPEED_NO_DEVICE: u32 = 0x1009;	// High-speed communication initialization could not be performed.
const LJX8IF_RC_ERR_HISPEED_OPEN_YET: u32 = 0x100A;	// High-speed communication was initialized.
const LJX8IF_RC_ERR_HISPEED_RECV_YET: u32 = 0x100B;	// Error already occurred during high-speed communication (for high-speed communication)
const LJX8IF_RC_ERR_BUFFER_SHORT: u32 = 0x100C;	// The buffer size passed as an argument is insufficient. 