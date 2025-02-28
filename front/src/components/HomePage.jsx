import React, { useState, useEffect } from "react";
import { Pencil } from "lucide-react";
import { useNavigate } from "react-router-dom";
import "./HomePage.css"; // Importando o CSS

const HomePage = () => {
  const [groupName, setGroupName] = useState("");
  const [isEditing, setIsEditing] = useState(false);
  const [people, setPeople] = useState([]);
  const [newName, setNewName] = useState("");
  const [newBalance, setNewBalance] = useState("");

  const navigate = useNavigate();

  // Carrega os dados salvos no localStorage ao montar o componente
  useEffect(() => {
    try {
      const savedData = localStorage.getItem("appData");
      console.log("Dados brutos carregados do localStorage:", savedData);

      if (savedData) {
        const parsedData = JSON.parse(savedData);
        console.log("Dados parsed do localStorage:", parsedData);
        
        // Verifica se parsedData.groupName e parsedData.people têm valores válidos
        setGroupName(parsedData.groupName || "Lista de Pessoas");
        setPeople(Array.isArray(parsedData.people) ? parsedData.people : []);
      } else {
        console.log("Nenhum dado encontrado no localStorage, usando valores padrão");
        setGroupName("Lista de Pessoas");
        setPeople([]);
      }
    } catch (error) {
      console.error("Erro ao carregar dados do localStorage:", error);
      console.log("Usando valores padrão devido a erro");
      setGroupName("Lista de Pessoas");
      setPeople([]);
    }
  }, []); // Executa apenas uma vez ao montar o componente

  // Salva os dados no localStorage sempre que houver alterações
  useEffect(() => {
    try {
      const dataToSave = {
        groupName,
        people,
      };
      localStorage.setItem("appData", JSON.stringify(dataToSave));
      console.log("Dados salvos no localStorage:", dataToSave);
    } catch (error) {
      console.error("Erro ao salvar dados no localStorage:", error);
    }
  }, [groupName, people]); // Executa toda vez que groupName ou people mudam

  const addPerson = () => {
    if (newName.trim() === "") return;
    setPeople([
      ...people,
      { id: people.length + 1, name: newName, balance: 0 }
    ]);
    setNewName("");
    setNewBalance("");
  };

  const removePerson = (id) => {
    setPeople(people.filter((person) => person.id !== id));
  };

  const handleGroupNameChange = (e) => {
    setGroupName(e.target.value);
  };

  const handleKeyDown = (e) => {
    if (e.key === "Enter") {
      setIsEditing(false);
    }
  };

  // Função para lidar com a tecla Enter no input de nome
  const handleNameInputKeyPress = (e) => {
    if (e.key === "Enter") {
      addPerson();
    }
  };

  return (
    <div className="page-container">
      <div className="card-container">
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
                <th>Saldo</th>
                <th>Ações</th>
              </tr>
            </thead>
            <tbody>
              {people.length > 0 ? (
                people.map((person) => (
                  <tr key={person.id} className="table-row">
                    <td>{person.name}</td>
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
                  <td colSpan="3" className="empty-state">
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
            onKeyPress={handleNameInputKeyPress} // Mantém o evento para Enter
          />
          <button onClick={addPerson} className="add-button">
            Adicionar
          </button>
        </div>
      </div>
    </div>
  );
};

export default HomePage;