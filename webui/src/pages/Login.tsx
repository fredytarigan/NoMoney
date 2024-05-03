"use client";

import React, { useState } from 'react';
import { LockOutlined } from "@mui/icons-material";
import {
  Container,
  CssBaseline,
  Box,
  Avatar,
  Typography,
  TextField,
  Button,
  Grid,
} from "@mui/material";
import axios from 'axios';

const Login = () => {
    const [username, setUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [loginError, setLoginError] = useState<string>("");
    
    const handleLogin = () => {
        setLoginError("");

        if ( username === "" || password === "" ) {
            setLoginError("Please enter a valid username and password")
        }

        axios.post("http://127.0.0.1:8083/api/v1/auth/login", {
            username, password
        }).then((response) => {
            console.log(response.data)
        }).catch((error) => {
            console.log(error.message);
        })
    };
    
    return (
        <>
            <Container maxWidth="xs">
                <CssBaseline />
                <Box
                sx={{
                    mt: 20,
                    display: "flex",
                    flexDirection: "column",
                    alignItems: "center",
                }}
                >
                <Avatar sx={{ m: 1, bgcolor: "primary.light" }}>
                    <LockOutlined />
                </Avatar>
                <Typography variant="h5">Login</Typography>
                <Box sx={{ mt: 1 }}>
                    <TextField
                    margin="normal"
                    required
                    fullWidth
                    id="username"
                    label="Username"
                    name="username"
                    autoFocus
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    />

                    <TextField
                    margin="normal"
                    required
                    fullWidth
                    id="password"
                    name="password"
                    label="Password"
                    type="password"
                    value={password}
                    onChange={(e) => {
                        setPassword(e.target.value);
                    }}
                    />

                    <Button
                    fullWidth
                    variant="contained"
                    sx={{ mt: 3, mb: 2 }}
                    onClick={handleLogin}
                    >
                    Login
                    </Button>
                    <Grid container justifyContent={"flex-end"}>
                    </Grid>
                </Box>
                </Box>
            </Container>
        </>
    )
}

export default Login