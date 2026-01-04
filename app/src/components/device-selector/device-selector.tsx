export type DeviceSelectorDeviceInformation = {
    name: string;
    productId: string;
}

export type DeviceSelectorComponentProps = {
    devices: Array<DeviceSelectorDeviceInformation>;
}

export const DeviceSelector = (props: DeviceSelectorComponentProps) => {
    return (
        <div className="device-selector">
            <h2 className="text-xl font-semibold text-white mb-2">Device Selector</h2>
            <p>Select a device to manage its settings.</p>
            {/* Device selection logic will go here */}
            <select className="text-xl font-semibold text-white mb-2">
                {props.devices.map((device, index) => (
                    <option key={index} value={device.productId}>
                        {device.name} ({device.productId})
                    </option>
                ))}
            </select>
        </div>
    );
}