import { SliderComponentHandle, SliderExtended } from "../components/slider-extended";
import { RGBColor, useDeviceManager } from "../components/device-manager";
import { useCallback, useRef } from "react";
import { NormalButton } from "../components/button";
import { PopoverColorPicker } from "../components/popover-colorpicker";

export const RgbSettings = () => {
    const sliderRef = useRef<SliderComponentHandle>(null);
    const deviceManager = useDeviceManager();

    const setSliderValue = useCallback((number: number) => {
        sliderRef.current?.setValueExtern(number, true);
    }, [sliderRef.current])

    if (!deviceManager) {
        console.log("DeviceManager not available. Ensure you are using the DeviceManagerProvider.");
        return null;
    }

    const {
        isLoading,
        isInitialized,
        error,
        setBacklightBrightness,
        setBacklightColor,
        deviceInformation,
    } = deviceManager;

    if (!deviceInformation || isLoading || !isInitialized) {
        return null;
    }

    if (error.isError) {
        console.error(`Fehler: ${error.message}`);
        return null;
    }

    const {
        backlightBrightness,
        backlightColor,
    } = deviceInformation!;

    return (
        <div className="flex-1 flex flex-row items-center justify-around w-full gap-8 h-full">
            <div className="flex flex-col items-start min-w-[240px] justify-center h-full">
                <div className="w-full px-2">
                    <SliderExtended
                        ref={sliderRef}
                        debounceDelay={500}
                        min={0}
                        max={100}
                        step={1}
                        onChange={value => setBacklightBrightness(value)}
                        className="w-full"
                        initialValue={backlightBrightness}
                    />

                    <div className="flex flex-row justify-between gap-2 mt-8">
                        {[0, 5, 25, 75, 100].map(val => (
                            <NormalButton
                                key={val}
                                text={`${val}%`}
                                onClick={() => setSliderValue(val)}
                                active={backlightBrightness === val}
                            />
                        ))}
                    </div>
                </div>
            </div>

            {/* Vertical Divider */}
            <div className="h-2/3 w-px bg-white/10" />

            {/* Right Side: Color Selection */}
            <div className="flex flex-col items-center justify-center h-full">
                <PopoverColorPicker
                    color={backlightColor}
                    onChange={(color) => setBacklightColor(color as RGBColor)}
                    presetColors={[
                        { r: 255, g: 255, b: 255 },
                        { r: 0, g: 255, b: 22 },
                        { r: 0, g: 15, b: 255 },
                        { r: 255, g: 0, b: 0 },
                    ]}
                />
            </div>
        </div>
    )
}