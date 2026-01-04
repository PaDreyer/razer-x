import { createFileRoute } from '@tanstack/react-router'
import { RgbColorPicker,  } from "react-colorful";
import toast from "react-hot-toast";
import {PossiblePollingRates, RGBColor, useDeviceManager} from "../components/device-manager";
import {useCallback, useRef} from "react";
import {SliderExtended, SliderComponentHandle} from "../components/slider-extended";
import {PopoverColorPicker} from "../components/popover-colorpicker";

export const Route = createFileRoute('/')({
    component: Index,
})

function Index() {
    const sliderRef = useRef<SliderComponentHandle>(null);
    const deviceManager = useDeviceManager();

    const setSliderValue = useCallback((number: number) => {
        sliderRef.current?.setValueExtern(number, true);
    }, [sliderRef.current])

    if (!deviceManager) {
        console.log("DeviceManager not available. Ensure you are using the DeviceManagerProvider.");
        return null;
    }

    const {
        isLoading,
        isInitialized,
        error,
        setPollingRate,
        setDpiXY,
        setBacklightBrightness,
        setBacklightColor,
        deviceInformation,
    } = deviceManager;

    if (!deviceInformation) {
        return null;
    }

    const {
        batteryLevel,
        pollingRate,
        dpiXY,
        backlightBrightness,
        backlightColor,
    } = deviceInformation!;

    console.log("Backlight brightness:", backlightBrightness);

    if (error.isError) {
        toast(`Fehler: ${error.message}`)
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8">
            <div className="max-w-4xl mx-auto relative">
                <h2 className="text-xl font-semibold text-white mb-2">Razer Basilisk V3 Pro</h2>
                <p className={`mb-2 text-${!isLoading ? "green" : "red"}-400`}>
                    {!isLoading ? "Verbunden" : "Lädt..."}
                </p>
                {(isInitialized && !error.isError) &&
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
                                    width={Math.max(0, 54 * (batteryLevel! / 100))}
                                    height="10"
                                    rx="3"
                                    fill="#22c55e"
                                />
                            </svg>
                            <span className="font-bold text-white text-xl">{batteryLevel ? `${batteryLevel}%` : "Unbekannt"}</span>
                        </div>

                        {/* Shortcuts für Energiesparmodus, Standard Modus und Max Performance */}
                        <div className="flex justify-center gap-4 mb-6">
                            <button
                                onClick={async () => {
                                    await setBacklightBrightness(10);
                                    await setPollingRate(125)
                                }}
                                className="px-4 py-1 rounded bg-blue-700 text-white hover:bg-blue-800 transition"
                            >
                                Energiesparmodus
                            </button>
                            <button
                                onClick={async () => {
                                    await setBacklightBrightness(25);
                                    await setPollingRate(500);
                                }}
                                className="px-4 py-1 rounded bg-gray-700 text-white hover:bg-gray-800 transition"
                            >
                                Standard
                            </button>
                            <button
                                onClick={async () => {
                                    await setBacklightBrightness(100);
                                    await setPollingRate(1000);
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
                                    <span className="text-2xl font-bold text-white">{dpiXY?.[0]}</span>
                                </div>
                                <div className="flex flex-col items-center">
                                    <span className="text-gray-400 text-sm">DPI Y</span>
                                    <span className="text-2xl font-bold text-white">{dpiXY?.[1]}</span>
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
                                            onClick={() => setPollingRate(rate as PossiblePollingRates)}
                                            className={`px-2 py-1 rounded text-sm transition ${
                                                pollingRate === rate
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
                                <SliderExtended
                                    ref={sliderRef}
                                    debounceDelay={500}
                                    min={0}
                                    max={100}
                                    step={1}
                                    onChange={value => setBacklightBrightness(value)}
                                    className="flex-1"
                                    initialValue={backlightBrightness}
                                />
                                {/* Shortcuts für Helligkeit */}
                                <div className="flex justify-center gap-3 mb-6">
                                    {[0, 5, 25, 75, 100].map(val => (
                                        <button
                                            key={val}
                                            onClick={() => setSliderValue(val)}
                                            className={`px-3 py-1 rounded ${backlightBrightness === val ? 'bg-green-600 text-white' : 'bg-gray-700 text-gray-200'} hover:bg-green-700 transition`}
                                        >
                                            {val}
                                        </button>
                                    ))}
                                </div>
                                <div className="flex items-center justify-between">
                                    <span className="text-gray-300">Farbe:</span>
                                    <span className="font-bold text-white">R/G/B:{backlightColor?.r}/{backlightColor?.g}/{backlightColor?.b}</span>
                                </div>
                            </div>
                            {/* Color Picker rechts */}
                            <div className="flex-1 flex flex-col items-center">
                                <p className="text-gray-300 mb-2">Farbe wählen:</p>
                                <PopoverColorPicker color={backlightColor} onChange={(color) => setBacklightColor(color as RGBColor)} />
                            </div>
                        </div>
                    </>
                }
            </div>
        </div>
    )
}



