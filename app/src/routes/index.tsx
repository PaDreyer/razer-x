import {createFileRoute } from '@tanstack/react-router'
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {RgbColorPicker} from "react-colorful";
import Slider from "rc-slider";
import 'rc-slider/assets/index.css';

export const Route = createFileRoute('/')({
    component: Index,
})

type DeviceInformation = {
    battery_status: number;
    dpi_x: number;
    dpi_y: number;
    polling_rate: number;
}

function Index() {
    const [currentColor, setCurrentColor] = useState({ r: 0, g: 0, b: 0 });
    const [newColor, setNewColor] = useState({ r: 0, g: 0, b: 0 });
    const [shouldUpdate, setShouldUpdate] = useState(true);
    const [deviceInformation, setDeviceInformation] = useState<DeviceInformation | null>(null);
    const [brightness, setBrightness] = useState(0);
    const [pollingRate, setPollingRate] = useState(0);

    useEffect(() => {
        //const timer = setInterval(() => {
            if (!shouldUpdate) return;

            invoke<string>('get_device_information')
                .then(data => setDeviceInformation(JSON.parse(data)));
        //}, 1500);

        //return () => clearInterval(timer);
    }, [shouldUpdate]);

    useEffect(() => {
        if (!deviceInformation) return;

        setPollingRate(deviceInformation.polling_rate);
    }, [deviceInformation]);

    useEffect((prev) => {
        const timeout = setTimeout(() => setDeviceBacklightBrightness(brightness), 500);

        return () => clearTimeout(timeout);
    }, [brightness]);

    useEffect((prev) => {
        if (prev !== 0 && pollingRate !== deviceInformation?.polling_rate) {
            setDevicePollingRate(pollingRate);
        }
    }, [pollingRate, deviceInformation])

    const setDeviceMatrixBacklightStatic = async (r: number, g: number, b: number) => {
        await invoke('set_device_matrix_backlight_static', { r, g, b });
    }

    const setDeviceBacklightBrightness = async (brightness: number) => {
        await invoke('set_device_backlight_brightness', { brightness });
    }

    const setDevicePollingRate = async (pollingRate: number) => {
        await invoke('set_device_polling_rate', { polling_rate: pollingRate });
    }

    const getDeviceLedRgbColor = async () => {
        const color = await invoke<{ rgb: number }>('get_device_led_rgb');
        console.log('Current LED Color:', color.rgb);
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8">
            <button onClick={() => getDeviceLedRgbColor()} className="mb-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition">get current LED color</button>
            {/*
            <button onClick={() => setDeviceMatrixBacklightStatic(255, 255, 255)}>set backlight white</button>
            <button onClick={() => setShouldUpdate(!shouldUpdate)} className="mb-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition">toggle update</button>
            */ }
            <div className="max-w-4xl mx-auto relative">
                <h2 className="text-xl font-semibold text-white mb-2">Razer Basilisk V3 Pro</h2>
                <p className={`mb-2 text-${!!deviceInformation ? "green" : "red"}-400`}>
                    {!!deviceInformation ? "Verbunden" : "Lädt..."}
                </p>
                {!!deviceInformation &&
                    <>
                        {/* Akkuanzeige oben rechts */}
                        <div className="absolute top-0 right-0 mt-4 mr-4 flex items-center space-x-3" style={{ transform: "scale(0.75)" }}>
                            <span className="text-gray-300 text-xl">Akku:</span>
                            <svg width="72" height="32" className="block">
                                <rect x="0" y="8" width="60" height="16" rx="4" fill="#222" stroke="#444" strokeWidth="3"/>
                                <rect x="60" y="13" width="7" height="6" rx="2" fill="#444"/>
                                <rect
                                    x="3"
                                    y="11"
                                    width={Math.max(0, 54 * (deviceInformation.battery_status / 100))}
                                    height="10"
                                    rx="3"
                                    fill="#22c55e"
                                />
                            </svg>
                            <span className="font-bold text-white text-xl">{deviceInformation.battery_status !== null ? `${deviceInformation.battery_status}%` : "Unbekannt"}</span>
                        </div>

                        {/* Shortcuts für Energiesparmodus, Standard Modus und Max Performance */}
                        <div className="flex justify-center gap-4 mb-6">
                            <button
                                onClick={() => {
                                    setBrightness(10);
                                    setPollingRate(125);
                                }}
                                className="px-4 py-1 rounded bg-blue-700 text-white hover:bg-blue-800 transition"
                            >
                                Energiesparmodus
                            </button>
                            <button
                                onClick={() => {
                                    setBrightness(50);
                                    setPollingRate(500);
                                }}
                                className="px-4 py-1 rounded bg-gray-700 text-white hover:bg-gray-800 transition"
                            >
                                Standard
                            </button>
                            <button
                                onClick={() => {
                                    setBrightness(100);
                                    setPollingRate(1000);
                                }}
                                className="px-4 py-1 rounded bg-pink-700 text-white hover:bg-pink-800 transition"
                            >
                                Max Performance
                            </button>
                        </div>

                        {/* Info Panel für DPI und Pollingrate */}
                        <div className="w-full bg-gray-800 rounded-lg shadow-md py-6 px-4 flex flex-row items-center justify-between gap-8 mt-8 mb-6">
                            {/* DPI Gruppe links */}
                            <div className="flex flex-row gap-8">
                                <div className="flex flex-col items-center">
                                    <span className="text-gray-400 text-sm">DPI X</span>
                                    <span className="text-2xl font-bold text-white">{deviceInformation.dpi_x}</span>
                                </div>
                                <div className="flex flex-col items-center">
                                    <span className="text-gray-400 text-sm">DPI Y</span>
                                    <span className="text-2xl font-bold text-white">{deviceInformation.dpi_y}</span>
                                </div>
                            </div>
                            {/* Polling Rate rechts */}
                            <div className="flex flex-col items-center">
                                <span className="text-gray-400 text-sm">Polling Rate</span>
                                <span className="text-2xl font-bold text-white">{pollingRate} Hz</span>
                                <div className="flex gap-2 mt-2">
                                    {[125, 500, 1000].map(rate => (
                                        <button
                                            key={rate}
                                            onClick={() => setPollingRate(rate)}
                                            className={`px-2 py-1 rounded text-sm transition ${
                                                deviceInformation.polling_rate === rate
                                                    ? 'bg-red-500 text-white ring-2 ring-orange-400'
                                                    : 'bg-gray-700 text-gray-200 hover:bg-orange-600'
                                            }`}
                                        >
                                            {rate}
                                        </button>
                                    ))}
                                </div>
                            </div>
                        </div>

                        {/* RGB Settings Panel */}
                        <div className="w-full flex flex-col md:flex-row gap-6 mt-4">
                            {/* Aktuelle Einstellungen links */}
                            <div className="flex-1 flex flex-col justify-center bg-gray-800 rounded-lg p-6">
                                <h3 className="text-lg font-semibold text-white mb-4">RGB Einstellungen</h3>
                                <div className="flex items-center justify-between mb-4">
                                    <span className="text-gray-300">Helligkeit:</span>
                                </div>
                                <div className="flex items-center gap-4 mb-6">
                                    <Slider
                                        min={0}
                                        max={100}
                                        step={1}
                                        value={brightness}
                                        onChange={value => setBrightness(value as number)}
                                        className="flex-1"
                                    />
                                    <input
                                        type="number"
                                        min={0}
                                        max={100}
                                        value={brightness}
                                        onChange={e => {
                                            let val = Number(e.target.value);
                                            if (val > 100) val = 100;
                                            if (val < 0) val = 0;
                                            setBrightness(val);
                                        }}
                                        className="w-24 px-2 py-1 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none"
                                    />
                                </div>
                                {/* Shortcuts für Helligkeit */}
                                <div className="flex justify-center gap-3 mb-6">
                                    {[0, 5, 25, 75, 100].map(val => (
                                        <button
                                            key={val}
                                            onClick={() => setBrightness(val)}
                                            className={`px-3 py-1 rounded ${brightness === val ? 'bg-green-600 text-white' : 'bg-gray-700 text-gray-200'} hover:bg-green-700 transition`}
                                        >
                                            {val}
                                        </button>
                                    ))}
                                </div>
                                <div className="flex items-center justify-between">
                                    <span className="text-gray-300">Farbe:</span>
                                    <span className="font-bold text-white">R/G/B:{currentColor.r}/{currentColor.g}/{currentColor.b}</span>
                                </div>
                            </div>
                            {/* Color Picker rechts */}
                            <div className="flex-1 flex flex-col items-center">
                                <p className="text-gray-300 mb-2">Farbe wählen:</p>
                                <RgbColorPicker color={newColor ?? currentColor} onChange={setNewColor} />
                                <button
                                    onClick={() => {
                                        setCurrentColor(newColor);
                                        setDeviceMatrixBacklightStatic(newColor.r, newColor.g, newColor.b);
                                    }}
                                    className="mt-4 px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
                                >Übernehmen</button>
                            </div>
                        </div>
                    </>
                }
            </div>
        </div>
    )
}



