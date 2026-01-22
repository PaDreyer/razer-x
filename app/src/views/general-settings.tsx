import { memo } from "react";
import { PopoverColorPicker } from "../components/popover-colorpicker";
import { RGBColor, useDeviceManager } from "../components/device-manager";

export const GeneralSettings = memo(function GeneralSettings() {
    const deviceManager = useDeviceManager();

    if (!deviceManager?.deviceInformation) return null;

    return (
        <div className="flex flex-col">
            <div className="flex flex-col items-start">
                <PopoverColorPicker
                    color={deviceManager.deviceInformation.backlightColor}
                    onChange={(color) => deviceManager.setBacklightColor(color as RGBColor)}
                    presetColors={[
                        { r: 255, g: 255, b: 255 },
                        { r: 0, g: 255, b: 22 },
                        { r: 0, g: 15, b: 255 },
                        { r: 255, g: 0, b: 0 },
                    ]}
                />
            </div>
        </div>
    )
});