import React, { useState } from 'react';

const AddExpenses = () => {
  const [expenseName, setExpenseName] = useState('');
  const [expenseValue, setExpenseValue] = useState('');
  const [expenses, setExpenses] = useState([]);
  const [selectedNames, setSelectedNames] = useState([]);
  const [payer, setPayer] = useState(''); // Estado para armazenar quem pagou

  // Lista de nomes de exemplo
  const names = ['João', 'Maria', 'Pedro', 'Ana'];

  const handleAddExpense = () => {
    if (expenseName && expenseValue && payer) {
      const newExpense = {
        name: expenseName,
        value: parseFloat(expenseValue),
        payer: payer, // Adiciona quem pagou à despesa
      };
      setExpenses([...expenses, newExpense]);
      setExpenseName('');
      setExpenseValue('');
      setPayer(''); // Limpa o selecionado após adicionar
    } else {
      alert('Preencha todos os campos e selecione quem pagou.');
    }
  };

  const handleNameSelection = (name) => {
    if (selectedNames.includes(name)) {
      setSelectedNames(selectedNames.filter((n) => n !== name));
    } else {
      setSelectedNames([...selectedNames, name]);
    }
  };

  const handlePayerSelection = (name) => {
    setPayer(name); // Define quem pagou (só pode selecionar um)
  };

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: '100vh',
        width: '100vw',
        backgroundColor: '#f9f9f9',
        color: '#000',
      }}
    >
      <div
        style={{
          backgroundColor: '#fff',
          padding: '20px',
          borderRadius: '8px',
          boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
          width: '90%',
          maxWidth: '800px', // Aumentei a largura máxima para acomodar as duas listas
        }}
      >
        <h1 style={{ textAlign: 'center', marginBottom: '20px', color: '#000' }}>Adicionar Despesa</h1>
        <div style={{ marginBottom: '15px' }}>
          <label style={{ color: '#000' }}>
            Nome da Despesa:
            <input
              type="text"
              value={expenseName}
              onChange={(e) => setExpenseName(e.target.value)}
              style={{
                marginLeft: '10px',
                width: '100%',
                padding: '8px',
                borderRadius: '4px',
                border: '1px solid #ccc',
                color: '#000',
              }}
            />
          </label>
        </div>
        <div style={{ marginBottom: '15px' }}>
          <label style={{ color: '#000' }}>
            Valor da Despesa:
            <input
              type="number"
              value={expenseValue}
              onChange={(e) => setExpenseValue(e.target.value)}
              style={{
                marginLeft: '10px',
                width: '100%',
                padding: '8px',
                borderRadius: '4px',
                border: '1px solid #ccc',
                color: '#000',
              }}
            />
          </label>
        </div>

        {/* Listas de nomes */}
        <div style={{ display: 'flex', gap: '20px', marginBottom: '20px' }}>
          {/* Lista de nomes (checkbox) */}
          <div style={{ flex: 1 }}>
            <h2 style={{ textAlign: 'center', color: '#000' }}>Participantes</h2>
            <ul style={{ listStyle: 'none', padding: '0', textAlign: 'center' }}>
              {names.map((name, index) => (
                <li key={index} style={{ marginBottom: '10px', color: '#000' }}>
                  <label>
                    <input
                      type="checkbox"
                      checked={selectedNames.includes(name)}
                      onChange={() => handleNameSelection(name)}
                      style={{ marginRight: '10px' }}
                    />
                    {name}
                  </label>
                </li>
              ))}
            </ul>
          </div>

          {/* Lista de quem pagou (radio button) */}
          <div style={{ flex: 1 }}>
            <h2 style={{ textAlign: 'center', color: '#000' }}>Quem pagou</h2>
            <ul style={{ listStyle: 'none', padding: '0', textAlign: 'center' }}>
              {names.map((name, index) => (
                <li key={index} style={{ marginBottom: '10px', color: '#000' }}>
                  <label>
                    <input
                      type="radio"
                      name="payer"
                      checked={payer === name}
                      onChange={() => handlePayerSelection(name)}
                      style={{ marginRight: '10px' }}
                    />
                    {name}
                  </label>
                </li>
              ))}
            </ul>
          </div>
        </div>

        <button
          onClick={handleAddExpense}
          style={{
            width: '100%',
            padding: '10px',
            backgroundColor: '#007bff',
            color: '#fff',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
          }}
        >
          Adicionar Despesa
        </button>

        <h2 style={{ textAlign: 'center', marginTop: '20px', color: '#000' }}>Despesas Adicionadas</h2>
        <ul style={{ listStyle: 'none', padding: '0', textAlign: 'center' }}>
          {expenses.map((expense, index) => (
            <li key={index} style={{ marginBottom: '10px', color: '#000' }}>
              {expense.name}: R$ {expense.value.toFixed(2)} (Pago por: {expense.payer})
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default AddExpenses;