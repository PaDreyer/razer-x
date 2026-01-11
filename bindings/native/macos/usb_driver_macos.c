#include <IOKit/IOCFPlugIn.h>
#include <IOKit/usb/IOUSBLib.h>

typedef int CGSConnection;
extern CGSConnection _CGSDefaultConnection(void);
extern void CGSSetSwipeScrollDirection(const CGSConnection cid, bool dir);

void set_swipe_scroll_direction(bool direction) {
    CGSConnection cid = _CGSDefaultConnection();
    CGSSetSwipeScrollDirection(cid, direction);
}

CFUUIDRef get_usb_device_uuid() {
    return kIOUSBDeviceUserClientTypeID;
}

CFUUIDRef get_plugin_uuid() {
    return kIOCFPlugInInterfaceID;
}

CFUUIDRef get_usb_device_interface_uuid() {
    return kIOUSBDeviceInterfaceID;
}