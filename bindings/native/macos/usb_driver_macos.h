#ifndef USB_DRIVER_MACOS_H
#define USB_DRIVER_MACOS_H

#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>
#include <IOKit/IOKitLib.h>

// Function declarations for USB driver operations on macOS
CFUUIDRef get_usb_device_uuid(void);
CFUUIDRef get_plugin_uuid(void);
CFUUIDRef get_usb_device_interface_uuid(void);

void set_swipe_scroll_direction(bool direction);

#endif