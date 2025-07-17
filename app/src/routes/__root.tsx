import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import "../App.css";
import {Toaster} from "react-hot-toast";
import {
    DeviceManagerProvider,
    IDeviceInformation,
    IDeviceManagerApi,
    PossiblePollingRates
} from "../components/device-manager";



export const Route = createRootRoute({
    component: () => (
        <>
            <Toaster
                position={"bottom-left"}
            />
            <DeviceManagerProvider api={{
                async setPollingRate(pollingRate: PossiblePollingRates): Promise<void> {
                console.log(`Setting polling rate to ${pollingRate} Hz`);
            },
                async setDpiXY(dpiX: number, dpiY: number): Promise<void> {
                console.log(`Setting DPI to X: ${dpiX}, Y: ${dpiY}`);
            },
                async getBatteryLevel(): Promise<number> {
                console.log("Fetching battery level");
                return 100; // Mocked value
            },
                async setBacklightBrightness(brightness: number): Promise<void> {
                console.log(`Setting backlight brightness to ${brightness}`);
            },
                async setBacklightColor(color: { r: number; g: number; b: number }): Promise<void> {
                console.log(`Setting backlight color to R: ${color.r}, G: ${color.g}, B: ${color.b}`);
            },
                async setMatrixBehavior(behavior: 'none' | 'static'): Promise<void> {
                console.log(`Setting matrix behavior to ${behavior}`);
            },
                async getDeviceInformation(): Promise<IDeviceInformation> {
                return {
                batteryLevel: 100,
                pollingRate: 1000,
                dpiXY: [800, 800],
                backlightBrightness: 50,
                backlightColor: { r: 255, g: 255, b: 255 },
                matrixBehavior: 'none',
            }
            },
                // async setSmartWheelEnabled(enabled: boolean): Promise<void> {},
            }}>
                <Outlet />
            </DeviceManagerProvider>
            <TanStackRouterDevtools />
        </>
    ),
})