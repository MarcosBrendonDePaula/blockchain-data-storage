import React, { useState, useEffect } from 'react';
import { Wallet } from '../utils/wallet';
import { getBalance, walletToHexAddress } from '../utils/api';

interface WalletListProps {
  wallets: Wallet[];
  selectedWallet: Wallet | null;
  onSelectWallet: (wallet: Wallet) => void;
}

const WalletList: React.FC<WalletListProps> = ({ wallets, selectedWallet, onSelectWallet }) => {
  const [walletBalances, setWalletBalances] = useState<Record<string, number>>({});
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Obter saldos reais das carteiras através da API RPC
  useEffect(() => {
    const fetchBalances = async () => {
      if (wallets.length === 0) return;
      
      setIsLoading(true);
      setError(null);
      const balances: Record<string, number> = {};
      
      try {
        // Chamada real para a API da blockchain para obter saldos
        for (const wallet of wallets) {
          try {
            const walletAddress = walletToHexAddress(wallet);
            const balance = await getBalance(walletAddress);
            balances[wallet.address] = balance;
          } catch (err) {
            console.error(`Erro ao obter saldo da carteira ${wallet.address}:`, err);
            // Em caso de erro para uma carteira específica, definimos o saldo como 0
            balances[wallet.address] = 0;
          }
        }
        
        setWalletBalances(balances);
      } catch (err: any) {
        setError(`Erro ao obter saldos: ${err.message}`);
        console.error('Erro ao obter saldos:', err);
      } finally {
        setIsLoading(false);
      }
    };
    
    fetchBalances();
  }, [wallets]);

  return (
    <div className="p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Suas Carteiras</h2>
      
      {error && (
        <div className="p-3 mb-4 bg-red-100 text-red-700 rounded-md">
          {error}
        </div>
      )}
      
      {isLoading && (
        <div className="p-3 mb-4 bg-blue-50 text-blue-700 rounded-md">
          Carregando saldos...
        </div>
      )}
      
      {wallets.length === 0 ? (
        <p className="text-gray-500">Nenhuma carteira encontrada. Crie uma nova carteira para começar.</p>
      ) : (
        <ul className="space-y-2">
          {wallets.map((wallet) => (
            <li 
              key={wallet.address}
              className={`p-4 border rounded-md cursor-pointer hover:bg-gray-50 ${
                selectedWallet?.address === wallet.address ? 'bg-blue-50 border-blue-500' : ''
              }`}
              onClick={() => onSelectWallet(wallet)}
            >
              <div className="flex justify-between items-center">
                <div>
                  <p className="font-medium">{wallet.name || 'Carteira sem nome'}</p>
                  <p className="text-sm text-gray-500 break-all">{wallet.address}</p>
                </div>
                <div className="text-right">
                  <p className="font-bold">{walletBalances[wallet.address] !== undefined ? walletBalances[wallet.address] : '...'} BDC</p>
                  <p className="text-xs text-gray-500">Saldo</p>
                </div>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default WalletList;
