import ReactDOM from "react-dom/client";
import App from "./app";
import "reactflow/dist/style.css";
import "./app.css";

const app = ReactDOM.createRoot(document.getElementById("app"));
app.render(<App />);
