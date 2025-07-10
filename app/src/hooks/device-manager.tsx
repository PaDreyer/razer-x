import * as React from "react";
import {useEffect} from "react";
import {invoke} from "@tauri-apps/api/core";

export type UseDeviceManagerProps = {};
type PossiblePollingRates = 125 | 250 | 500 | 1000;
type PossibleMatrixBehaviors = 'none' | 'static';

/**
 * Custom hook to manage device settings such as polling rate, DPI, battery level, backlight brightness, color,
 * matrixBehavior and smart wheel state.
 * @param props
 */
export function useDeviceManager(props: UseDeviceManagerProps) {
    const [pollingRate, setPollingRate] = React.useState<undefined | keyof PossiblePollingRates>(undefined);
    const [dpiXY, setDpiXY] = React.useState<undefined | [number, number]>(undefined);
    const [batteryLevel, setBatteryLevel] = React.useState<undefined | number>(undefined);
    const [backlightBrightness, setBacklightBrightness] = React.useState<undefined | number>(undefined);
    const [backlightColor, setBacklightColor] = React.useState<undefined | {r: number; g: number; b: number;}>(undefined);
    const [matrixBehavior, setMatrixBehavior] = React.useState<undefined | keyof PossibleMatrixBehaviors>(undefined);
    const [smartWheelEnabled, setSmartWheelEnabled] = React.useState<undefined | boolean>(undefined);

    // Update device polling rate when pollingRate changes
    useEffect(() => {
        if (pollingRate !== undefined) {
            invoke('set_device_polling_rate', { polling_rate: pollingRate });
        }
    }, [pollingRate]);

    // Update device DPI when dpiXY changes
    useEffect(() => {
        if (dpiXY !== undefined) {
            invoke('set_device_dpi', { dpi_x: dpiXY[0], dpi_y: dpiXY[1] });
        }
    }, [dpiXY]);

    // Update device backlight brightness when backlightBrightness changes
    useEffect(() => {
        if (backlightBrightness !== undefined) {
            invoke('set_device_backlight_brightness', { brightness: backlightBrightness });
        }
    }, [backlightBrightness]);

    // Update device backlight color when deviceBacklightColor changes
    useEffect(() => {
        if (backlightColor !== undefined) {
            invoke('set_device_backlight_color', { color: backlightColor });
        }
    }, [backlightColor]);

    // Update device matrix behavior when matrixBehavior changes
    useEffect(() => {
        if (matrixBehavior !== undefined) {
            invoke('set_device_matrix_behavior', { behavior: matrixBehavior });
        }
    }, [matrixBehavior]);

    // Update device smartwheel state when smartwheelEnabled changes
    useEffect(() => {
        if (smartWheelEnabled !== undefined) {
            invoke('set_device_smartwheel_enabled', { enabled: smartWheelEnabled });
        }
    }, [smartWheelEnabled]);

    // Fetch initial device information
    useEffect(() => {
        invoke<string>('get_device_information')
            .then(rawData => {
                const { polling_rate, dpi_xy, battery_level } = JSON.parse(rawData);
                setPollingRate(polling_rate);
                setDpiXY(dpi_xy);
                setBatteryLevel(battery_level);
            })
            .catch((error) => {
                console.error("Failed to get device information:", error);
            });
    }, []);

    return {
        pollingRate,
        setPollingRate,
        dpiXY,
        setDpiXY,
        batteryLevel,
        backlightBrightness,
        setBacklightBrightness,
        setBacklightColor,
        matrixBehavior,
        setMatrixBehavior,
        smartWheelEnabled,
        setSmartWheelEnabled,
    };
}