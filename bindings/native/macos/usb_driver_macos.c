#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>

CFUUIDRef get_usb_device_uuid() {
    return kIOUSBDeviceUserClientTypeID;
}

CFUUIDRef get_plugin_uuid() {
    return kIOCFPlugInInterfaceID;
}

CFUUIDRef get_usb_device_interface_uuid() {
    return kIOUSBDeviceInterfaceID;
}