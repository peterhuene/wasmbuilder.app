import ReactDOM from "react-dom/client";
import App from "./app";
import { printGraph } from "./graph";
import process from "process";

const app = ReactDOM.createRoot(document.getElementById("app"));
app.render(<App />);

if (!process.env.NODE_ENV || process.env.NODE_ENV === "development") {
  globalThis.printGraph = printGraph;
}
