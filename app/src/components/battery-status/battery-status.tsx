export type BatteryStatusComponentProps = {
    batteryLevel: number;
}

export const BatteryStatus = (props: BatteryStatusComponentProps) => {
    return (
        <div className="flex items-center" style={{ transform: "scale(0.75)" }}>
            <span className="text-gray-300 text-xl">Akku:</span>
            <svg width="72" height="32" className="block">
                <rect x="0" y="8" width="60" height="16" rx="4" fill="#222" stroke="#444" strokeWidth="3"/>
                <rect x="60" y="13" width="7" height="6" rx="2" fill="#444"/>
                <rect
                    x="3"
                    y="11"
                    width={Math.max(0, 54 * (props.batteryLevel / 100))}
                    height="10"
                    rx="3"
                    fill="#22c55e"
                />
            </svg>
            <span className="font-bold text-white text-xl">{props.batteryLevel ? `${props.batteryLevel}%` : "Unbekannt"}</span>
        </div>
    );
}