# Razer-X 🖱️

A cross-platform application to control Razer Basilisk V3 Pro mouse on macOS and Linux - without requiring Razer Synapse!

## ✨ Features

### Device Control
- ✅ **DPI Settings** - Configure DPI stages and active DPI
- ✅ **Polling Rate** - Adjust polling rate (125Hz, 500Hz, 1000Hz)
- ✅ **RGB Lighting** - Control backlight color and brightness
- ✅ **Battery Status** - Monitor battery level
- ✅ **Scroll Direction** - Automatic trackpad scroll direction management

### System Integration
- ✅ **Tray Icon** - System tray integration with context menu
- ✅ **Auto-Apply Settings** - Automatically applies settings when mouse is detected
- ✅ **Hotplug Detection** - Detects when mouse/dongle is connected or disconnected
- ✅ **Wireless Mouse Power Detection** - Detects when wireless mouse powers on/off (even when dongle stays plugged in)

### Platform Support
- ✅ **macOS** - Full support with native IOKit integration (macOS 10.13+)
- 🚧 **Linux** - In progress

## 🔥 Wireless Mouse Power State Detection

One of the standout features is **intelligent wireless mouse detection**:

- **Problem**: Traditional USB hotplug detection doesn't work for wireless mice because the USB receiver stays plugged in
- **Solution**: Polls the device every 2 seconds and analyzes the Razer protocol status byte
- **Result**: Automatically applies your settings when you turn the mouse on, and reverts trackpad settings when you turn it off

**How it works:**
1. Queries firmware version from the device
2. Checks the status byte in the response:
   - `0x02` = Mouse is ON ✅
   - `0x04` = Mouse is OFF/Timeout ❌
3. Triggers callbacks only on state changes
4. Also handles physical USB dongle unplug/plug events

This means you get seamless integration - your mouse settings are always applied when you need them!

## 📋 System Requirements

### macOS
- **macOS 10.13 (High Sierra) or later**
- IOKit framework (included in macOS)
- USB access permissions (automatically requested)

### Linux (In Progress)
- libusb 1.0+
- udev rules for device access

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70+)
- **Node.js** & **Yarn** (for Tauri UI)
- **Git**

### Installation

```bash
# Clone the repository
git clone https://github.com/PaDreyer/razer-x.git
cd razer-x

# Build the project
cargo build --release

# Run the application
cd app
yarn install
yarn tauri dev
```

### Building for Production

```bash
cd app
yarn tauri build
```

The built application will be compatible with **macOS 10.13 (High Sierra) and later**.

## 📁 Project Structure

```
razer-x/
├── app/              # Tauri application (UI + main process)
├── driver/           # USB driver abstraction layer
├── razer/            # Razer protocol implementation
├── bindings/         # Native bindings (IOKit, libusb, etc.)
├── gui/              # GUI components
└── service-installer/# Service installation utilities
```

## 🛠️ Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Debug Logging

Enable detailed logging:

```bash
RUST_LOG=debug yarn tauri dev
```

## 🎯 Supported Devices

Currently supported:
- **Razer Basilisk V3 Pro** (Wired & Wireless)

Planned support:
- More Razer mice (contributions welcome!)

## 🔧 Configuration

The application automatically:
- Detects your Razer mouse
- Applies default settings (3200 DPI, 1000Hz polling, white RGB)
- Manages trackpad scroll direction based on mouse state

Settings are applied when:
- Mouse powers on
- USB dongle is plugged in
- Application starts with mouse already connected

## 🐛 Known Issues

- Linux support is work in progress
- Only Basilisk V3 Pro is currently supported
- Windows support is not planned at this time

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Add support for more devices** - Implement protocol for other Razer mice
2. **Linux support** - Complete Linux implementation
3. **UI improvements** - Enhance the user interface
4. **Bug fixes** - Report and fix bugs
5. **Documentation** - Improve docs and add examples

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 Technical Details

### Razer Protocol

The application implements the Razer USB HID protocol:
- Uses USB control transfers for device communication
- Implements firmware version, battery, DPI, polling rate, and RGB commands
- Status byte analysis for wireless mouse state detection

### Platform-Specific Implementation

**macOS:**
- Uses IOKit for USB device access
- IOHIDDevice notifications for hotplug events
- Polling-based wireless mouse power state detection
- Compatible with macOS 10.13 (High Sierra) and later

**Linux (In Progress):**
- libusb for USB device access
- libusb hotplug API for device events

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [OpenRazer](https://github.com/openrazer/openrazer) - For protocol documentation and inspiration
- [Tauri](https://tauri.app/) - For the amazing desktop app framework
- Razer community - For reverse engineering efforts

## 📧 Contact

- **Issues**: [GitHub Issues](https://github.com/PaDreyer/razer-x/issues)
- **Discussions**: [GitHub Discussions](https://github.com/PaDreyer/razer-x/discussions)

---

**Note**: This is an unofficial project and is not affiliated with Razer Inc.
