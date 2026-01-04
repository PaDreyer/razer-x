import {Checkbox} from "../components/checkbox";
import {memo, useState} from "react";
import {useDeviceManager} from "../components/device-manager";

export const GeneralSettings = memo(function GeneralSettings(){
    const deviceManager = useDeviceManager();
    const [smartWheelEnabled, _setSmartWheelEnabled] = useState(false);
    const [mouseWheelInverted, _setMouseWheelInverted] = useState(false);

    if (!deviceManager) {
        console.log("DeviceManager not available. Ensure you are using the DeviceManagerProvider.");
        return null;
    }

    const setSmartWheelEnabled = (enabled: boolean) => {
        deviceManager.setSmartWheelEnabled(enabled)
            .then(() => {
                _setSmartWheelEnabled(enabled);
            })
            .catch((error) => {
                console.error(`Fehler beim Aktivieren des Smartwheels: ${error.message}`);
            });
    }

    const setMouseWheelInverted = (inverted: boolean) => {
        deviceManager.setMouseWheelInverted(inverted)
            .then(() => {
                _setMouseWheelInverted(inverted);
            })
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
            { isMacOs &&
                <div>
                    <Checkbox
                        label={"Normales Mausrad"}
                        checked={mouseWheelInverted}
                        onChange={(checked) => setMouseWheelInverted(checked)}
                    />
                </div>
            }
        </div>
    )
});