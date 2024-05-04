import Dashboard from "@/pages/Dashboard";
import { useRoutes } from "react-router-dom";

const PrivateRoutes = () => {
    const privateRoutes = [
        {
            path: '/',
            element: (
                <Dashboard />
            )
        }
    ]

    const routes = useRoutes([...privateRoutes]);

    return routes;
}

export default PrivateRoutes