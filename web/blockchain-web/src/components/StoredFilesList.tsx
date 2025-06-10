import React, { useState, useEffect } from 'react';
import { Wallet } from '../utils/wallet';
import { getOffchainData } from '../utils/api';

interface StoredFile {
  hash: string;
  name: string;
  type: string;
  size: number;
  timestamp: number;
}

interface StoredFilesListProps {
  selectedWallet: Wallet | null;
  storedFiles: StoredFile[];
  isLoading: boolean;
}

const StoredFilesList: React.FC<StoredFilesListProps> = ({ selectedWallet, storedFiles, isLoading }) => {
  const [selectedFile, setSelectedFile] = useState<StoredFile | null>(null);
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [loadingContent, setLoadingContent] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleViewFile = async (file: StoredFile) => {
    setSelectedFile(file);
    setLoadingContent(true);
    setError(null);
    
    try {
      const content = await getOffchainData(file.hash);
      setFileContent(content);
    } catch (err: any) {
      setError(`Erro ao carregar arquivo: ${err.message || 'Tente novamente mais tarde.'}`);
      console.error('Erro ao carregar arquivo:', err);
    } finally {
      setLoadingContent(false);
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleString();
  };

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  };

  if (!selectedWallet) {
    return (
      <div className="text-center py-8 text-gray-500">
        <p>Selecione uma carteira para ver os arquivos armazenados.</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="text-center py-8 text-gray-500">
        <p>Carregando arquivos armazenados...</p>
      </div>
    );
  }

  return (
    <div className="mt-6">
      <h3 className="text-xl font-bold mb-4">Arquivos Armazenados</h3>
      
      {storedFiles.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <p>Nenhum arquivo armazenado por esta carteira.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Nome</th>
                  <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Tamanho</th>
                  <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Data</th>
                  <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Ações</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {storedFiles.map((file) => (
                  <tr key={file.hash} className={selectedFile?.hash === file.hash ? 'bg-blue-50' : ''}>
                    <td className="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{file.name}</td>
                    <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-500">{formatFileSize(file.size)}</td>
                    <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-500">{formatDate(file.timestamp)}</td>
                    <td className="px-4 py-4 whitespace-nowrap text-sm">
                      <button
                        onClick={() => handleViewFile(file)}
                        className="text-blue-600 hover:text-blue-800"
                      >
                        Visualizar
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          
          <div className="border rounded-lg p-4">
            {selectedFile ? (
              <div>
                <h4 className="font-bold mb-2">{selectedFile.name}</h4>
                <div className="text-sm mb-4">
                  <p>Hash: <span className="font-mono">{selectedFile.hash.substring(0, 10)}...{selectedFile.hash.substring(selectedFile.hash.length - 10)}</span></p>
                  <p>Tipo: {selectedFile.type || 'Desconhecido'}</p>
                  <p>Tamanho: {formatFileSize(selectedFile.size)}</p>
                  <p>Data: {formatDate(selectedFile.timestamp)}</p>
                </div>
                
                {loadingContent ? (
                  <div className="text-center py-4 text-gray-500">
                    <p>Carregando conteúdo do arquivo...</p>
                  </div>
                ) : error ? (
                  <div className="text-red-600 text-sm">
                    {error}
                  </div>
                ) : fileContent ? (
                  <div className="mt-4">
                    {selectedFile.type.startsWith('image/') ? (
                      <img 
                        src={`data:${selectedFile.type};base64,${fileContent}`} 
                        alt={selectedFile.name}
                        className="max-w-full h-auto"
                      />
                    ) : (
                      <div className="bg-gray-50 p-4 rounded overflow-auto max-h-60 font-mono text-sm">
                        {fileContent.length > 1000 
                          ? `${fileContent.substring(0, 1000)}... (arquivo muito grande para exibição completa)` 
                          : fileContent}
                      </div>
                    )}
                  </div>
                ) : (
                  <div className="text-center py-4 text-gray-500">
                    <p>Clique em "Visualizar" para carregar o conteúdo do arquivo.</p>
                  </div>
                )}
              </div>
            ) : (
              <div className="text-center py-8 text-gray-500">
                <p>Selecione um arquivo para visualizar os detalhes.</p>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default StoredFilesList;
