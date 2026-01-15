import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import './UpdateOverlay.css';

const UpdateOverlay: React.FC = () => {
    const [progress, setProgress] = useState<number>(0);
    const [status, setStatus] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [isVisible, setIsVisible] = useState<boolean>(false);

    useEffect(() => {
        const setupListeners = async () => {
            const unlistenAvailable = await listen<string>('update-available', () => {
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
                <img src="/AppLogo.png" alt="RazerX Logo" className="overlay-logo" />
                <div className="overlay-content">
                    <p className="updating-title">Updating Razer X</p>
                    <div className="progress-bar-bg">
                        <div
                            className="progress-bar-fill"
                            style={{ width: `${progress}%` }}
                        />
                    </div>
                    <p className="update-status">
                        {status === 'Update complete!'
                            ? 'Update complete! The app will restart now.'
                            : status}
                    </p>
                </div>
                {error && <p className="error-text">{error}</p>}
            </div>
        </div>
    );
};

export default UpdateOverlay;
