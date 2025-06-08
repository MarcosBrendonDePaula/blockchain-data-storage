import React, { useState, useEffect } from 'react';
import { Wallet } from '../utils/wallet';
import { sendTransaction, sendTokenTransferTransaction, walletToHexAddress, listTokens, TokenMetadata } from '../utils/api';

interface TransactionCreatorProps {
  selectedWallet: Wallet | null;
  wallets: Wallet[];
  onTransactionSent?: (amount: number) => void;
}

const TransactionCreator: React.FC<TransactionCreatorProps> = ({ selectedWallet, wallets, onTransactionSent }) => {
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [transactionHash, setTransactionHash] = useState<string | null>(null);
  const [transactionType, setTransactionType] = useState<'native' | 'token'>('native');
  const [tokens, setTokens] = useState<TokenMetadata[]>([]);
  const [selectedToken, setSelectedToken] = useState<string>('');
  const [loadingTokens, setLoadingTokens] = useState<boolean>(false);
  
  // Carregar lista de tokens disponíveis
  useEffect(() => {
    const fetchTokens = async () => {
      try {
        setLoadingTokens(true);
        const fetchedTokens = await listTokens();
        setTokens(fetchedTokens);
        if (fetchedTokens.length > 0) {
          setSelectedToken(fetchedTokens[0].metadata_hash);
        }
      } catch (error) {
        console.error("Erro ao carregar tokens:", error);
        setError("Não foi possível carregar a lista de tokens. Tente novamente mais tarde.");
      } finally {
        setLoadingTokens(false);
      }
    };
    
    fetchTokens();
  }, []);

  const handleSendTransaction = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedWallet) {
      setError('Selecione uma carteira de origem');
      return;
    }

    if (!recipient) {
      setError('Selecione um destinatário');
      return;
    }

    if (!amount) {
      setError('Informe um valor para a transação');
      return;
    }

    const amountValue = parseFloat(amount);
    if (isNaN(amountValue) || amountValue <= 0) {
      setError('Forneça um valor válido para a transação');
      return;
    }

    // Validação específica para transferência de token
    if (transactionType === 'token' && !selectedToken) {
      setError('Selecione um token para transferir');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Chamada real para a API RPC da blockchain
      const senderAddress = walletToHexAddress(selectedWallet);
      const recipientAddress = recipient; // Assumindo que o recipient já é um endereço válido
      
      // Convertendo endereços de string para Uint8Array para a API
      const senderBytes = new TextEncoder().encode(senderAddress);
      const recipientBytes = new TextEncoder().encode(recipientAddress);
      
      let txHash: string;
      
      if (transactionType === 'native') {
        // Transferência de moeda nativa
        txHash = await sendTransaction(
          senderBytes,
          recipientBytes,
          amountValue
        );
      } else {
        // Transferência de token específico
        txHash = await sendTokenTransferTransaction(
          senderBytes,
          recipientBytes,
          selectedToken,
          amountValue
        );
      }
      
      setTransactionHash(txHash);
      setIsSuccess(true);
      if (onTransactionSent) {
        onTransactionSent(amountValue);
      }
    } catch (err: any) {
      setError(`Erro ao enviar transação: ${err.message || 'Tente novamente mais tarde.'}`);
      console.error('Erro ao enviar transação:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Enviar Transação</h2>
      
      {!selectedWallet ? (
        <div className="text-center py-8 text-gray-500">
          <p>Selecione uma carteira para enviar uma transação.</p>
        </div>
      ) : isSuccess ? (
        <div className="text-center py-8">
          <div className="text-green-600 font-bold text-xl mb-2">Transação enviada com sucesso!</div>
          <p className="mb-4">
            <span className="font-bold">{amount} {transactionType === 'native' ? 'COINS' : 
              tokens.find(t => t.metadata_hash === selectedToken)?.symbol || 'TOKEN'}</span> foram enviados para o endereço {recipient.substring(0, 10)}...{recipient.substring(recipient.length - 10)}.
          </p>
          <button
            onClick={() => {
              setIsSuccess(false);
              setRecipient('');
              setAmount('');
            }}
            className="py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors"
          >
            Enviar Nova Transação
          </button>
        </div>
      ) : (
        <form onSubmit={handleSendTransaction} className="space-y-4">
          <div>
            <label htmlFor="wallet" className="block text-sm font-medium text-gray-700 mb-1">
              Carteira de Origem
            </label>
            <div className="p-2 bg-gray-50 border border-gray-200 rounded text-sm">
              {selectedWallet.address}
            </div>
            <div className="mt-1 text-sm text-gray-500">
              Saldo disponível: <span className="font-medium">{selectedWallet.balance} COINS</span>
            </div>
          </div>
          
          {/* Tipo de transação */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Tipo de Transação
            </label>
            <div className="flex space-x-4">
              <label className="inline-flex items-center">
                <input
                  type="radio"
                  className="form-radio"
                  name="transactionType"
                  value="native"
                  checked={transactionType === 'native'}
                  onChange={() => setTransactionType('native')}
                  disabled={isLoading}
                />
                <span className="ml-2">Moeda Nativa (COINS)</span>
              </label>
              <label className="inline-flex items-center">
                <input
                  type="radio"
                  className="form-radio"
                  name="transactionType"
                  value="token"
                  checked={transactionType === 'token'}
                  onChange={() => setTransactionType('token')}
                  disabled={isLoading || tokens.length === 0}
                />
                <span className="ml-2">Token</span>
              </label>
            </div>
          </div>
          
          {/* Seletor de token (apenas visível quando transactionType === 'token') */}
          {transactionType === 'token' && (
            <div>
              <label htmlFor="tokenSelect" className="block text-sm font-medium text-gray-700 mb-1">
                Selecione o Token
              </label>
              {loadingTokens ? (
                <div className="text-sm text-gray-500">Carregando tokens...</div>
              ) : tokens.length === 0 ? (
                <div className="text-sm text-red-500">Nenhum token disponível</div>
              ) : (
                <select
                  id="tokenSelect"
                  value={selectedToken}
                  onChange={(e) => setSelectedToken(e.target.value)}
                  className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  disabled={isLoading}
                >
                  {tokens.map((token) => (
                    <option key={token.metadata_hash} value={token.metadata_hash}>
                      {token.name} ({token.symbol})
                    </option>
                  ))}
                </select>
              )}
            </div>
          )}
          
          <div>
            <label htmlFor="recipientAddress" className="block text-sm font-medium text-gray-700 mb-1">
              Endereço do Destinatário
            </label>
            <input
              type="text"
              id="recipientAddress"
              value={recipient}
              onChange={(e) => setRecipient(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Endereço da carteira de destino"
              disabled={isLoading}
            />
          </div>
          
          <div>
            <label htmlFor="amount" className="block text-sm font-medium text-gray-700 mb-1">
              Quantidade
            </label>
            <input
              type="number"
              id="amount"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Ex: 100"
              min="0.000001"
              max={transactionType === 'native' ? selectedWallet.balance.toString() : undefined}
              step="0.000001"
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
            disabled={isLoading || (transactionType === 'token' && tokens.length === 0)}
            className="w-full py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors disabled:bg-blue-300 disabled:cursor-not-allowed"
          >
            {isLoading ? 'Enviando...' : 'Enviar Transação'}
          </button>
        </form>
      )}
    </div>
  );
};

export default TransactionCreator;
