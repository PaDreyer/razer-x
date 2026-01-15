import { createFileRoute } from '@tanstack/react-router'
import toast from "react-hot-toast";
import { useDeviceManager } from "../components/device-manager";
import { DpiSettings } from "../views/dpi-settings.tsx";
import { Panel } from "../components/panel";
import { BatteryStatus } from "../components/battery-status";
import { RgbSettings } from "../views/rgb-settings.tsx";
import { GeneralSettings } from "../views/general-settings.tsx";
import { PollingRateSettings } from "../views/polling-rate-settings.tsx";

export const Route = createFileRoute('/')({
    component: Index,
})

function Index() {
    const deviceManager = useDeviceManager();

    if (!deviceManager) {
        console.log("DeviceManager not available. Ensure you are using the DeviceManagerProvider.");
        return null;
    }

    if (!deviceManager.deviceInformation) {
        return null;
    }

    const { batteryLevel, isCharging } = deviceManager.deviceInformation!;

    if (deviceManager.error.isError) {
        toast(`Fehler: ${deviceManager.error.message}`)
    }

    const shouldShow = deviceManager.isInitialized && !deviceManager.error.isError;


    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8">
            <div className="max-w-4xl mx-auto relative">

                <div className="flex flex-wrap-reverse flex-row justify-between">
                    <h2 className="flex-grow text-4xl font-semibold mt-4 mb-8" >Razer Basilisk V3 Pro</h2>
                    {shouldShow &&
                        <div className="flex flex-grow gap-4 items-start justify-end">
                            <BatteryStatus batteryLevel={batteryLevel} isCharging={isCharging} />
                        </div>
                    }
                </div>

                {shouldShow &&
                    <>
                        <div className="grid grid-cols-6 gap-8 max-w-screen">
                            <Panel className="col-span-6 md:col-span-3 sm:col-span-6 row-span-2">
                                <Panel.Header>
                                    RGB Einstellungen
                                </Panel.Header>
                                <Panel.Body>
                                    <RgbSettings />
                                </Panel.Body>
                            </Panel>
                            <Panel className="col-span-6 sm:col-span-6 md:col-span-3 md:col-start-4 row-span-1">
                                <Panel.Header>
                                    Polling Rate Einstellungen
                                </Panel.Header>
                                <Panel.Body>
                                    <PollingRateSettings />
                                </Panel.Body>
                            </Panel>
                            <Panel className="col-span-6 sm:col-span-6 md:col-span-3 md:col-start-4 row-span-1">
                                <Panel.Header>
                                    Allgemeine Einstellungen
                                </Panel.Header>
                                <Panel.Body>
                                    <GeneralSettings />
                                </Panel.Body>
                            </Panel>
                            <Panel className="col-span-6 row-span-2 bg-gray-800 rounded-lg p-6">
                                <Panel.Header>
                                    DPI Einstellungen
                                </Panel.Header>
                                <Panel.Body>
                                    <DpiSettings />
                                </Panel.Body>
                            </Panel>
                        </div>
                    </>
                }

            </div>
        </div>
    )
}
