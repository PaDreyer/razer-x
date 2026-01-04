import DeviceManagerContext, { type DeviceManagerContextState } from "./device-manager.context.tsx";
import { PropsWithChildren, ReactNode, useCallback, useEffect, useState } from "react";
import { memo } from "react";
import { ErrorState, IDeviceInformation, IDeviceManagerApi, PossiblePollingRates } from "./types.ts";


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
    const api = props.api
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setIsError] = useState<ErrorState>({
        isError: false,
        message: null,
    });
    const [isInitialized, setIsInitialized] = useState<boolean>(false);
    const [deviceInformation, setDeviceInformation] = useState<IDeviceInformation | null>(null);

    const handleError = useCallback((error: any) => {
        setIsError({
            isError: true,
            message: error?.toString() ?? "Unknown error"
        })
        throw new Error(error?.toString() ?? "Unknown error");
    }, []);

    const setPollingRate = useCallback((pollingRate: number) => {
        if (![125, 250, 500, 1000].includes(pollingRate)) {
            throw new Error("Invalid polling rate. Must be one of: 125, 250, 500, 1000");
        }

        console.log("pollingRate: ", pollingRate)

        return api.setPollingRate(pollingRate as PossiblePollingRates)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, pollingRate: pollingRate as PossiblePollingRates } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const setDpiXY = useCallback((dpiX: number, dpiY: number) => {
        return api.setDpiXY(dpiX, dpiY)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, dpiX, dpiY } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const setBacklightBrightness = useCallback((brightness: number) => {
        return api.setBacklightBrightness(brightness)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, backlightBrightness: brightness } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const setBacklightColor = useCallback((color: { r: number; g: number; b: number }) => {
        return api.setBacklightColor(color)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, backlightColor: color } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const setSmartWheelEnabled = useCallback((enabled: boolean) => {
        return api.setSmartWheelEnabled(enabled)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, smartWheelEnabled: enabled } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const setMouseWheelInverted = useCallback((inverted: boolean) => {
        return api.setMouseWheelInverted(inverted)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, mouseWheelInverted: inverted } : null);
            })
            .catch(handleError);
    }, [ api ]);

    const getDpiStages = useCallback(async () => {
        return api.getDpiStages()
            .catch(handleError);
    }, [ api ]);

    const loadDeviceInformation = useCallback(async () => {
        const deviceInformation = await api.getDeviceInformation()
            .catch(handleError);

        if (deviceInformation) {
            setDeviceInformation(deviceInformation);
        }
    }, [ api ]);

    // Initialize device information on first render
    useEffect(() => {
        if (!isInitialized) {
            setIsLoading(true);
            loadDeviceInformation()
                .then(() => {
                    setIsInitialized(true);
                })
                .catch(handleError)
                .finally(() => {
                    setIsLoading(false);
                });
        }

        return () => {
            console.log("Cleaning up device manager provider (again...)");
        }
    }, []);

    // Update battery level periodically
    useEffect(() => {
        //const intervalTimer = setInterval(updateBatteryLevel, 5000);

        //return () => clearTimeout(intervalTimer);
    }, [])

    const value: DeviceManagerContextState = {
        api: props.api,
        deviceInformation,
        isLoading,
        error,
        isInitialized,
        setBacklightBrightness,
        setBacklightColor,
        setDpiXY,
        setPollingRate,
        setMouseWheelInverted,
        setSmartWheelEnabled,
        getDpiStages,
    };

    return (
        <DeviceManagerContext.Provider value={value}>
            { props.children }
        </DeviceManagerContext.Provider>
    );
}

export const DeviceManagerProvider =  memo(DeviceManagerProviderComponent);