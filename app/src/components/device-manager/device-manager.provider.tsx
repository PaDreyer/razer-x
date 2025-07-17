import DeviceManagerContext, { type DeviceManagerContextState } from "./device-manager.context.tsx";
import type { PropsWithChildren, ReactNode } from "react";
import { memo } from "react";
import { IDeviceManagerApi } from "./types.ts";


/**
 * Props for the DeviceManagerProvider component, which provides access to the device management API.
 */
export type DeviceManagerProviderProps = {
    api: IDeviceManagerApi,
}

/**
 * DeviceManagerProvider that provides the device management API context to its children.
 * This allows child components to access device management functions and state.
 *
 * @param {DeviceManagerProviderProps} props - The properties for the DeviceManagerProvider component.
 * @returns {JSX.Element} The rendered DeviceManager component.
 */
function DeviceManagerProviderComponent(props: PropsWithChildren<DeviceManagerProviderProps>): ReactNode {
    const value: DeviceManagerContextState = {
        api: props.api,
    };

    return (
        <DeviceManagerContext.Provider value={value}>
            { props.children }
        </DeviceManagerContext.Provider>
    );
}

export const DeviceManagerProvider =  memo(DeviceManagerProviderComponent);