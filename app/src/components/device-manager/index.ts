export { DeviceManagerProvider } from "./device-manager.provider.tsx";
export type { DeviceManagerProviderProps } from "./device-manager.provider.tsx";

export { useDeviceManager } from "./device-manager.hook.tsx";

export type {
    IDeviceManagerApi,
    PossiblePollingRates,
    PossibleMatrixBehaviors,
    RGBColor,
    ErrorState,
    OkState,
    FailureState,
    IDeviceInformation,
    DpiStage,
    IAppSettings,
} from "./types.ts";