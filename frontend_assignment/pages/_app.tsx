import "../styles/globals.css";
import type { AppProps } from "next/app";
import Form from "../components/form";
import { ethers } from "hardhat";
import Index from "../pages/index";

function MyApp({ Component, pageProps }: AppProps) {
  return <Form />;
  // return <Index />;
}

export default MyApp;
