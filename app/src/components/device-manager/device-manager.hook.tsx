import { useContext } from "react";
import { DeviceManagerContext } from "./device-manager.context.tsx";

/**
 * Custom hook to manage device settings and state for a gaming mouse or similar device.
 * @returns {object} Device management functions and state.
 */
export const useDeviceManager = () => {
    const {
        deviceInformation,
        isLoading,
        error,
        isInitialized,
        setBacklightBrightness,
        setBacklightColor,
        setDpiXy,
        setPollingRate,
        setMouseWheelInverted,
        //         setSmartWheelEnabled,
        getDpiStages,
        setDpiStages,
        appSettings,
        updateAppSettings,
    } = useContext(DeviceManagerContext);

    return {
        deviceInformation,
        isLoading,
        error,
        isInitialized,
        setBacklightBrightness,
        setBacklightColor,
        setDpiXy,
        setPollingRate,
        setMouseWheelInverted,
        //         setSmartWheelEnabled,
        getDpiStages,
        setDpiStages,
        appSettings,
        updateAppSettings,
    }
}