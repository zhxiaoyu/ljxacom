#if defined(_WIN32)
#include "windows/include/LJX8_IF.h"
#include "windows/include/LJX8_ErrorCode.h"
#include "windows/include/LJXA_ACQ.h"
#elif defined(__linux__)
#include "linux/include/LJX8_IF_Linux.h"
#include "linux/include/LJX8_ErrorCode.h"
#include "linux/include/LJXA_ACQ.h"
#endif