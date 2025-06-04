import React, { useState } from 'react';
import { generateWallet, saveWallet, Wallet } from '../utils/wallet';

interface WalletCreatorProps {
  onWalletCreated?: (wallet: Wallet) => void;
}

const WalletCreator: React.FC<WalletCreatorProps> = ({ onWalletCreated }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [newWallet, setNewWallet] = useState<Wallet | null>(null);
  const [showPrivateKey, setShowPrivateKey] = useState(false);
  const [isSaved, setIsSaved] = useState(false);

  const handleCreateWallet = () => {
    setIsLoading(true);
    
    // Simular um pequeno atraso para dar feedback visual ao usuário
    setTimeout(() => {
      const wallet = generateWallet();
      setNewWallet(wallet);
      setIsLoading(false);
      
      if (onWalletCreated) {
        onWalletCreated(wallet);
      }
    }, 500);
  };

  const handleSaveWallet = () => {
    if (newWallet) {
      saveWallet(newWallet);
      setIsSaved(true);
    }
  };

  const toggleShowPrivateKey = () => {
    setShowPrivateKey(!showPrivateKey);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // Poderia adicionar uma notificação de "Copiado!" aqui
  };

  return (
    <div className="p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Criar Nova Carteira</h2>
      
      {!newWallet ? (
        <button
          onClick={handleCreateWallet}
          disabled={isLoading}
          className="w-full py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors"
        >
          {isLoading ? 'Gerando...' : 'Gerar Nova Carteira'}
        </button>
      ) : (
        <div className="space-y-4">
          <div className="p-4 border border-gray-200 rounded-md">
            <div className="flex justify-between items-center mb-2">
              <span className="font-semibold text-gray-700">Endereço:</span>
              <button 
                onClick={() => copyToClipboard(newWallet.address)}
                className="text-xs text-blue-600 hover:text-blue-800"
              >
                Copiar
              </button>
            </div>
            <p className="text-sm bg-gray-50 p-2 rounded break-all">{newWallet.address}</p>
          </div>
          
          <div className="p-4 border border-gray-200 rounded-md">
            <div className="flex justify-between items-center mb-2">
              <span className="font-semibold text-gray-700">Chave Privada:</span>
              <div className="flex space-x-2">
                <button 
                  onClick={toggleShowPrivateKey}
                  className="text-xs text-blue-600 hover:text-blue-800"
                >
                  {showPrivateKey ? 'Ocultar' : 'Mostrar'}
                </button>
                <button 
                  onClick={() => copyToClipboard(newWallet.privateKey)}
                  className="text-xs text-blue-600 hover:text-blue-800"
                >
                  Copiar
                </button>
              </div>
            </div>
            {showPrivateKey ? (
              <p className="text-sm bg-gray-50 p-2 rounded break-all">{newWallet.privateKey}</p>
            ) : (
              <p className="text-sm bg-gray-50 p-2 rounded">••••••••••••••••••••••••••••••••</p>
            )}
            <div className="mt-2">
              <div className="text-red-600 text-xs">
                <strong>IMPORTANTE:</strong> Nunca compartilhe sua chave privada. Quem tiver acesso a ela terá controle total sobre sua carteira.
              </div>
            </div>
          </div>
          
          <div className="flex justify-between">
            <button
              onClick={handleCreateWallet}
              className="py-2 px-4 bg-gray-200 text-gray-800 font-semibold rounded-lg shadow-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-opacity-75 transition-colors"
            >
              Gerar Nova
            </button>
            
            <button
              onClick={handleSaveWallet}
              disabled={isSaved}
              className={`py-2 px-4 font-semibold rounded-lg shadow-md focus:outline-none focus:ring-2 focus:ring-opacity-75 transition-colors ${
                isSaved 
                  ? 'bg-green-100 text-green-800 cursor-not-allowed' 
                  : 'bg-green-600 text-white hover:bg-green-700 focus:ring-green-500'
              }`}
            >
              {isSaved ? 'Carteira Salva' : 'Salvar Carteira'}
            </button>
          </div>
          
          {isSaved && (
            <div className="mt-2 text-sm text-green-600">
              Carteira salva com sucesso no armazenamento local do navegador.
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default WalletCreator;
