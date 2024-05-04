import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@radix-ui/react-label";
import axios from "axios";
import { useState } from "react"

const Login = () => {
    const [username, setUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    // const [loginError, setLoginError] = useState<string>("");

    const handleLogin = async () => {
        if (username && password) {
            axios
                .post(import.meta.env.VITE_API_URL + "/v1/auth/login", {username, password})
                .then((response) => {
                    console.log(response.data)
                })
                .catch((error) => {
                    console.log(error.message)
                })
        } else {
            console.log("Undefined username password");
        }
    }

    return (
        <>
            <Card className="w-full max-w-sm">
                <CardHeader>
                    <CardTitle className="text-2xl">Login</CardTitle>
                    <CardDescription>
                        Enter your username and password below to login into your account.
                    </CardDescription>
                </CardHeader>
                <CardContent className="grid gap4">
                    <div className="grid gap-2">
                        <Label htmlFor="username">Username</Label>
                        <Input 
                            id="username" 
                            name="username"
                            type="text" 
                            placeholder="username" 
                            required 
                            autoFocus
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                        />
                    </div>
                    <div className="grid gap-2">
                        <Label htmlFor="password">Password</Label>
                        <Input 
                            id="password" 
                            name="password"
                            type="password" 
                            required 
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </div>
                </CardContent>
                <CardFooter>
                    <Button 
                        className="w-full"
                        onClick={handleLogin}
                    >Login</Button>
                </CardFooter>
            </Card>
        </>
    )
}

export default Login