#ifndef USB_DRIVER_MACOS_H
#define USB_DRIVER_MACOS_H

#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>
#include <IOKit/IOKitLib.h>
//#include <CoreFoundation/CoreFoundation.h>

CFUUIDRef get_usb_device_uuid(void);
CFUUIDRef get_plugin_uuid(void);

int macos_usb_send_control(IOUSBDeviceInterface **device,
                           uint8_t request_type,
                           uint8_t request,
                           uint16_t value,
                           uint16_t index,
                           void *data,
                           uint16_t length);

#endif