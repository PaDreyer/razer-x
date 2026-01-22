import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import './Splashscreen.css';

const Splashscreen: React.FC = () => {
    console.log('Splashscreen component mounted');
    const [loadingStatus, setLoadingStatus] = useState<string>('Starting up...');
    const [updateProgress, setUpdateProgress] = useState<number>(0);
    const [updateStatus, setUpdateStatus] = useState<string | null>(null);
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
            const unlistenAvailable = await listen<string>('update-available', () => {
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
            {/* Ambient Background Blobs - Mirroring main page for consistency */}
            <div className="absolute inset-0 overflow-hidden pointer-events-none">
                <div className="absolute top-0 -left-4 w-72 h-72 bg-blue-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob"></div>
                <div className="absolute top-0 -right-4 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 animate-blob animation-delay-2000"></div>
            </div>

            <div className="splash-overlay">
                <div className="splashscreen-content">
                    <img src="/AppLogo.png" alt="RazerX Logo" className="logo" />

                    <div className="status-section">
                        {!showProgress ? (
                            <p className="loading-text">{loadingStatus}</p>
                        ) : (
                            <div className="update-container">
                                <p className="updating-title">Updating...</p>
                                <div className="progress-bar-bg">
                                    <div
                                        className="progress-bar-fill"
                                        style={{ width: `${updateProgress}%` }}
                                    />
                                </div>
                                <p className="update-status">
                                    {updateStatus === 'Update complete!'
                                        ? 'Update complete! The app will restart now.'
                                        : updateStatus}
                                </p>
                            </div>
                        )}

                        {error && <p className="error-text">{error}</p>}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Splashscreen;
