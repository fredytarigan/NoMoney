import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@radix-ui/react-label";
import axios from "axios";
import { useState } from "react"
import { useNavigate } from "react-router-dom";

const Login = () => {
    const [username, setUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    // const [loginError, setLoginError] = useState<string>("");

    const navigate = useNavigate();

    const handleLogin = async () => {
        if (username && password) {
            axios
                .post(import.meta.env.VITE_API_URL + "/v1/auth/login", {username, password})
                .then((response) => {
                    let token = response.data.data.token;
                    localStorage.setItem('access_token', token);

                    navigate("/dashboard");

                })
                .catch((error) => {
                    console.log(error.message)
                })
        } else {
            console.log("Undefined username password");
        }
    }

    return (
        <div className="relative h-screen flex-col items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0">
            <div className="relative hidden h-full flex-col bg-muted p-10 text-white dark:border-r lg:flex">
                <div className="absolute inset-0 bg-primary dark:bg-secondary" />
                <div className="relative z-20 flex items-center text-lg font-medium">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    className="mr-2 h-6 w-6"
                >
                    <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />
                </svg>
                Logo
                </div>
                <div className="relative z-20 mt-auto">
                    <blockquote className="space-y-2">
                        <p className="text-lg">
                        &ldquo;This library has saved me countless hours of work and
                        helped me deliver stunning designs to my clients faster than ever
                        before.&rdquo;
                        </p>
                        <footer className="text-sm">Sofia Davis</footer>
                    </blockquote>
                </div>
            </div>
            <div className="flex h-full items-center p-4 lg:p-8">
                <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
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
                </div>
            </div>
        </div>
    )
}

export default Login