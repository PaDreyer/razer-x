import { DpiSlider } from "../components/dpi-slider";
import { useCallback, useMemo, useState } from "react";
import { useDeviceManager } from "../components/device-manager";
import { Checkbox } from "../components/checkbox";




export const DpiSettings = () => {
    const deviceManager = useDeviceManager();
    const [useDpiStages, toggleDpiStages] = useState(false);
    const [individualXYStates, setIndividualXYStates] = useState<{ [key: string]: boolean }>({
        "1": false,
        "2": false,
        "3": false,
        "4": false,
        "5": false,
    });
    const [individualXY, setIndividualXY] = useState(false);

    const deviceInformation = deviceManager?.deviceInformation;

    // Determine currently active stage from device information
    const activeDpiStage = useMemo(() => {
        const activeStage = deviceInformation?.dpiStages.find(s => s.active === true);
        return activeStage ? activeStage.stage.toString() : "1";
    }, [deviceInformation?.dpiStages]);

    const handleDpiChange = useCallback((value: { x: number; y: number }) => {
        deviceManager?.setDpiXy(value.x, value.y);
    }, [deviceManager]);

    const handleStageDpiChange = useCallback((stageNum: number, value: { x: number; y: number }) => {
        if (!deviceInformation) return;

        const newStages = deviceInformation.dpiStages.map(s => {
            if (s.stage === stageNum) {
                return { ...s, dpiX: value.x, dpiY: value.y };
            }
            return s;
        });

        console.log('Dispatching new stages:', newStages);
        deviceManager?.setDpiStages(newStages);

        // If this is the active stage, also update the main DPI
        if (stageNum === parseInt(activeDpiStage)) {
            deviceManager?.setDpiXy(value.x, value.y);
        }
    }, [deviceInformation, deviceManager, activeDpiStage]);

    const handleStageClick = useCallback((stageNum: number) => {
        if (!deviceInformation) return;

        const newStages = deviceInformation.dpiStages.map(s => ({
            ...s,
            active: s.stage === stageNum
        }));

        deviceManager?.setDpiStages(newStages);

        // Also apply the DPI of the new stage to the mouse
        const selectedStage = newStages.find(s => s.stage === stageNum);
        if (selectedStage) {
            deviceManager?.setDpiXy(selectedStage.dpiX, selectedStage.dpiY);
        }
    }, [deviceInformation, deviceManager]);

    const toggleIndividualXYForStage = useCallback((stageId: string) => {
        setIndividualXYStates(prev => ({
            ...prev,
            [stageId]: !prev[stageId]
        }));
    }, []);

    if (!deviceInformation) {
        return null;
    }

    const { dpiXy, dpiStages } = deviceInformation;
    const initialDpiState = { x: dpiXy[0], y: dpiXy[1] };

    return (
        <div className="flex flex-col gap-4">
            {/* 
                Original Comment:
                <p className="text-sm text-gray-500">
                Adjust the DPI settings for your device. This will affect the sensitivity of your mouse or touchpad.
            </p> 
            */}

            <div className="flex flex-row items-center justify-between mb-2">
                <span className="text-gray-400 text-sm">Aktuelle DPI Stufe: {activeDpiStage}</span>
                <Checkbox
                    checked={useDpiStages}
                    onChange={() => toggleDpiStages(!useDpiStages)}
                    label={"DPI-Stufen aktivieren"}
                />
            </div>

            <div className="mx-2 my-2 flex flex-col gap-4">
                {!useDpiStages ? (
                    // Simplified View: Single Slider
                    <div className="flex flex-row items-center gap-4 my-2">
                        <div className="flex w-full rounded-lg shadow-md bg-gray-700 flex-row items-center justify-around">
                            <div className="flex w-10/12">
                                <DpiSlider
                                    id="single-dpi-slider"
                                    individualXY={individualXY}
                                    initialDpiState={initialDpiState}
                                    step={100}
                                    min={100}
                                    max={35000}
                                    debounceDelay={300}
                                    onChange={handleDpiChange}
                                />
                            </div>
                            <button
                                className="btn btn-secondary select-none mr-6"
                                onClick={() => setIndividualXY(!individualXY)}
                            >
                                {individualXY ? "X & Y" : "X / Y"}
                            </button>
                        </div>
                    </div>
                ) : (
                    // Advanced View: Multiple Stages
                    dpiStages.sort((a, b) => a.stage - b.stage).map((stage) => {
                        const stageId = stage.stage.toString();
                        const isActive = stageId === activeDpiStage;

                        return (
                            <div className="flex flex-row items-center gap-4 my-2" key={stageId}>
                                <div
                                    className={`flex items-center justify-center border bg-gray-700 text-gray-900 hover:text-white font-semibold shadow rounded-full w-12 h-12 flex-shrink-0 transition-all ${isActive ? "border-white text-white cursor-default" : "border-gray-700 hover:scale-110 cursor-pointer"}`}
                                    onClick={() => handleStageClick(stage.stage)}
                                >
                                    <div className="flex text-4xl font-bold select-none">
                                        {stageId}
                                    </div>
                                </div>
                                <div className="flex w-full rounded-lg shadow-md bg-gray-700 flex-row items-center justify-around">
                                    <div className="flex w-10/12">
                                        <DpiSlider
                                            id={`${stageId}-dpi-slider`}
                                            individualXY={individualXYStates[stageId]}
                                            initialDpiState={{ x: stage.dpiX, y: stage.dpiY }}
                                            step={100}
                                            min={100}
                                            max={35000}
                                            debounceDelay={300}
                                            onChange={(value) => handleStageDpiChange(stage.stage, value)}
                                        />
                                    </div>
                                    <button
                                        className="btn btn-secondary select-none mr-6"
                                        onClick={() => toggleIndividualXYForStage(stageId)}
                                    >
                                        {individualXYStates[stageId] ? "X & Y" : "X / Y"}
                                    </button>
                                </div>
                            </div>
                        );
                    })
                )}
            </div>
        </div>
    );
}