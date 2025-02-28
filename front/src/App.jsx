import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Login from "./components/Login";
import Home from "./components/HomePage";
import AddExpenses from './components/AddExpenses';

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Login />} />
        <Route path="/home" element={<Home />} />
        <Route path="/add-expenses" element={<AddExpenses />} />
      </Routes>
    </Router>
  );
}