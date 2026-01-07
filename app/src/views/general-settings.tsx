import { Checkbox } from "../components/checkbox";
import { memo } from "react";
import { useDeviceManager } from "../components/device-manager";

export const GeneralSettings = memo(function GeneralSettings() {
    const deviceManager = useDeviceManager();
    const smartWheelEnabled = deviceManager.deviceInformation?.smartWheelEnabled ?? false;
    const mouseWheelInverted = deviceManager.deviceInformation?.mouseWheelInverted ?? false;

    const setSmartWheelEnabled = (enabled: boolean) => {
        deviceManager.setSmartWheelEnabled(enabled)
            .catch((error) => {
                console.error(`Fehler beim Aktivieren des Smartwheels: ${error.message}`);
            });
    }

    const setMouseWheelInverted = (inverted: boolean) => {
        deviceManager.setMouseWheelInverted(inverted)
            .catch((error) => {
                console.error(`Fehler beim Invertieren des Mausrads: ${error.message}`);
            });
    }

    const isMacOs = deviceManager.deviceInformation?.targetOs === "macos"

    return (
        <div className="flex-col items-center">
            <div>
                <Checkbox
                    label={"Smartwheel aktivieren"}
                    checked={smartWheelEnabled}
                    onChange={(checked) => setSmartWheelEnabled(checked)}
                />
            </div>
            {isMacOs &&
                <div>
                    <Checkbox
                        label={"Natürliches Scrollen"}
                        checked={mouseWheelInverted}
                        onChange={(checked) => setMouseWheelInverted(checked)}
                    />
                </div>
            }
        </div>
    )
});