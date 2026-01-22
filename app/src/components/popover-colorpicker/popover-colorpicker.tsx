import { useCallback, useEffect, useRef, useState } from "react";
import { RgbColorPicker } from "react-colorful";
import "./popover-colorpicker.css";
import useClickOutside from "../../hooks/use-click-outside";


function convertRgbToString(color: { r: number; g: number; b: number; }) {
    return `rgb(${color.r}, ${color.g}, ${color.b})`;
}


export type PopoverColorpickerComponentProps = {
    color: { r: number; g: number; b: number; };
    onChange: (color: { r: number; g: number; b: number; }) => void;
    presetColors?: Array<{ r: number; g: number; b: number; }>;
    className?: string;
}

export const PopoverColorPicker = (props: PopoverColorpickerComponentProps) => {
    const popover = useRef<HTMLDivElement>(null);
    const [isOpen, toggle] = useState(false);
    const [selectedColor, setSelectedColor] = useState(props.color);

    useEffect(() => {
        setSelectedColor(props.color);
    }, [props.color])

    const close = useCallback(() => {
        console.log("CLOSE")
        toggle(false)
        setSelectedColor(props.color)
    }, [props.color]);

    useClickOutside(popover, close);

    const computedClassName = props.className ? `flex-1 flex-row ${props.className}` : "flex-1 flex-row";

    return (
        <div className={computedClassName}>
            <div className="picker">
                <div className="relative">
                    <div
                        className="swatch"
                        style={{ backgroundColor: convertRgbToString(props.color) }}
                        onClick={() => toggle(true)}
                    />

                    {isOpen && (
                        <div className="popover" ref={popover}>
                            <RgbColorPicker color={selectedColor} onChange={setSelectedColor} />

                            <div className="popover-footer">
                                <button className="btn btn-secondary" onClick={close}>
                                    Close
                                </button>
                                <button className="btn btn-primary" onClick={() => props.onChange(selectedColor)}>
                                    Apply
                                </button>
                            </div>
                        </div>
                    )}
                </div>

                {props.presetColors &&
                    <div className="picker__swatches">
                        {props.presetColors.map((presetColor) => (
                            <button
                                key={convertRgbToString(presetColor)}
                                className="picker__swatch"
                                style={{ background: convertRgbToString(presetColor) }}
                                onClick={() => props.onChange(presetColor)}
                            />
                        ))}
                    </div>
                }
            </div>
        </div>
    );
};
