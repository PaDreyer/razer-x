
#ifndef USB_DRIVER_LINUX_H
#define USB_DRIVER_LINUX_H

#include <libusb-1.0/libusb.h>

int linux_usb_send_control(struct libusb_device_handle *handle,
                           uint8_t request_type,
                           uint8_t request,
                           uint16_t value,
                           uint16_t index,
                           unsigned char *data,
                           uint16_t length);

#endif
