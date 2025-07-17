import { useCallback, useContext, useEffect, useState } from "react";
import { DeviceManagerContext } from "./device-manager.context.tsx";
import { ErrorState, IDeviceInformation, PossiblePollingRates } from "./types.ts";

/**
 * Custom hook to manage device settings and state for a gaming mouse or similar device.
 * @returns {object} Device management functions and state.
 */
export const useDeviceManager = () => {
    const { api } = useContext(DeviceManagerContext);
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
    }, []);

    const setPollingRate = useCallback((pollingRate: number) => {
        if (![125, 250, 500, 1000].includes(pollingRate)) {
            throw new Error("Invalid polling rate. Must be one of: 125, 250, 500, 1000");
        }

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

    const updateBatteryLevel = useCallback(() => {
        return api.getBatteryLevel()
            .then((batteryLevel) => {
                setDeviceInformation(prev => prev ? { ...prev, batteryLevel } : null);
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

    const setMatrixBehavior = useCallback((behavior: 'none' | 'static') => {
        return api.setMatrixBehavior(behavior)
            .then(() => {
                setDeviceInformation(prev => prev ? { ...prev, matrixBehavior: behavior } : null);
            })
            .catch(handleError);
    }, [ api ]);

    // const setSmartWheelEnabled = useCallback((enabled: boolean) => {
    //     return api.setSmartWheelEnabled(enabled)
    //         .then(() => {
    //             setDeviceInformation(prev => prev ? { ...prev, smartWheelEnabled: enabled } : null);
    //         })
    //         .catch(handleError);
    // }, [ api ]);

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
    }, []);

    // Update battery level periodically
    useEffect(() => {
        //const intervalTimer = setInterval(updateBatteryLevel, 5000);

        //return () => clearTimeout(intervalTimer);
    }, [])

    return {
        deviceInformation,
        isLoading,
        error,
        isInitialized,
        setBacklightBrightness,
        setBacklightColor,
        setDpiXY,
        setPollingRate,
        setMatrixBehavior,
    }
}