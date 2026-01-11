import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import './Splashscreen.css';

const Splashscreen: React.FC = () => {
    console.log('Splashscreen component mounted');
    const [loadingStatus, setLoadingStatus] = useState<string>('Starting up...');
    const [updateProgress, setUpdateProgress] = useState<number>(0);
    const [updateStatus, setUpdateStatus] = useState<string | null>(null);
    const [updateVersion, setUpdateVersion] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [showProgress, setShowProgress] = useState<boolean>(false);

    useEffect(() => {
        const setupListeners = async () => {
            console.log('Setting up splashscreen event listeners...');

            // Listen for loading status messages
            const unlistenLoading = await listen<string>('loading-status', (event) => {
                console.log('Received loading-status event:', event.payload);
                setLoadingStatus(event.payload);
            });

            // Listen for update events
            const unlistenAvailable = await listen<string>('update-available', (event) => {
                setUpdateVersion(event.payload);
                setShowProgress(true);
                setUpdateStatus('Downloading update...');
                setLoadingStatus('Downloading update...');
            });

            const unlistenProgress = await listen<number>('update-progress', (event) => {
                setUpdateProgress(event.payload);
            });

            const unlistenStatus = await listen<string>('update-status', (event) => {
                if (event.payload === 'installing') {
                    setUpdateStatus('Installing update...');
                    setLoadingStatus('Installing update...');
                    setUpdateProgress(100);
                } else if (event.payload === 'finished') {
                    setUpdateStatus('Update complete!');
                    setLoadingStatus('Update complete!');
                }
            });

            const unlistenError = await listen<string>('update-error', (event) => {
                setError(event.payload);
                setUpdateStatus('Update failed');
                setLoadingStatus('Continuing without update...');
                setShowProgress(false);
            });

            const unlistenComplete = await listen('initialization-complete', () => {
                console.log('Initialization complete, splashscreen will close');
            });

            return { unlistenLoading, unlistenAvailable, unlistenProgress, unlistenStatus, unlistenError, unlistenComplete };
        };

        const listenersPromise = setupListeners();

        return () => {
            listenersPromise.then((listeners) => {
                listeners.unlistenLoading();
                listeners.unlistenAvailable();
                listeners.unlistenProgress();
                listeners.unlistenStatus();
                listeners.unlistenError();
                listeners.unlistenComplete();
            });
        };
    }, []);

    return (
        <div className="splashscreen">
            <div className="splashscreen-content">
                <img src="/AppLogo.png" alt="RazerX Logo" className="logo" />

                <div className="status-section">
                    <p className="loading-text">{loadingStatus}</p>

                    {showProgress && (
                        <div className="update-section">
                            {updateVersion && <p className="version">Version {updateVersion}</p>}

                            <div className="progress-bar-bg">
                                <div
                                    className="progress-bar-fill"
                                    style={{ width: `${updateProgress}%` }}
                                />
                            </div>

                            <div className="progress-info">
                                <p className="progress-percentage">{Math.round(updateProgress)}%</p>
                            </div>

                            {updateStatus && <p className="update-status">{updateStatus}</p>}
                        </div>
                    )}

                    {error && <p className="error-text">{error}</p>}
                </div>
            </div>
        </div>
    );
};

export default Splashscreen;
