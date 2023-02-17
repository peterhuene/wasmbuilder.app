import ReactDOM from "react-dom/client";
import App from "./app";
import { graph } from "./graph";
import process from "process";
import "reactflow/dist/style.css";
import "./app.css";

const app = ReactDOM.createRoot(document.getElementById("app"));
app.render(<App />);

if (!process.env.NODE_ENV || process.env.NODE_ENV === "development") {
  globalThis.printGraph = graph.printGraph;
}
