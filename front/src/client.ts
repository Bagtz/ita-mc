import { createPublicClient, createWalletClient, http, custom } from 'viem'
import { mainnet } from 'viem/chains'
import { EthereumProvider } from '@walletconnect/ethereum-provider'
 
export const publicClient = createPublicClient({
  chain: mainnet,
  transport: http(),
})
 
// eg: Metamask
export const walletClient = createWalletClient({
  chain: mainnet,
  transport: custom(window.ethereum!),
})
 
// eg: WalletConnect
const provider = await EthereumProvider.init({
  projectId: "abcd1234",
  showQrModal: true,
  chains: [1],
})
 
export const walletClientWC = createWalletClient({
  chain: mainnet,
  transport: custom(provider),
})