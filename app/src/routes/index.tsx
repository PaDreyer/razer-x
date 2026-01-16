import { createFileRoute, Link } from '@tanstack/react-router'
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

                <div className="flex flex-wrap-reverse flex-row justify-between items-center mb-8">
                    <h2 className="text-4xl font-semibold mt-4" >Razer Basilisk V3 Pro</h2>
                    <div className="flex gap-4 items-center">
                        {shouldShow &&
                            <BatteryStatus batteryLevel={batteryLevel} isCharging={isCharging} />
                        }
                        <Link
                            to="/settings"
                            className="p-2 rounded-full hover:bg-white/10 transition-colors text-white"
                            title="Settings"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" /><circle cx="12" cy="12" r="3" /></svg>
                        </Link>
                    </div>
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
