export interface IDeviceInformation {
    batteryLevel: number;
    pollingRate: PossiblePollingRates;
    dpiXY: [number, number];
    backlightBrightness: number;
    backlightColor: RGBColor;
    matrixBehavior: PossibleMatrixBehaviors;
    // smartWheelEnabled: boolean;
}

export type PossiblePollingRates = 125 | 250 | 500 | 1000;
export type PossibleMatrixBehaviors = 'none' | 'static';

export type RGBColor = { r: number; g: number; b: number };

export interface IDeviceManagerApi {
    getDeviceInformation(): Promise<IDeviceInformation>;
    setPollingRate(pollingRate: PossiblePollingRates): Promise<void>;
    setDpiXY(dpiX: number, dpiY: number): Promise<void>;
    getBatteryLevel(): Promise<number>;
    setBacklightBrightness(brightness: number): Promise<void>;
    setBacklightColor(color: RGBColor): Promise<void>;
    setMatrixBehavior(behavior: PossibleMatrixBehaviors): Promise<void>;
    // setSmartWheelEnabled(enabled: boolean): Promise<void>;
}

export type OkState = {
    isError: false;
    message: null;
};
export type FailureState = {
    isError: true;
    message: string;
};
export type ErrorState = OkState | FailureState;
