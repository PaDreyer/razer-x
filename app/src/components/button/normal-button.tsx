import { memo } from "react";

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
            className={`flex flex-grow px-4 py-2 rounded-xl text-sm transition-all duration-300 font-bold select-none items-center justify-center border ${props.disabled
                ? 'border-white/10 text-white/30 cursor-default bg-white/5 shadow-none'
                : `border-white/10 bg-white/5 text-gray-400 hover:text-white cursor-pointer hover:bg-white/10 hover:border-white/20 active:scale-95 shadow-lg ${props.active ? 'border-blue-400 text-white bg-blue-500/25 shadow-[0_2px_8px_rgba(59,130,246,0.6)] ring-1 ring-white/30' : ''}`
                }`}
        >
            <div className="flex">
                {props.text}
            </div>
        </div>
    )
})
