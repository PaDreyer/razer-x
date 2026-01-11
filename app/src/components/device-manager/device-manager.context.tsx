import { createContext } from "react";
import { IDeviceInformation, IDeviceManagerApi, DpiStage } from "./types.ts";


/**
 * Context state for the Device Manager, providing access to the device management API.
 */
export type DeviceManagerContextState = {
    api: IDeviceManagerApi;
    deviceInformation: IDeviceInformation | null;
    isLoading: boolean;
    error: { isError: boolean; message: string | null };
    isInitialized: boolean;
    setBacklightBrightness: (brightness: number) => Promise<void>;
    setBacklightColor: (color: { r: number; g: number; b: number }) => Promise<void>;
    setDpiXy: (dpiX: number, dpiY: number) => Promise<void>;
    setPollingRate: (pollingRate: number) => Promise<void>;
    setMouseWheelInverted: (inverted: boolean) => Promise<void>;
    setSmartWheelEnabled: (enabled: boolean) => Promise<void>;
    getDpiStages: () => Promise<Array<DpiStage>>;
    setDpiStages: (stages: Array<DpiStage>) => Promise<void>;
};

/**
 * Default context state for the Device Manager, initialized with a proxy that throws errors on access.
 * This ensures that any attempt to use the API without proper initialization will result in an error.
 */
// TODO: Fix this default implementation to provide a more meaningful default state.
// @ts-ignore
export const DefaultDeviceManagerContext: DeviceManagerContextState = {
    api: new Proxy({}, { get() { throw new Error() }, set() { throw new Error() } }) as IDeviceManagerApi,
};

/**
 * Context for managing device settings and state.
 */
export const DeviceManagerContext = createContext<DeviceManagerContextState>(
    DefaultDeviceManagerContext
);
export const DeviceManagerProvider = DeviceManagerContext.Provider;
export const DeviceManagerConsumer = DeviceManagerContext.Consumer;

export default {
    Provider: DeviceManagerProvider,
    Consumer: DeviceManagerConsumer,
}