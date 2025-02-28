import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const handleLogin = (e) => {
    e.preventDefault();
    if (email === "a@a" && password === "a") {
      navigate("/home");
    } else {
      setError("Credenciais inválidas.");
    }
  };

  return (
    <div 
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
        width: "100vw",
        margin: 0,
        padding: 0,
        position: "fixed",
        top: 0,
        left: 0,
        backgroundColor: "#f3f4f6" // Equivalente a bg-gray-100
      }}
    >
      <div 
        style={{
          backgroundColor: "white",
          padding: "1.5rem",
          borderRadius: "0.5rem",
          boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
          width: "100%",
          maxWidth: "28rem" // Equivalente a max-w-md
        }}
      >
        <h2 
          style={{
            fontSize: "1.5rem",
            fontWeight: "600",
            textAlign: "center",
            marginBottom: "1.5rem",
            color: "#000000" // Preto
          }}
        >
          Login
        </h2>
        
        {error && (
          <p 
            style={{
              color: "#000000", // Mudado de vermelho para preto
              fontSize: "0.875rem",
              textAlign: "center",
              marginBottom: "1rem"
            }}
          >
            {error}
          </p>
        )}
        
        <form 
          onSubmit={handleLogin} 
          style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
        >
          <div style={{ display: "flex", flexDirection: "column" }}>
            <label 
              htmlFor="email" 
              style={{ 
                fontSize: "0.875rem", 
                fontWeight: "500", 
                color: "#000000" // Preto
              }}
            >
              E-mail:
            </label>
            <input
              type="email"
              id="email"
              style={{
                width: "100%",
                padding: "0.5rem",
                border: "1px solid #d1d5db",
                borderRadius: "0.375rem",
                outline: "none",
                transition: "all 0.2s",
              }}
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>

          <div style={{ display: "flex", flexDirection: "column" }}>
            <label 
              htmlFor="password" 
              style={{ 
                fontSize: "0.875rem", 
                fontWeight: "500", 
                color: "#000000" // Preto
              }}
            >
              Senha:
            </label>
            <input
              type="password"
              id="password"
              style={{
                width: "100%",
                padding: "0.5rem",
                border: "1px solid #d1d5db",
                borderRadius: "0.375rem",
                outline: "none",
                transition: "all 0.2s",
              }}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>

          <button
            type="submit"
            style={{
              width: "100%",
              marginTop: "1rem",
              backgroundColor: "#3b82f6", // Equivalente a bg-blue-500
              color: "white",
              padding: "0.5rem",
              borderRadius: "0.375rem",
              border: "none",
              cursor: "pointer",
              transition: "background-color 0.2s",
            }}
            onMouseOver={(e) => (e.target.style.backgroundColor = "#2563eb")} // hover:bg-blue-600
            onMouseOut={(e) => (e.target.style.backgroundColor = "#3b82f6")}
          >
            Entrar
          </button>
        </form>
      </div>
    </div>
  );
}