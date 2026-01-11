import {SliderComponentHandle, SliderExtended} from "../components/slider-extended";
import {PopoverColorPicker} from "../components/popover-colorpicker";
import {RGBColor, useDeviceManager} from "../components/device-manager";
import {useCallback, useRef} from "react";
import {NormalButton} from "../components/button";

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
        <div className="flex-1 flex flex-col justify-center bg-gray-800 rounded-lg">
            <h4 className="font-medium text-gray-200">
                Helligkeit
            </h4>
            <div className="flex flex-col justify-between w-full px-4 mb-1">
                <SliderExtended
                    ref={sliderRef}
                    debounceDelay={500}
                    min={0}
                    max={100}
                    step={1}
                    onChange={value => setBacklightBrightness(value)}
                    className="flex-1"
                    initialValue={backlightBrightness}
                />

                <div className="flex justify-center gap-3 mt-6 mb-1 flex-wrap">
                    {[0, 5, 25, 75, 100].map(val => <NormalButton
                        key={val}
                        text={`${val}%`}
                        onClick={() => setSliderValue(val)}
                        active={backlightBrightness === val}
                        />
                    )}
                </div>
            </div>

            <hr className="my-4 text-gray-700 border h-1" bg-gray-700/>
            <h4 className="font-medium text-gray-200 mb-1">
                Farbe
            </h4>
            <PopoverColorPicker
                className="mb-4"
                color={backlightColor}
                onChange={(color) => setBacklightColor(color as RGBColor)}
                presetColors={[
                    {r: 255, g: 255, b: 255},
                    {r: 0, g: 255, b: 22},
                    {r: 0, g: 15, b: 255},
                    {r: 255, g: 0, b: 0},
                ]}
            />
        </div>
    )
}