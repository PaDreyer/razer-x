import { createContext } from "react";
import { IDeviceManagerApi } from "./types.ts";


/**
 * Context state for the Device Manager, providing access to the device management API.
 */
export type DeviceManagerContextState = {
    api: IDeviceManagerApi;
};

/**
 * Default context state for the Device Manager, initialized with a proxy that throws errors on access.
 * This ensures that any attempt to use the API without proper initialization will result in an error.
 */
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