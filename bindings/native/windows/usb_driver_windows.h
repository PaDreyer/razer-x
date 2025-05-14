
#ifndef USB_DRIVER_WINDOWS_H
#define USB_DRIVER_WINDOWS_H

#include <windows.h>
#include <winusb.h>

int windows_usb_send_control(WINUSB_INTERFACE_HANDLE handle,
                             UCHAR request_type,
                             UCHAR request,
                             USHORT value,
                             USHORT index,
                             PUCHAR buffer,
                             USHORT length);

#endif
