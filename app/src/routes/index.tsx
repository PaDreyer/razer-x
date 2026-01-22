import { createFileRoute, Link } from '@tanstack/react-router'
import { useDeviceManager } from "../components/device-manager";
import { DpiSettings } from "../views/dpi-settings.tsx";
import { Panel } from "../components/panel";
import { BatteryStatus } from "../components/battery-status";
import { RgbSettings } from "../views/rgb-settings.tsx";
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

    if (deviceManager.isLoading && !deviceManager.isInitialized) {
        return (
            <div className="h-screen w-full bg-[#0a0a0a] flex items-center justify-center">
                <div className="flex flex-col items-center gap-4">
                    <div className="w-12 h-12 border-4 border-blue-500/20 border-t-blue-500 rounded-full animate-spin"></div>
                    <p className="text-white/50 animate-pulse">Initializing Device...</p>
                </div>
            </div>
        );
    }

    if (deviceManager.error.isError) {
        return (
            <div className="h-screen w-full bg-[#0a0a0a] flex items-center justify-center p-6 text-center">
                <div className="max-w-md p-8 rounded-3xl bg-white/5 border border-white/10 shadow-2xl backdrop-blur-xl">
                    <div className="w-16 h-16 bg-red-500/20 rounded-full flex items-center justify-center mx-auto mb-6">
                        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ef4444" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10" /><line x1="12" y1="8" x2="12" y2="12" /><line x1="12" y1="16" x2="12.01" y2="16" /></svg>
                    </div>
                    <h2 className="text-2xl font-bold mb-4">Initialisierungsfehler</h2>
                    <p className="text-white/60 mb-8 leading-relaxed">
                        {deviceManager.error.message}
                    </p>
                    <button
                        onClick={() => window.location.reload()}
                        className="px-8 py-3 bg-white/10 hover:bg-white/20 transition-all rounded-xl font-bold active:scale-95"
                    >
                        Erneut versuchen
                    </button>
                </div>
            </div>
        );
    }

    if (!deviceManager.deviceInformation) {
        return (
            <div className="h-screen w-full bg-[#0a0a0a] flex items-center justify-center text-white/50">
                Warte auf Geräte-Informationen...
            </div>
        );
    }

    const { batteryLevel, isCharging } = deviceManager.deviceInformation;

    const shouldShow = deviceManager.isInitialized && !deviceManager.error.isError;


    return (
        <div className="h-screen w-full bg-[#0a0a0a] text-white overflow-hidden relative font-sans selection:bg-blue-500/30">
            {/* Ambient Background Blobs - Isolated to prevent scroll issues */}
            <div className="absolute inset-0 overflow-hidden pointer-events-none">
                <div className="absolute top-0 -left-4 w-72 h-72 bg-blue-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob"></div>
                <div className="absolute top-0 -right-4 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob animation-delay-2000"></div>
                <div className="absolute -bottom-8 left-20 w-72 h-72 bg-indigo-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob animation-delay-4000"></div>
            </div>

            {/* Scrollable Content Container */}
            <div className="h-full w-full overflow-y-scroll overflow-x-hidden pl-6 pr-[16px] pt-10 pb-12 relative z-10">
                <div className="max-w-4xl mx-auto mt-0 px-4">
                    <div className="flex flex-wrap-reverse flex-row justify-between items-end mb-10 px-2">
                        <div className="flex flex-col gap-1">
                            <span className="text-blue-500 font-bold tracking-widest text-xs uppercase">Device Dashboard</span>
                            <h2 className="text-4xl font-extrabold tracking-tighter">Razer Basilisk V3 Pro</h2>
                        </div>
                        <div className="flex gap-6 items-center mb-2">
                            {shouldShow &&
                                <BatteryStatus batteryLevel={batteryLevel} isCharging={isCharging} />
                            }
                            <Link
                                to="/settings"
                                className="p-3 rounded-full bg-white/5 hover:bg-white/10 transition-all active:scale-95 text-white border border-white/5 shadow-2xl backdrop-blur-xl"
                                title="Settings"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" /><circle cx="12" cy="12" r="3" /></svg>
                            </Link>
                        </div>
                    </div>

                    {shouldShow &&
                        <>
                            <div className="grid grid-cols-6 gap-6">
                                {/* Left Column: RGB (now wider) */}
                                <div className="col-span-6 md:col-span-4">
                                    <Panel className="h-full flex flex-col relative z-20">
                                        <Panel.Header className="text-sm font-bold text-gray-200 uppercase tracking-wider">
                                            RGB Einstellungen
                                        </Panel.Header>
                                        <Panel.Body className="flex-1">
                                            <RgbSettings />
                                        </Panel.Body>
                                    </Panel>
                                </div>

                                {/* Right Column: Polling (now vertical) */}
                                <div className="col-span-6 md:col-span-2 flex flex-col gap-6">
                                    <Panel className="h-full flex flex-col">
                                        <Panel.Header className="text-sm font-bold text-gray-200 uppercase tracking-wider">
                                            Polling Rate
                                        </Panel.Header>
                                        <Panel.Body className="flex-1 flex flex-col justify-center">
                                            <PollingRateSettings />
                                        </Panel.Body>
                                    </Panel>
                                </div>

                                {/* Footer: DPI */}
                                <Panel className="col-span-6">
                                    <Panel.Header className="text-sm font-bold text-gray-200 uppercase tracking-wider">
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
        </div>
    )
}
