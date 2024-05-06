import { Button } from "@/components/ui/button";
import axios from "axios";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom"

interface userProfile {
    user_id: String;
    username: String;
    email: String;
    first_name: String,
    last_name: String,
    avatar_path: String,
    role_name: String;
    family_name: String,
}

const Dashboard = () => {

    const navigate = useNavigate();
    const [user, setUser] = useState<userProfile>();

    useEffect(() => {
        if ( localStorage.getItem('access_token') == "" || localStorage.getItem('access_token') == null ) {
            navigate("/login");
        } else {
            getUser();
        }
    }, []);

    const getUser = () => {
        axios
            .get(import.meta.env.VITE_API_URL + '/v1/profiles', { headers: { Authorization: 'Bearer ' + localStorage.getItem('access_token')}})
            .then((response) => {
                setUser(response.data.data);
                console.log(response.data)
            })
            .catch((error) => {
                console.log(error);
            })
    };

    const handleLogout = () => {
        localStorage.setItem('access_token', '');
        navigate('/login');
    }

    return (
        <div className="text-white">
            <h1>I am Dashboard</h1>
            <h2>Welcome, {user?.username}</h2>
            <p>&nbsp;</p>
            <h2>Your User ID: {user?.user_id}</h2>
            <h2>Your Email: {user?.email}</h2>
            <h2>Your First Name: {user?.first_name}</h2>
            <h2>Your Last Name: {user?.last_name}</h2>
            <h2>Your Avatar Path: {user?.avatar_path}</h2>
            <h2>Your Role Name: {user?.role_name}</h2>
            <h2>Your Family Name: {user?.family_name}</h2>

            <Button
                className="w-full mt-5"
                onClick={handleLogout}
            >Logout</Button>
        </div>
    )
}

export default Dashboard