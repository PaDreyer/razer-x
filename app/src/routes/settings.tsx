import { createFileRoute, Link } from '@tanstack/react-router'

export const Route = createFileRoute('/settings')({
    component: SettingsPage,
})

function SettingsPage() {
    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 p-8">
            <div className="max-w-4xl mx-auto relative">
                <div className="flex items-center gap-4 mb-8 mt-4">
                    <Link
                        to="/"
                        className="p-2 rounded-full hover:bg-gray-700 transition-colors text-white"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6" /></svg>
                    </Link>
                    <h2 className="text-4xl font-semibold text-white">Settings</h2>
                </div>

                <div className="flex flex-col items-center justify-center p-20 text-gray-500">
                    <p className="text-xl">Settings will be added soon.</p>
                </div>
            </div>
        </div>
    )
}
