import React, { useState, useEffect } from 'react';
import WalletCreator from './components/WalletCreator';
import WalletList from './components/WalletList';
import CoinCreator from './components/CoinCreator';
import TransactionCreator from './components/TransactionCreator';
import TokenList from './components/TokenList';
import FileStorage from './components/FileStorage'; // Importar componente de armazenamento de arquivos
import { Wallet, getWallets } from './utils/wallet';
import { listTokens, getTokenBalance, TokenMetadata, walletToHexAddress } from './utils/api';
import './App.css';

function App() {
  const [selectedWallet, setSelectedWallet] = useState<Wallet | null>(null);
  const [activeTab, setActiveTab] = useState<
    'carteiras' | 'criar-moeda' | 'transacoes' | 'tokens' | 'arquivos' // Adicionar aba 'arquivos'
  >('carteiras');
  const [wallets, setWallets] = useState<Wallet[]>([]); // Estado para armazenar as carteiras
  const [allTokens, setAllTokens] = useState<TokenMetadata[]>([]); // Estado para todos os tokens
  const [tokenBalances, setTokenBalances] = useState<Record<string, number>>({}); // Estado para saldos de tokens da carteira selecionada { tokenId: balance }
  const [loadingBalances, setLoadingBalances] = useState<boolean>(false);

  // Carregar carteiras do localStorage e lista de tokens da API ao montar o componente
  useEffect(() => {
    setWallets(getWallets());
    const fetchInitialData = async () => {
      try {
        const tokens = await listTokens();
        setAllTokens(tokens);
      } catch (error) {
        console.error("Erro ao buscar lista de tokens:", error);
        // Opcional: mostrar erro para o usuário
      }
    };
    fetchInitialData();
  }, []);

  const handleWalletCreated = (newWallet: Wallet) => {
    // Atualizar a lista de carteiras quando uma nova for criada
    setWallets((prevWallets) => [...prevWallets, newWallet]);
    setActiveTab('carteiras');
  };

  // Atualizado para buscar saldos de tokens quando uma carteira é selecionada
  const handleSelectWallet = async (wallet: Wallet) => {
    setSelectedWallet(wallet);
    setTokenBalances({}); // Limpar saldos antigos
    setLoadingBalances(true);
    const balances: Record<string, number> = {};
    const addressHex = walletToHexAddress(wallet);
    try {
      for (const token of allTokens) {
        try {
          const balance = await getTokenBalance(addressHex, token.metadata_hash);
          balances[token.metadata_hash] = balance;
        } catch (error) {
          console.error(`Erro ao buscar saldo do token ${token.symbol} para ${addressHex}:`, error);
          balances[token.metadata_hash] = 0; // Assumir 0 se houver erro
        }
      }
      setTokenBalances(balances);
    } catch (error) {
      console.error("Erro ao buscar saldos de tokens:", error);
      // Opcional: mostrar erro para o usuário
    } finally {
      setLoadingBalances(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-blue-800 text-white p-4 shadow-md">
        <div className="container mx-auto">
          <h1 className="text-3xl font-bold">Blockchain Data Storage</h1>
          <p className="text-blue-200">Interface de gerenciamento de carteiras e transações</p>
        </div>
      </header>

      <main className="container mx-auto py-8 px-4">
        <div className="flex flex-col md:flex-row gap-6">
          {/* Sidebar */}
          <div className="md:w-1/3">
            <div className="bg-white rounded-lg shadow-md p-6 mb-6">
              <h2 className="text-xl font-bold mb-4">Carteira Selecionada</h2>
              {selectedWallet ? (
                <div>
                  <div className="mb-2">
                    <span className="text-gray-600">Endereço:</span>
                    <div className="font-mono text-sm bg-gray-50 p-2 rounded mt-1 break-all">
                      {selectedWallet.address}
                    </div>
                  </div>
                  <div className="mb-4">
                    <span className="text-gray-600">Saldo Nativo:</span>
                    <div className="text-2xl font-bold">{selectedWallet.balance} COINS</div>
                  </div>

                  {/* Exibir saldos de tokens */}
                  <h3 className="text-lg font-semibold mb-2 border-t pt-3">Saldos de Tokens</h3>
                  {loadingBalances ? (
                    <div className="text-gray-500">Carregando saldos...</div>
                  ) : (
                    <div className="space-y-2">
                      {allTokens.length === 0 && <p className="text-sm text-gray-500">Nenhum token encontrado.</p>}
                      {allTokens.map((token) => (
                        <div key={token.metadata_hash} className="flex justify-between items-center text-sm">
                          <span className="text-gray-700">{token.symbol}:</span>
                          <span className="font-medium">{(tokenBalances[token.metadata_hash] ?? 0).toLocaleString()}</span>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              ) : (
                <div className="text-center py-4 text-gray-500">
                  <p>Nenhuma carteira selecionada</p>
                  <p className="mt-2 text-sm">Selecione uma carteira da lista ou crie uma nova</p>
                </div>
              )}
            </div>

            <div className="bg-white rounded-lg shadow-md overflow-hidden">
              <div className="flex flex-wrap border-b">
                <button
                  className={`flex-1 py-3 px-2 text-center font-medium ${
                    activeTab === 'carteiras' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('carteiras')}
                >
                  Carteiras
                </button>
                <button
                  className={`flex-1 py-3 px-2 text-center font-medium ${
                    activeTab === 'criar-moeda' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('criar-moeda')}
                >
                  Criar Moeda
                </button>
                <button
                  className={`flex-1 py-3 px-2 text-center font-medium ${
                    activeTab === 'transacoes' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('transacoes')}
                >
                  Transações
                </button>
                <button
                  className={`flex-1 py-3 px-2 text-center font-medium ${
                    activeTab === 'arquivos' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('arquivos')}
                >
                  Arquivos
                </button>
              </div>
              <div className="p-4">
                {activeTab === 'carteiras' && (
                  <div>
                    <WalletCreator onWalletCreated={handleWalletCreated} />
                    <div className="mt-6">
                      {/* Passar o estado wallets para WalletList */}
                      <WalletList wallets={wallets} selectedWallet={selectedWallet} onSelectWallet={handleSelectWallet} />
                    </div>
                  </div>
                )}
                {activeTab === 'criar-moeda' && (
                  <CoinCreator selectedWallet={selectedWallet} />
                )}
                {activeTab === 'transacoes' && (
                  <TransactionCreator selectedWallet={selectedWallet} wallets={wallets} />
                )}
                {activeTab === 'arquivos' && (
                  <FileStorage selectedWallet={selectedWallet} />
                )}
              </div>
            </div>
          </div>

          {/* Main Content */}
          <div className="md:w-2/3">
            <div className="bg-white rounded-lg shadow-md p-6">
              <h2 className="text-2xl font-bold mb-4">Sobre o Projeto</h2>
              <p className="mb-4">
                Este é um projeto de blockchain especializada em armazenamento de dados, com interface web para interação com as funcionalidades básicas.
              </p>
              <h3 className="text-xl font-bold mt-6 mb-2">Funcionalidades</h3>
              <ul className="list-disc pl-5 space-y-2">
                <li>Criação e gerenciamento de carteiras</li>
                <li>Criação de moedas personalizadas</li>
                <li>Envio e recebimento de transações</li>
                <li>Armazenamento seguro de dados na blockchain</li>
              </ul>
              
              <div className="mt-8 p-4 bg-yellow-50 border border-yellow-200 rounded-md">
                <h3 className="text-lg font-bold text-yellow-800 mb-2">Modo de Demonstração</h3>
                <p className="text-yellow-700">
                  Esta interface está atualmente em modo de demonstração. As operações são simuladas localmente e não estão
                  sendo enviadas para a blockchain. A integração com a API RPC será implementada em breve.
                </p>
              </div>
            </div>
          </div>
        </div>
      </main>

      <footer className="bg-gray-800 text-white p-4 mt-8">
        <div className="container mx-auto text-center">
          <p>&copy; 2025 Blockchain Data Storage. Todos os direitos reservados.</p>
        </div>
      </footer>
    </div>
  );
}

export default App;
