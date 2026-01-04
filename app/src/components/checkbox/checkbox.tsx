import { useRef } from "react";


export type CheckboxComponentProps = {
    label?: string;
    checked?: boolean;
    onChange?: (checked: boolean) => void;
}

export const Checkbox = (props: CheckboxComponentProps) => {
    const inputId = useRef(crypto.randomUUID());

    return (
        <div className="flex flex-row leading-4 p-1">
            <input
                id={inputId.current}
                type="checkbox"
                checked={props.checked}
                onChange={(e) => {
                    props.onChange?.(e.target.checked);
                }}
                className="form-checkbox h-5 w-5 text-blue-600 cursor-pointer"
            />
            { props.label &&
                <label htmlFor={inputId.current} className="flex items-center select-none cursor-pointer">
                    <span className="ml-2 text-white">{ props.label }</span>
                </label>
            }
        </div>
    );
}