import {SliderComponentHandle, SliderComponentProps, SliderExtended} from "../slider-extended";
import {memo, useCallback, useEffect, useRef, useState} from "react";

export type DpiSliderBaseComponentProps = {
    individualXY?: boolean; // Optional prop to toggle individual x and y DPI settings
    initialDpiState?: { x: number; y: number }; // Optional initial DPI state
    onChange?: (dpi: { x: number; y: number }) => void; // Callback for DPI changes
}

export type DpiSliderComponentProps = DpiSliderBaseComponentProps & Omit<SliderComponentProps, "onChange" | "initialValue">;

// Optionally set x and y via toggle in props, otherwise set x and y to the same value
// Also forward props to the SliderExtended component if needed
// Also forward ref to the SliderExtended component if needed
export const DpiSlider = memo(function DpiSlider(props: DpiSliderComponentProps) {
    const { individualXY, initialDpiState, onChange: propsOnChange, ...sliderProps } = props;
    const ySlider = useRef<SliderComponentHandle>(null);
    const [dpiState, setDpiState] = useState({
        x: props.initialDpiState?.x ?? 16000,
        y: props.initialDpiState?.y ?? 16000,
    });

    const onChange = useCallback((x: number, y: number) => {
        setDpiState({ x, y});
        propsOnChange?.({ x, y});
    }, [setDpiState, propsOnChange]);

    useEffect(() => {
        if (individualXY === false && dpiState?.x !== undefined) {
            if (initialDpiState?.y !== dpiState.x) {
                ySlider.current?.setValueExtern(dpiState.x, true)
            }
        }
    }, [individualXY, dpiState]);

    return (
        <div className="w-full flex flex-col gap-2 p-4 px-6 mb-2">

            <div className="w-full flex flex-row gap-4 items-center">
                <p className="select-none cursor-default">{ individualXY ? `DPI X`: `DPI X/Y`}</p>
                <SliderExtended
                    initialValue={dpiState?.x}
                    {...sliderProps}
                    onChange={value => {
                        onChange(value, props.individualXY ? dpiState.y : value);

                        // If individualXY is false, set the ySlider to the same value
                        // This allows for a single slider to control both x and y DPI
                        // Causes another render (ySlider.onChange is called -> onChange again with the same values)... but it works
                        // And is required to keep the ySlider in sync with the xSlider
                        if (!props.individualXY) {
                            ySlider.current?.setValueExtern(value, true);
                        }
                    }}
                />
            </div>

            <div className={`w-full flex flex-row gap-4 items-center ${props.individualXY ? '' : 'hidden'}`}>
                <p className="select-none cursor-default">DPI Y</p>
                <SliderExtended
                    ref={ySlider}
                    initialValue={dpiState?.y}
                    {...sliderProps}
                    onChange={value => onChange(dpiState.x, value)}
                />
            </div>
        </div>
    )
});