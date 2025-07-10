import { createRootRoute, Link, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import "../App.css";
import {Toaster} from "react-hot-toast";

export const Route = createRootRoute({
    component: () => (
        <>
            <Toaster
                position={"bottom-left"}
            />
            { /*
            <div className="p-2 flex gap-2">
                <Link to="/" className="[&.active]:font-bold">
                    Home
                </Link>{' '}
                <Link to="/about" className="[&.active]:font-bold">
                    About
                </Link>
            </div>
            */ }
            <Outlet />
            <TanStackRouterDevtools />
        </>
    ),
})