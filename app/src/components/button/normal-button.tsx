import {memo} from "react";

export type NormalButtonComponentProps = {
    active?: boolean;
    disabled?: boolean;
    onClick?: () => void;
    text?: string;
}

export const NormalButton = memo(function NormalButton(props: NormalButtonComponentProps) {
    return (
        <div
            onClick={() => props.onClick?.()}
            className={`flex flex-grow px-2 py-1 rounded text-sm transition font-semibold select-none items-center justify-center ${
                props.disabled
                    ? 'border border-white text-white cursor-default'
                    : `border  border-gray-700 bg-gray-700 text-gray-400 hover:text-white cursor-pointer hover:scale-103 ${props.active ? 'border-white text-white bg-gray-800' : ''}`
            }`}
        >
            <div className="flex">
                { props.text }
            </div>
        </div>
    )
})
