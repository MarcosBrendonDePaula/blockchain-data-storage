import React, { useState, useEffect } from 'react';
import { listTokens, TokenMetadata } from '../utils/api';

const TokenList: React.FC = () => {
  const [tokens, setTokens] = useState<TokenMetadata[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchTokens = async () => {
      try {
        setLoading(true);
        setError(null);
        const fetchedTokens = await listTokens();
        setTokens(fetchedTokens);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Erro desconhecido ao buscar tokens');
        console.error("Erro ao buscar tokens:", err);
      } finally {
        setLoading(false);
      }
    };

    fetchTokens();
  }, []);

  if (loading) {
    return <div className="text-center py-4">Carregando tokens...</div>;
  }

  if (error) {
    return <div className="text-center py-4 text-red-600">Erro ao carregar tokens: {error}</div>;
  }

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-xl font-bold mb-4">Tokens Registrados</h2>
      {tokens.length === 0 ? (
        <p className="text-gray-500">Nenhum token encontrado na blockchain.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Nome</th>
                <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Símbolo</th>
                <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Total Supply</th>
                <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ID (Hash)</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {tokens.map((token) => (
                <tr key={token.metadata_hash}>
                  <td className="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{token.name}</td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-500">{token.symbol}</td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-500">{token.total_supply.toLocaleString()}</td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-500 font-mono break-all" title={token.metadata_hash}>{`${token.metadata_hash.substring(0, 10)}...`}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default TokenList;

