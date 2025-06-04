import React, { useState } from 'react';
import WalletCreator from './components/WalletCreator';
import WalletList from './components/WalletList';
import CoinCreator from './components/CoinCreator';
import TransactionCreator from './components/TransactionCreator';
import { Wallet } from './utils/wallet';
import './App.css';

function App() {
  const [selectedWallet, setSelectedWallet] = useState<Wallet | null>(null);
  const [activeTab, setActiveTab] = useState<'carteiras' | 'criar-moeda' | 'transacoes'>('carteiras');

  const handleWalletCreated = () => {
    // Atualizar a lista de carteiras quando uma nova for criada
    setActiveTab('carteiras');
  };

  const handleSelectWallet = (wallet: Wallet) => {
    setSelectedWallet(wallet);
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
                  <div className="mb-2">
                    <span className="text-gray-600">Saldo:</span>
                    <div className="text-2xl font-bold">{selectedWallet.balance} COINS</div>
                  </div>
                </div>
              ) : (
                <div className="text-center py-4 text-gray-500">
                  <p>Nenhuma carteira selecionada</p>
                  <p className="mt-2 text-sm">Selecione uma carteira da lista ou crie uma nova</p>
                </div>
              )}
            </div>

            <div className="bg-white rounded-lg shadow-md overflow-hidden">
              <div className="flex border-b">
                <button
                  className={`flex-1 py-3 px-4 text-center font-medium ${
                    activeTab === 'carteiras' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('carteiras')}
                >
                  Carteiras
                </button>
                <button
                  className={`flex-1 py-3 px-4 text-center font-medium ${
                    activeTab === 'criar-moeda' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('criar-moeda')}
                >
                  Criar Moeda
                </button>
                <button
                  className={`flex-1 py-3 px-4 text-center font-medium ${
                    activeTab === 'transacoes' ? 'bg-blue-100 text-blue-800' : 'text-gray-600 hover:bg-gray-50'
                  }`}
                  onClick={() => setActiveTab('transacoes')}
                >
                  Transações
                </button>
              </div>
              <div className="p-4">
                {activeTab === 'carteiras' && (
                  <div>
                    <WalletCreator onWalletCreated={handleWalletCreated} />
                    <div className="mt-6">
                      <WalletList onSelectWallet={handleSelectWallet} />
                    </div>
                  </div>
                )}
                {activeTab === 'criar-moeda' && (
                  <CoinCreator selectedWallet={selectedWallet} />
                )}
                {activeTab === 'transacoes' && (
                  <TransactionCreator selectedWallet={selectedWallet} />
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
