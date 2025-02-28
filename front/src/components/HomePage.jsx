import React, { useState, useEffect } from "react";
import { Pencil } from "lucide-react";
import { useNavigate } from "react-router-dom";
import "./HomePage.css"; // Importing CSS

// Wallet connection (Metamask, etc.)
import { ConnectButton } from '@rainbow-me/rainbowkit';

// Import the contract interaction function
import { addParticipant, getBalance, isParticipant } from "../App"; // Adjust the import path as needed

const HomePage = () => {
  const [groupName, setGroupName] = useState("");
  const [isEditing, setIsEditing] = useState(false);
  const [people, setPeople] = useState([]);
  const [newName, setNewName] = useState("");
  const [newAddress, setNewAddress] = useState(""); // New state for address input
  const [loading, setLoading] = useState(false); // Loading state for contract interactions

  const navigate = useNavigate();

  // Load saved data from localStorage when component mounts
  useEffect(() => {
    try {
      const savedData = localStorage.getItem("appData");

      if (savedData) {
        const parsedData = JSON.parse(savedData);
        setGroupName(parsedData.groupName || "Lista de Pessoas");
        setPeople(Array.isArray(parsedData.people) ? parsedData.people : []);
      } else {
        setGroupName("Lista de Pessoas");
        setPeople([]);
      }
    } catch (error) {
      console.error("Erro ao carregar dados do localStorage:", error);
      setGroupName("Lista de Pessoas");
      setPeople([]);
    }
  }, []);

  // Save data to localStorage whenever there are changes
  useEffect(() => {
    try {
      const dataToSave = {
        groupName,
        people,
      };
      localStorage.setItem("appData", JSON.stringify(dataToSave));
    } catch (error) {
      console.error("Erro ao salvar dados no localStorage:", error);
    }
  }, [groupName, people]);

  const addPerson = async () => {
    // Only proceed if both name and address are filled
    if (newName.trim() === "" || newAddress.trim() === "") return;
    
    setLoading(true);
    
    try {
      // Call the contract to add the participant
      await addParticipant(newAddress);
      
      // After successful contract interaction, update the UI
      setPeople([
        ...people,
        { 
          id: people.length + 1, 
          name: newName, 
          address: newAddress,
          balance: 0 
        }
      ]);
      
      // Clear inputs after adding
      setNewName("");
      setNewAddress("");
    } catch (error) {
      console.error("Error adding participant to contract:", error);
      alert("Falha ao adicionar participante ao contrato. Verifique o console para mais detalhes.");
    } finally {
      setLoading(false);
    }
  };

  const removePerson = (id) => {
    setPeople(people.filter((person) => person.id !== id));
    // Note: There is no contract function to remove participants in the provided code
  };

  const handleGroupNameChange = (e) => {
    setGroupName(e.target.value);
  };

  const handleKeyDown = (e) => {
    if (e.key === "Enter") {
      setIsEditing(false);
    }
  };

  // Function to handle Enter key in input fields
  const handleInputKeyPress = (e) => {
    if (e.key === "Enter" && newName.trim() !== "" && newAddress.trim() !== "") {
      addPerson();
    }
  };

  return (
    <div className="page-container">
      <div className="top-bar">
        <div className="connect-button-container">
        </div>
      </div>
      
      <div className="card-container">
      <ConnectButton />
        {/* Header */}
        <div className="header">
          {isEditing ? (
            <input
              type="text"
              className="title-input"
              value={groupName}
              onChange={handleGroupNameChange}
              onKeyDown={handleKeyDown}
              onBlur={() => setIsEditing(false)}
              autoFocus
            />
          ) : (
            <div className="title-container" onClick={() => setIsEditing(true)}>
              <h1 className="title">{groupName || "Lista de Pessoas"}</h1>
              <Pencil className="pencil-icon" />
            </div>
          )}
        </div>

        {/* Table */}
        <div className="table-container">
          <table className="people-table">
            <thead>
              <tr className="table-header">
                <th>Nome</th>
                <th>Wallet</th>
                <th>Saldo</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              {people.length > 0 ? (
                people.map((person) => (
                  <tr key={person.id} className="table-row">
                    <td>{person.name}</td>
                    <td>{person.address}</td>
                    <td>
                      {person.balance.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' })}
                    </td>
                    <td>
                      <button
                        onClick={() => removePerson(person.id)}
                        className="remove-button"
                      >
                        X
                      </button>
                    </td>
                  </tr>
                ))
              ) : (
                <tr className="table-row">
                  <td colSpan="4" className="empty-state">
                    Nenhuma pessoa adicionada ainda.
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        {/* Input Section */}
        <div className="input-section">
          <input
            type="text"
            className="name-input"
            placeholder="Nome da pessoa"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onKeyPress={handleInputKeyPress}
            disabled={loading}
          />
          <input
            type="text"
            className="address-input"
            placeholder="Wallet da pessoa"
            value={newAddress}
            onChange={(e) => setNewAddress(e.target.value)}
            onKeyPress={handleInputKeyPress}
            disabled={loading}
          />
          <button 
            onClick={addPerson} 
            className="add-button"
            disabled={newName.trim() === "" || newAddress.trim() === "" || loading}
          >
            {loading ? "Adicionando..." : "Adicionar"}
          </button>
        </div>
      </div>
    </div>
  );
};

export default HomePage;