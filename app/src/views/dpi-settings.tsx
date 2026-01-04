import {DpiSlider} from "../components/dpi-slider";
import {useEffect, useMemo, useState} from "react";
import {Checkbox} from "../components/checkbox";


const defaultDpiState: Record<string, { x: number; y: number; }> = {
    "1": { x: 400, y: 400 },
    "2": { x: 800, y: 800 },
    "3": { x: 1600, y: 1600 },
    "4": { x: 3200, y: 3200 },
    "5": { x: 6400, y: 6400 },
}

export const DpiSettings = () => {
    const [ useDpiStages, toggleDpiStages ] = useState(false);
    const [ activeDpiStage, setActiveDpiStage ] = useState("3");
    const [ dpiStageValues, setDpiStageValues ] = useState<{ [key: string]: { x: number; y: number; } }>({
        "1": defaultDpiState["1"],
        "2": defaultDpiState["2"],
        "3": defaultDpiState["3"],
        "4": defaultDpiState["4"],
        "5": defaultDpiState["5"],
    });
    const dpiStages = useMemo(() => useDpiStages ? ["1", "2", "3", "4", "5"] : [activeDpiStage], [useDpiStages, activeDpiStage]);
    const [ individualXYStates, setIndividualXYStates ] = useState<{ [key: string]: boolean }>({
        "1": false,
        "2": false,
        "3": false,
        "4": false,
        "5": false,
    });

    useEffect(() => {
        console.log("DpiStages: ", dpiStageValues)
    }, [dpiStageValues]);

    return (
        <div className="flex flex-col gap-4">
            { /*
                <p className="text-sm text-gray-500">
                Adjust the DPI settings for your device. This will affect the sensitivity of your mouse or touchpad.
            </p>
            */ }
            <span className="text-gray-400 text-sm mb-2">Aktuelle DPI Stufe: { activeDpiStage }</span>
            <Checkbox
                checked={useDpiStages}
                onChange={() => toggleDpiStages(!useDpiStages)}
                label={"DPI-Stufen aktivieren"}
            />
            <div className="mx-2 my-4 flex flex-col gap-4">
                {
                    dpiStages.map((stage, index) =>
                        <div key={index}>
                            <div className="flex flex-row items-center gap-4 my-2" key={index}>
                                <div className={`flex items-center justify-center border bg-gray-700 text-gray-900 hover:text-white font-semibold shadow rounded-full w-12 h-12 ${stage === activeDpiStage ? "border-white text-white cursor-default" : "border-gray-700 hover:scale-110 cursor-pointer"}`}
                                    onClick={() => setActiveDpiStage(stage)}
                                >
                                    <div className="flex text-4xl font-bold select-none">
                                        {stage}
                                    </div>
                                </div>
                                <div className="flex w-full rounded-lg shadow-md bg-gray-700 flex-row items-center justify-around">
                                    <div className="flex w-10/12">
                                        <DpiSlider
                                            id={`${stage}-dpi-slider`}
                                            key={index}
                                            individualXY={individualXYStates[stage]}
                                            initialDpiState={dpiStageValues[stage]}
                                            step={100}
                                            min={100}
                                            max={35000}
                                            onChange={(value) => setDpiStageValues(prevState => ({
                                                ...prevState,
                                                    [stage]: {
                                                        x: individualXYStates[stage] ? value.x : value.y,
                                                        y: individualXYStates[stage] ? value.y : value.x
                                                    }
                                            }))}
                                        />
                                    </div>
                                    <button
                                        className="btn btn-secondary select-none"
                                        onClick={() => setIndividualXYStates({
                                            ...individualXYStates,
                                            [stage]: !individualXYStates[stage]
                                        })}
                                    >
                                        {individualXYStates[stage] ? "X & Y" : "X / Y"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    )
                }
            </div>
        </div>
    );
}