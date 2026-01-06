export type BatteryStatusComponentProps = {
    batteryLevel: number;
}

export const BatteryStatus = (props: BatteryStatusComponentProps) => {
    return (
        <div className="flex items-center gap-2" style={{ transform: "scale(0.75)" }}>
            <svg width="72" height="36" className="block flex-shrink-0">
                <rect x="0" y="7" width="60" height="22" rx="4" fill="#222" stroke="#444" strokeWidth="3" />
                <rect x="60" y="14" width="7" height="8" rx="2" fill="#444" />
                <rect
                    x="3"
                    y="10"
                    width={Math.max(0, 54 * (props.batteryLevel / 100))}
                    height="16"
                    rx="3"
                    fill="#22c55e"
                />
            </svg>
            <span className="font-bold text-white text-xl leading-none select-none">
                {props.batteryLevel ? `${props.batteryLevel}%` : "Unbekannt"}
            </span>
        </div>
    );
}