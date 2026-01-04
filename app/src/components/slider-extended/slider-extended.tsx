import Slider from "rc-slider";
import "rc-slider/assets/index.css";
import {
    useState,
    memo,
    useCallback,
    useImperativeHandle,
    forwardRef,
} from "react";
import { debounce } from "../../utils/debounce.ts";

export type SliderComponentProps = {
    initialValue?: number;
    debounceDelay?: number;
    onChange?: (value: number) => void;
    min?: number;
    max?: number;
    step?: number;
    className?: string;
};

export type SliderComponentHandle = {
    setValueExtern: (value: number, skipDebounceIfExists: boolean) => void;
};

export const SliderExtended = memo(
    forwardRef<SliderComponentHandle, SliderComponentProps>(function SliderExtended(
        props,
        ref
    ) {
        const [value, setValue] = useState(props.initialValue ?? 0);

        const onChange = useCallback(
            props.debounceDelay
                ? debounce((val: number) => {
                    props.onChange?.(val);
                }, props.debounceDelay)
                : props.onChange || (() => {}),
            [props.debounceDelay, props.onChange]
        );

        // Methode nach außen geben
        useImperativeHandle(ref, () => ({
            setValueExtern: (val: number, skipDebounceIfExists: boolean) => {
                setValue(val);
                if (skipDebounceIfExists) {
                    props.onChange?.(val);
                } else {
                    onChange(val);
                }
            },
        }));

        return (
            <div className="flex items-center gap-4 mb-6">
                <Slider
                    min={props.min}
                    max={props.max}
                    step={props.step}
                    value={value}
                    onChange={(val) => {
                        setValue(val as number);
                        onChange(val as number);
                    }}
                    className="flex-1"
                />
                <input
                    type="number"
                    min={props.min}
                    max={props.max}
                    value={value}
                    onChange={(e) => {
                        let val = Number(e.target.value);
                        if (val > 100) val = 100;
                        if (val < 0) val = 0;
                        setValue(val);
                        onChange(val);
                    }}
                    className="w-24 px-2 py-1 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none"
                />
            </div>
        );
    })
);
