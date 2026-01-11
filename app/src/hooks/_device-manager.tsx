import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export type PossiblePollingRates = 125 | 250 | 500 | 1000;
export type PossibleMatrixBehaviors = 'none' | 'static';

export type RGBColor = { r: number; g: number; b: number };

export type UseDeviceManagerProps = {};

/**
 * Custom hook to manage device settings and state for a gaming mouse or similar device.
 * @param _
 * @deprecated use components/device-manager.tsx instead
 */
export function useDeviceManager(_: UseDeviceManagerProps) {
    const [pollingRate, setPollingRate] = useState<PossiblePollingRates | undefined>();
    const [dpiXy, setDpiXy] = useState<[number, number] | undefined>();
    const [batteryLevel, setBatteryLevel] = useState<number | undefined>();
    const [backlightBrightness, setBacklightBrightness] = useState<number | undefined>();
    const [backlightColor, setBacklightColor] = useState<RGBColor | undefined>();
    const [matrixBehavior, setMatrixBehavior] = useState<PossibleMatrixBehaviors | undefined>();
    const [smartWheelEnabled, setSmartWheelEnabled] = useState<boolean | undefined>();
    const [isError, setIsError] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [isInitialized, setInitialized] = useState(false);

    const safeInvoke = useCallback(async function <T = unknown>(
        command: string,
        args?: Record<string, any>,
        callback?: (data: T) => void,
        ignoreLoadingState = false
    ): Promise<void> {
        if (!ignoreLoadingState) setIsLoading(true);

        try {
            const data = await invoke<T>(command, args);
            callback?.(data);
        } catch (error) {
            setIsError(true);
        } finally {
            if (!ignoreLoadingState) setIsLoading(false);
        }
    }, []);

    // Generic state sync effect
    function useSyncEffect<T>(
        value: T | undefined,
        command: string,
        argBuilder: (v: T) => Record<string, any>
    ) {
        useEffect(() => {
            if (value !== undefined) {
                safeInvoke(command, argBuilder(value));
            }
        }, [value, command, argBuilder]);
    }

    useSyncEffect(pollingRate, "set_device_polling_rate", v => ({ polling_rate: v }));
    useSyncEffect(dpiXy, "set_device_dpi", ([x, y]) => ({ dpi_x: x, dpi_y: y }));
    useSyncEffect(backlightBrightness, "set_device_backlight_brightness", v => ({ brightness: v }));
    useSyncEffect(backlightColor, "set_device_backlight_color", v => ({ color: v }));
    useSyncEffect(matrixBehavior, "set_device_matrix_behavior", v => ({ behavior: v }));
    useSyncEffect(smartWheelEnabled, "set_device_smartwheel_enabled", v => ({ enabled: v }));

    // Initial fetch
    useEffect(() => {
        safeInvoke<string>("get_device_information", undefined, (rawData) => {
            try {
                const { polling_rate, dpi_xy, battery_level } = JSON.parse(rawData);
                setPollingRate(polling_rate);
                setDpiXy(dpi_xy);
                setBatteryLevel(battery_level);
            } catch (e) {
                setIsError(true);
            } finally {
                setInitialized(true);
            }
        });
    }, [safeInvoke]);

    return {
        pollingRate, setPollingRate,
        dpiXy, setDpiXy,
        batteryLevel,
        backlightBrightness, setBacklightBrightness,
        backlightColor, setBacklightColor,
        matrixBehavior, setMatrixBehavior,
        smartWheelEnabled, setSmartWheelEnabled,
        isError,
        isLoading,
        isInitialized,
    };
}