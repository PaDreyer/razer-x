import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import './UpdateOverlay.css';

const UpdateOverlay: React.FC = () => {
    const [progress, setProgress] = useState<number>(0);
    const [status, setStatus] = useState<string | null>(null);
    const [version, setVersion] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [isVisible, setIsVisible] = useState<boolean>(false);

    useEffect(() => {
        const setupListeners = async () => {
            const unlistenAvailable = await listen<string>('update-available', (event) => {
                setVersion(event.payload);
                setIsVisible(true);
                setStatus('Downloading update...');
            });

            const unlistenProgress = await listen<number>('update-progress', (event) => {
                setProgress(event.payload);
            });

            const unlistenStatus = await listen<string>('update-status', (event) => {
                if (event.payload === 'installing') {
                    setStatus('Installing update...');
                    setProgress(100);
                } else if (event.payload === 'finished') {
                    setStatus('Update complete!');
                    setTimeout(() => setIsVisible(false), 2000);
                }
            });

            const unlistenError = await listen<string>('update-error', (event) => {
                setError(event.payload);
                setStatus('Update failed');
                setTimeout(() => setIsVisible(false), 5000);
            });

            return { unlistenAvailable, unlistenProgress, unlistenStatus, unlistenError };
        };

        const listenersPromise = setupListeners();

        return () => {
            listenersPromise.then((listeners) => {
                listeners.unlistenAvailable();
                listeners.unlistenProgress();
                listeners.unlistenStatus();
                listeners.unlistenError();
            });
        };
    }, []);

    if (!isVisible) return null;

    return (
        <div className="update-overlay">
            <div className="update-card">
                <h2>Updating Razer X</h2>
                {version && <p className="version">Version {version}</p>}
                <div className="status-container">
                    <p className="status-text">{status}</p>
                    <div className="progress-bar-bg">
                        <div
                            className="progress-bar-fill"
                            style={{ width: `${progress}%` }}
                        />
                    </div>
                    <div className="progress-info">
                        <p className="progress-percentage">{Math.round(progress)}%</p>
                    </div>
                </div>
                {error && <p className="error-text">{error}</p>}
            </div>
        </div>
    );
};

export default UpdateOverlay;
