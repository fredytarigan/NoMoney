import React, { Suspense } from "react";
import { BrowserRouter } from "react-router-dom";
import ThemeProvider from "./themes";
import { HelmetProvider } from 'react-helmet-async';

const AppProvider = ( { children } : { children: React.ReactNode}) => {
    return (
        <Suspense>
            <HelmetProvider>
            <BrowserRouter basename='/ui'>
                <ThemeProvider defaultTheme="dark" storageKey="nomoney-ui-theme">
                    {children}
                </ThemeProvider>
            </BrowserRouter>
            </HelmetProvider>
        </Suspense>
    )
};

export default AppProvider;