import { PossiblePollingRates, useDeviceManager } from "../components/device-manager";
import { NormalButton } from "../components/button";

export const PollingRateSettings = () => {
    const deviceManager = useDeviceManager();

    if (!deviceManager) {
        console.log("DeviceManager not available. Ensure you are using the DeviceManagerProvider.");
        return null;
    }

    const {
        isLoading,
        isInitialized,
        error,
        deviceInformation,
        setPollingRate,
    } = deviceManager;

    if (!deviceInformation || isLoading || !isInitialized) {
        return null;
    }

    if (error.isError) {
        console.error(`Fehler: ${error.message}`);
        return null;
    }

    const {
        pollingRate,
    } = deviceInformation!;

    return (
        <div>
            <div className="flex flex-col gap-3 mt-2">
                {[125, 500, 1000].map(rate => (
                    <NormalButton
                        key={rate}
                        onClick={() => setPollingRate(rate as PossiblePollingRates)}
                        text={`${rate}Hz`}
                        active={pollingRate === rate}
                    />
                ))}
            </div>
        </div>
    )
}