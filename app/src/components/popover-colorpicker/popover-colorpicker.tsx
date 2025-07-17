import { useCallback, useRef, useState } from "react";
import { RgbColorPicker } from "react-colorful";
import "./popover-colorpicker.css";
import useClickOutside from "../../hooks/use-click-outside";


export type PopoverColorpickerComponentProps = {
    color: { r: number; g: number; b: number; };
    onChange: (color: { r: number; g: number; b: number; }) => void;
}

export const PopoverColorPicker = (props: PopoverColorpickerComponentProps) => {
    const popover = useRef<HTMLDivElement>(null);
    const [isOpen, toggle] = useState(false);

    const close = useCallback(() => toggle(false), []);
    useClickOutside(popover, close);

    return (
        <div className="picker">
            <div
                className="swatch"
                style={{ backgroundColor: `rgb(${props.color.r},${props.color.g},${props.color.b})` }}
                onClick={() => toggle(true)}
            />

            {isOpen && (
                <div className="popover" ref={popover}>
                    <RgbColorPicker color={props.color} onChange={props.onChange} />
                    <div className="popover-footer">
                        <button className="btn btn-secondary" onClick={close}>
                            Close
                        </button>
                        <button className="btn btn-primary" onClick={close}>
                            Apply
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};
