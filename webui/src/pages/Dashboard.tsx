import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom"

const Dashboard = () => {
    const navigate = useNavigate();
    const [token, setToken] = useState({});

    useEffect(() => {
        if ( localStorage.getItem('access_token') == "" || localStorage.getItem('access_token') == null ) {
            navigate("/login");
        } else {
            getToken();
        }
    }, []);

    const getToken = () => {
        console.log(token);
    }

    return (
        <h1>I am Dashboard</h1>
    )
}

export default Dashboard