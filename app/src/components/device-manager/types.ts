export interface IDeviceInformation {
    batteryLevel: number;
    isCharging: boolean;
    pollingRate: PossiblePollingRates;
    dpiXy: [number, number];
    backlightBrightness: number;
    backlightColor: RGBColor;
    matrixBehavior: PossibleMatrixBehaviors;
    targetOs: TargetOs;
    smartWheelEnabled: boolean;
    mouseWheelInverted: boolean;
    dpiStages: Array<DpiStage>;
}

export type TargetOs = 'windows' | 'linux' | 'macos' | 'unknown';
export type PossiblePollingRates = 125 | 250 | 500 | 1000;
export type PossibleMatrixBehaviors = 'none' | 'static';

export type RGBColor = { r: number; g: number; b: number };
export type DpiStage = {
    dpiX: number;
    dpiY: number;
    stage: number;
    active: boolean
}

export interface IDeviceManagerApi {
    getDeviceInformation(): Promise<IDeviceInformation>;
    setPollingRate(pollingRate: PossiblePollingRates): Promise<void>;
    setDpiXy(dpiX: number, dpiY: number): Promise<void>;
    getDpiStages(): Promise<Array<DpiStage>>;
    setDpiStages(stages: Array<DpiStage>): Promise<void>;
    getBatteryLevel(): Promise<number>;
    getChargingStatus(): Promise<boolean>;
    setBacklightBrightness(brightness: number): Promise<void>;
    setBacklightColor(color: RGBColor): Promise<void>;
    getTargetOs(): Promise<TargetOs>;
    setSmartWheelEnabled?(enabled: boolean): Promise<void>;
    setMouseWheelInverted(inverted: boolean): Promise<void>;
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
