import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./App.scss";
import { StarknetModule } from 'starknet-signature';

let starknet = new StarknetModule();

const root = ReactDOM.createRoot(document.getElementById("app"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

export {starknet}