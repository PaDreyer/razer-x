import { PropsWithChildren } from "react";

export type PanelComponentProps = {
    className?: string;
    title?: string;
}

export const Panel = (props: PropsWithChildren<PanelComponentProps>) => {
    const combinedClassName = `w-full h-full bg-white/[0.03] backdrop-blur-2xl border border-white/10 rounded-3xl shadow-[0_8px_32px_0_rgba(0,0,0,0.37)] py-6 px-6 gap-2 select-none transition-all duration-500 hover:bg-white/[0.06] hover:border-white/20 hover:shadow-blue-500/10 ${props.className || ''}`;

    return (
        <div className={combinedClassName}>
            {props.children}
        </div>
    );
}

Panel.Header = (props: PropsWithChildren<{ className?: string }>) => {
    return (
        <div className={props.className}>
            {props.children}
        </div>
    )
}

Panel.SubHeader = (props: PropsWithChildren) => {
    return (
        <h3 className="text-md font-semibold text-gray-400 mb-2 hover:cursor-default">
            {props.children}
        </h3>
    )
}

Panel.Body = (props: PropsWithChildren<{ className?: string }>) => {
    return (
        <div className={`flex flex-col gap-4 ${props.className || ''}`}>
            {props.children}
        </div>
    )
}