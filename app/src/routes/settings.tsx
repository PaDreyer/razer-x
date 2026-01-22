import { createFileRoute, Link } from '@tanstack/react-router'
import { useDeviceManager } from "../components/device-manager";
import { Checkbox } from "../components/checkbox/checkbox";
import { ReactNode, useEffect, useState } from "react";
import { getVersion } from '@tauri-apps/api/app';

export const Route = createFileRoute('/settings')({
    component: SettingsPage,
})

interface SettingRowProps {
    label: string;
    description?: string;
    action: ReactNode;
    icon?: ReactNode;
}

function SettingRow({ label, description, action, icon }: SettingRowProps) {
    return (
        <div className="flex items-center justify-between py-4 px-6 hover:bg-white/[0.02] transition-colors">
            <div className="flex items-center gap-4">
                {icon && <div className="text-gray-400">{icon}</div>}
                <div className="flex flex-col">
                    <span className="text-white font-medium">{label}</span>
                    {description && <span className="text-sm text-gray-400">{description}</span>}
                </div>
            </div>
            <div>{action}</div>
        </div>
    );
}

function SettingsGroup({ title, children }: { title?: string; children: ReactNode }) {
    return (
        <div className="mb-8">
            {title && (
                <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 ml-6">
                    {title}
                </h3>
            )}
            <div className="bg-gray-800/40 backdrop-blur-md border border-white/5 rounded-2xl overflow-hidden divide-y divide-white/5">
                {children}
            </div>
        </div>
    );
}

function SettingsPage() {
    const deviceManager = useDeviceManager();
    const [version, setVersion] = useState<string>('...');

    useEffect(() => {
        getVersion().then(setVersion).catch((e) => console.log(e) || setVersion('Unknown'));
    }, []);

    if (!deviceManager) return null;

    const { appSettings, updateAppSettings } = deviceManager;

    const handleAutoUpdateChange = (enabled: boolean) => {
        if (appSettings) {
            updateAppSettings({
                ...appSettings,
                autoUpdate: enabled
            });
        }
    };

    return (
        <div className="h-screen w-full bg-[#0a0a0a] text-white overflow-hidden relative font-sans selection:bg-blue-500/30">
            {/* Ambient Background Blobs - Mirroring main page for consistency */}
            <div className="absolute inset-0 overflow-hidden pointer-events-none">
                <div className="absolute top-0 -left-4 w-72 h-72 bg-blue-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-10 animate-blob"></div>
                <div className="absolute top-0 -right-4 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-10 animate-blob animation-delay-2000"></div>
            </div>

            <div className="h-full w-full overflow-y-scroll overflow-x-hidden pl-6 pr-[16px] pt-10 pb-12 relative z-10">
                <div className="max-w-4xl mx-auto mt-0 px-4">
                    <div className="flex items-center gap-6 mb-12">
                        <Link
                            to="/"
                            className="p-3 rounded-full bg-white/5 hover:bg-white/10 transition-all active:scale-95 text-white border border-white/5 shadow-xl"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6" /></svg>
                        </Link>
                        <h2 className="text-5xl font-bold tracking-tight">Settings</h2>
                    </div>

                    <SettingsGroup title="General">
                        <SettingRow
                            label="Automatic Updates"
                            description="Check for and install updates on application startup"
                            icon={
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" /><path d="M3 3v5h5" /><path d="m7 11 2-2" /><path d="m7 11 2 2" /><path d="M20.4 15.6A9 9 0 0 1 12 21a9.75 9.75 0 0 1-6.74-2.74L3 16" /><path d="M16 16h5v5" /></svg>
                            }
                            action={
                                <div className="scale-110 origin-right">
                                    <Checkbox
                                        checked={appSettings?.autoUpdate ?? false}
                                        onChange={handleAutoUpdateChange}
                                    />
                                </div>
                            }
                        />
                        {deviceManager.deviceInformation?.targetOs === 'macos' && (
                            <SettingRow
                                label="Natural Scrolling"
                                description="Reverse scroll direction according to macOS system preference"
                                icon={
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m7 15 5 5 5-5" /><path d="m7 9 5-5 5 5" /></svg>
                                }
                                action={
                                    <div className="scale-110 origin-right">
                                        <Checkbox
                                            checked={deviceManager.deviceInformation?.mouseWheelInverted ?? false}
                                            onChange={(checked) => deviceManager.setMouseWheelInverted(checked)}
                                        />
                                    </div>
                                }
                            />
                        )}
                    </SettingsGroup>

                    <SettingsGroup title="About">
                        <SettingRow
                            label="Version"
                            description="View currently installed version"
                            icon={
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10" /><path d="M12 16v-4" /><path d="M12 8h.01" /></svg>
                            }
                            action={
                                <span className="text-sm font-medium text-gray-500 mr-2">{version}</span>
                            }
                        />
                    </SettingsGroup>
                </div>
            </div>
        </div>
    );
}
