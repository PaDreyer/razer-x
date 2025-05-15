#include <CoreFoundation/CoreFoundation.h>
#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>

CFUUIDRef get_usb_device_uuid() {
    //return kIOUSBDeviceUserClientTypeID;
    return kIOUSBInterfaceUserClientTypeID;
}

CFUUIDRef get_plugin_uuid() {
    return kIOCFPlugInInterfaceID;
}