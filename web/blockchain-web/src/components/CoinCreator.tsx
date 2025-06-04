import React, { useState } from 'react';
import { Wallet } from '../utils/wallet';
import { createToken, walletToHexAddress } from '../utils/api';

interface CoinCreatorProps {
  selectedWallet: Wallet | null;
  onCoinCreated?: (amount: number) => void;
}

const CoinCreator: React.FC<CoinCreatorProps> = ({ selectedWallet, onCoinCreated }) => {
  const [coinName, setCoinName] = useState('');
  const [coinSymbol, setCoinSymbol] = useState('');
  const [initialSupply, setInitialSupply] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [createdTokenInfo, setCreatedTokenInfo] = useState<any>(null);

  const handleCreateCoin = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedWallet) {
      setError('Selecione uma carteira para criar a moeda');
      return;
    }

    if (!coinName || !coinSymbol || !initialSupply) {
      setError('Preencha todos os campos');
      return;
    }

    const supply = parseFloat(initialSupply);
    if (isNaN(supply) || supply <= 0) {
      setError('Forneça um valor válido para o suprimento inicial');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Chamada real para a API RPC da blockchain
      const walletAddress = walletToHexAddress(selectedWallet);
      const tokenInfo = await createToken(
        walletAddress,
        coinName,
        coinSymbol,
        supply
      );
      
      setCreatedTokenInfo(tokenInfo);
      setIsSuccess(true);
      if (onCoinCreated) {
        onCoinCreated(supply);
      }
    } catch (err: any) {
      setError(`Erro ao criar moeda: ${err.message || 'Tente novamente mais tarde.'}`);
      console.error('Erro ao criar moeda:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Criar Nova Moeda</h2>
      
      {!selectedWallet ? (
        <div className="text-center py-8 text-gray-500">
          <p>Selecione uma carteira para criar uma nova moeda.</p>
        </div>
      ) : isSuccess ? (
        <div className="text-center py-8">
          <div className="text-green-600 font-bold text-xl mb-2">Moeda criada com sucesso!</div>
          <p className="mb-4">
            <span className="font-bold">{coinName} ({coinSymbol})</span> foi criada com um suprimento inicial de {initialSupply} tokens.
          </p>
          <button
            onClick={() => {
              setIsSuccess(false);
              setCoinName('');
              setCoinSymbol('');
              setInitialSupply('');
            }}
            className="py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors"
          >
            Criar Outra Moeda
          </button>
        </div>
      ) : (
        <form onSubmit={handleCreateCoin} className="space-y-4">
          <div>
            <label htmlFor="wallet" className="block text-sm font-medium text-gray-700 mb-1">
              Carteira Selecionada
            </label>
            <div className="p-2 bg-gray-50 border border-gray-200 rounded text-sm">
              {selectedWallet.address}
            </div>
          </div>
          
          <div>
            <label htmlFor="coinName" className="block text-sm font-medium text-gray-700 mb-1">
              Nome da Moeda
            </label>
            <input
              type="text"
              id="coinName"
              value={coinName}
              onChange={(e) => setCoinName(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Ex: MinhaMoeda"
              disabled={isLoading}
            />
          </div>
          
          <div>
            <label htmlFor="coinSymbol" className="block text-sm font-medium text-gray-700 mb-1">
              Símbolo da Moeda
            </label>
            <input
              type="text"
              id="coinSymbol"
              value={coinSymbol}
              onChange={(e) => setCoinSymbol(e.target.value.toUpperCase())}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Ex: MCN"
              maxLength={5}
              disabled={isLoading}
            />
          </div>
          
          <div>
            <label htmlFor="initialSupply" className="block text-sm font-medium text-gray-700 mb-1">
              Suprimento Inicial
            </label>
            <input
              type="number"
              id="initialSupply"
              value={initialSupply}
              onChange={(e) => setInitialSupply(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Ex: 1000000"
              min="1"
              disabled={isLoading}
            />
          </div>
          
          {error && (
            <div className="text-red-600 text-sm">
              {error}
            </div>
          )}
          
          <button
            type="submit"
            disabled={isLoading}
            className="w-full py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors disabled:bg-blue-300 disabled:cursor-not-allowed"
          >
            {isLoading ? 'Criando...' : 'Criar Moeda'}
          </button>
        </form>
      )}
    </div>
  );
};

export default CoinCreator;
