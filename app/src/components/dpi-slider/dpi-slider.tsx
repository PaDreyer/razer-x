import { SliderComponentHandle, SliderComponentProps, SliderExtended } from "../slider-extended";
import { memo, useCallback, useEffect, useRef, useState } from "react";

export type DpiSliderBaseComponentProps = {
    individualXY?: boolean; // Optional prop to toggle individual x and y DPI settings
    initialDpiState?: { x: number; y: number }; // Optional initial DPI state
    onChange?: (dpi: { x: number; y: number }) => void; // Callback for DPI changes
}

export type DpiSliderComponentProps = DpiSliderBaseComponentProps & Omit<SliderComponentProps, "onChange" | "initialValue">;

/**
 * Optionally set x and y via toggle in props, otherwise set x and y to the same value
 * Also forward props to the SliderExtended component if needed
 */
export const DpiSlider = memo(function DpiSlider(props: DpiSliderComponentProps) {
    const { individualXY, initialDpiState, onChange: propsOnChange, ...sliderProps } = props;
    const ySlider = useRef<SliderComponentHandle>(null);
    const xSlider = useRef<SliderComponentHandle>(null);
    const [dpiState, setDpiState] = useState({
        x: props.initialDpiState?.x ?? 1600,
        y: props.initialDpiState?.y ?? 1600,
    });

    // Sync internal state with props (e.g. when hardware settings are loaded or changed elsewhere)
    useEffect(() => {
        if (props.initialDpiState) {
            setDpiState(props.initialDpiState);
            xSlider.current?.setValueOnly(props.initialDpiState.x);
            ySlider.current?.setValueOnly(props.initialDpiState.y);
        }
    }, [props.initialDpiState?.x, props.initialDpiState?.y]);

    const handleXChange = useCallback((x: number) => {
        setDpiState(prev => {
            const nextY = individualXY ? prev.y : x;
            if (!individualXY) {
                // Update Y slider UI only, without triggering hardware onChange
                ySlider.current?.setValueOnly(x);
            }
            propsOnChange?.({ x, y: nextY });
            return { x, y: nextY };
        });
    }, [individualXY, propsOnChange]);

    const handleYChange = useCallback((y: number) => {
        setDpiState(prev => {
            propsOnChange?.({ x: prev.x, y });
            return { ...prev, y };
        });
    }, [propsOnChange]);

    return (
        <div className="w-full flex flex-col gap-2 p-4 px-6 mb-2">
            <div className="w-full flex flex-row gap-4 items-center">
                <p className="select-none cursor-default w-20">{individualXY ? `DPI X` : `DPI X/Y`}</p>
                <SliderExtended
                    ref={xSlider}
                    initialValue={dpiState.x}
                    {...sliderProps}
                    onChange={handleXChange}
                />
            </div>

            <div className={`w-full flex flex-row gap-4 items-center ${individualXY ? '' : 'hidden'}`}>
                <p className="select-none cursor-default w-20">DPI Y</p>
                <SliderExtended
                    ref={ySlider}
                    initialValue={dpiState.y}
                    {...sliderProps}
                    onChange={handleYChange}
                />
            </div>
        </div>
    )
});