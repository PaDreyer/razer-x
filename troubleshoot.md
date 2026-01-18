# Troubleshooting

## Linux

### USB Permissions

#### udev
Permission Fix: Since the error is LIBUSB_ERROR_ACCESS (error -3), the user needs to add a udev rule.
Create a file 
/etc/udev/rules.d/99-razer.rules
 with the following content:
 
# Razer Basilisk V3 Pro
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1532", ATTRS{idProduct}=="00ab", MODE="0666"

Then run:
sudo udevadm control --reload-rules && sudo udevadm trigger
Re-plug the device and try again.