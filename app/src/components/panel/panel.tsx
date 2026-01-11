import {PropsWithChildren} from "react";

export type PanelComponentProps = {
    className?: string;
    title?: string;
}

export const Panel = (props: PropsWithChildren<PanelComponentProps>) => {
    const combinedClassName = `w-full h-full bg-gray-800 rounded-lg shadow-md py-6 px-6 gap-8 select-none ${props.className || ''}`;

    return (
        <div className={combinedClassName}>
            { props.children }
        </div>
    );
}

Panel.Header = (props: PropsWithChildren) => {
    return (
        <h2 className="text-lg font-semibold text-white mb-4 hover:cursor-default">
            { props.children }
        </h2>
    )
}

Panel.SubHeader = (props: PropsWithChildren) => {
    return (
        <h3 className="text-md font-semibold text-gray-400 mb-2 hover:cursor-default">
            { props.children }
        </h3>
    )
}

Panel.Body = (props: PropsWithChildren) => {
    return (
        <div className="flex flex-col gap-4">
            { props.children }
        </div>
    )
}