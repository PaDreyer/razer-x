import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import "../App.css";
import { Toaster } from "react-hot-toast";
import {
    DeviceManagerProvider,
    IDeviceInformation,
    PossiblePollingRates,
    DpiStage
} from "../components/device-manager";
import { invoke } from "@tauri-apps/api/core";
import UpdateOverlay from "../components/update-overlay/UpdateOverlay";

export const Route = createRootRoute({
    component: () => (
        <>
            <Toaster
                position={"bottom-left"}
            />
            <DeviceManagerProvider api={{
                async getTargetOs(): Promise<"windows" | "linux" | "macos" | "unknown"> {
                    console.log("Fetching target OS");
                    return invoke<"windows" | "linux" | "macos" | "unknown">('get_target_os');
                },
                async setMouseWheelInverted(inverted: boolean): Promise<void> {
                    console.log(`Setting mouse wheel inverted to ${inverted}`);
                    return invoke('set_mouse_wheel_inverted', { inverted });
                },
                async setSmartWheelEnabled(enabled: boolean): Promise<void> {
                    console.log(`Setting smart wheel enabled to ${enabled}`);
                    return invoke('set_device_smart_wheel', { enabled });
                },
                async setPollingRate(pollingRate: PossiblePollingRates): Promise<void> {
                    console.log(`Setting polling rate to ${pollingRate} Hz`);
                    return invoke('set_device_polling_rate', { pollingRate });
                },
                async setDpiXy(dpiX: number, dpiY: number): Promise<void> {
                    console.log(`Setting DPI to X: ${dpiX}, Y: ${dpiY}`);
                    return invoke('set_device_dpi', { dpiX, dpiY });
                },
                async getBatteryLevel(): Promise<number> {
                    console.log("Fetching battery level");
                    return invoke<number>("get_device_battery_status");
                },
                async getChargingStatus(): Promise<boolean> {
                    console.log("Fetching charging status");
                    return invoke<boolean>("get_device_charging_status");
                },
                async setBacklightBrightness(brightness: number): Promise<void> {
                    console.log(`Setting backlight brightness to ${brightness}`);
                    return invoke<void>("set_device_backlight_brightness", { brightness });
                },
                async setBacklightColor(color: { r: number; g: number; b: number }): Promise<void> {
                    console.log(`Setting backlight color to R: ${color.r}, G: ${color.g}, B: ${color.b}`);
                    return invoke<void>("set_device_matrix_backlight_static", { ...color });
                },
                async getDpiStages(): Promise<Array<DpiStage>> {
                    console.log('Fetching dpi stages');
                    return invoke("get_device_dpi_stages");
                },
                async setDpiStages(stages: Array<DpiStage>): Promise<void> {
                    console.log('Setting DPI stages:', stages);
                    return invoke("set_device_dpi_stages", { stages });
                },
                async getDeviceInformation(): Promise<IDeviceInformation> {
                    const result = await invoke<string>('get_device_information');
                    const data = JSON.parse(result) as IDeviceInformation;
                    console.log("Device information fetched:", JSON.parse(result));
                    console.log("Fetching device information");
                    return data;
                },
            }}>
                <Outlet />
                <UpdateOverlay />
            </DeviceManagerProvider>
            <TanStackRouterDevtools />
        </>
    ),
})