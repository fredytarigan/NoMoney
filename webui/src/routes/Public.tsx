import Login from "@/pages/Login"
import { useRoutes } from "react-router-dom";

const PublicRoutes = () => {
    const publicRoutes = [
        {
            path: '/login',
            element: (
                <Login />
            )
        }
    ]

    const routes = useRoutes([...publicRoutes]);

    return routes;
}

export default PublicRoutes