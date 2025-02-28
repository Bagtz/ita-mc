import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Login from "./components/Login";
import Home from "./components/HomePage";
import AddExpenses from "./components/AddExpenses";

import "@rainbow-me/rainbowkit/styles.css";
import { RainbowKitProvider } from "@rainbow-me/rainbowkit";
import { WagmiProvider, createConfig } from "wagmi";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
import { http } from "viem";

// Libs to interact with contracts
import { getContract } from 'viem';
import { wagmiAbi } from './abi';
import { publicClient, walletClient } from './client';

// Define local Arbitrum dev chain
const localArbitrum = {
  id: 412346,
  name: "Local Arbitrum Nitro",
  network: "arbitrum-local",
  nativeCurrency: {
    name: "ETH",
    symbol: "ETH",
    decimals: 18,
  },
  rpcUrls: {
    default: { http: ["http://localhost:8547"] },
  },
};

// Configurar Wagmi
const config = createConfig({
  chains: [localArbitrum],
  transports: {
    [localArbitrum.id]: http('http://localhost:8547'),
  },
});

const queryClient = new QueryClient();

// Criar instância do contrato
const contract = getContract({
  address: '0xa6e41ffd769491a42a6e5ce453259b93983a22ef',
  abi: wagmiAbi,
  client: { public: publicClient, wallet: walletClient },
  chain: localArbitrum,
});



// Funções para interagir com o contrato
export const addParticipant = async (wallet) => {
  try {
    const tx = await contract.write.addParticipant([wallet]);
    return tx;
  } catch (error) {
    console.error("Error adding participant:", error);
    throw error;
  }
};

export const getBalance = async (wallet) => {
  try {
    return await contract.read.getBalance([wallet]);
  } catch (error) {
    console.error("Error getting balance:", error);
    throw error;
  }
};

export const isParticipant = async (wallet) => {
  try {
    return await contract.read.isParticipant([wallet]);
  } catch (error) {
    console.error("Error checking participant status:", error);
    throw error;
  }
};

export const simplifyDebts = async () => {
  try {
    const tx = await contract.write.simplifyDebts();
    return tx;
  } catch (error) {
    console.error("Error simplifying debts:", error);
    throw error;
  }
};

export const splitEqually = async (payer, borrowers, amount) => {
  try {
    const tx = await contract.write.splitEqually([payer, borrowers, amount]);
    return tx;
  } catch (error) {
    console.error("Error splitting equally:", error);
    throw error;
  }
};

export const chains = [localArbitrum];

export default function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider chains={[localArbitrum]}>
          <Router>
            <Routes>
              <Route path="/" element={<Login />} />
              <Route path="/home" element={<Home />} />
              <Route path="/add-expenses" element={<AddExpenses />} />
            </Routes>
          </Router>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}