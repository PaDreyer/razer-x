export type BatteryStatusComponentProps = {
    batteryLevel: number;
    isCharging: boolean;
}

export const BatteryStatus = (props: BatteryStatusComponentProps) => {
    return (
        <div className="flex items-center" style={{ transform: "scale(0.8)" }}>
            <div className="w-8 flex items-center justify-center">
                {props.isCharging && (
                    <svg
                        viewBox="0 0 24 24"
                        fill="white"
                        className="w-6 h-6 drop-shadow-[0_0_2px_rgba(0,0,0,0.8)]"
                    >
                        <path d="M11 20V13H7L13 4V11H17L11 20Z" />
                    </svg>
                )}
            </div>
            <div className="flex items-center justify-center border-2 border-white rounded-md px-4 py-0.5 bg-gray-800/40 backdrop-blur-md min-w-[4.2rem] h-[24px] shadow-lg">
                <span className="font-bold text-white text-base tracking-tight select-none">
                    {props.batteryLevel ? `${props.batteryLevel}%` : "--%"}
                </span>
            </div>
        </div>
    );
}