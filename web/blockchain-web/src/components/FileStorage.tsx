import React, { useState, useRef, useEffect } from 'react';
import { Wallet } from '../utils/wallet';
import { sendStorageTransaction, walletToHexAddress } from '../utils/api';
import StoredFilesList from './StoredFilesList';

interface FileStorageProps {
  selectedWallet: Wallet | null;
}

interface StoredFile {
  hash: string;
  name: string;
  type: string;
  size: number;
  timestamp: number;
}

const FileStorage: React.FC<FileStorageProps> = ({ selectedWallet }) => {
  const [file, setFile] = useState<File | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isSuccess, setIsSuccess] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [transactionHash, setTransactionHash] = useState<string | null>(null);
  const [estimatedCost, setEstimatedCost] = useState<number>(0);
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  // Estado para armazenar os arquivos da carteira selecionada
  const [storedFiles, setStoredFiles] = useState<StoredFile[]>([]);
  const [loadingFiles, setLoadingFiles] = useState<boolean>(false);

  // Taxa de custo por KB (exemplo)
  const COST_PER_KB = 0.01;

  // Efeito para carregar arquivos armazenados quando a carteira é selecionada
  useEffect(() => {
    if (selectedWallet) {
      loadStoredFiles();
    } else {
      setStoredFiles([]);
    }
  }, [selectedWallet]);

  // Função para carregar arquivos armazenados pela carteira selecionada
  const loadStoredFiles = async () => {
    if (!selectedWallet) return;
    
    setLoadingFiles(true);
    
    try {
      // Simulação de dados para demonstração
      // Em um ambiente real, isso seria uma chamada à API para buscar os arquivos da carteira
      setTimeout(() => {
        const mockFiles: StoredFile[] = [
          {
            hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            name: "documento.pdf",
            type: "application/pdf",
            size: 1024 * 1024 * 2.5, // 2.5 MB
            timestamp: Date.now() - 86400000 * 2 // 2 dias atrás
          },
          {
            hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            name: "imagem.jpg",
            type: "image/jpeg",
            size: 1024 * 512, // 512 KB
            timestamp: Date.now() - 86400000 // 1 dia atrás
          }
        ];
        setStoredFiles(mockFiles);
        setLoadingFiles(false);
      }, 1000);
    } catch (error) {
      console.error("Erro ao carregar arquivos armazenados:", error);
      setLoadingFiles(false);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const selectedFile = e.target.files[0];
      setFile(selectedFile);
      
      // Calcular custo estimado baseado no tamanho do arquivo
      const fileSizeInKB = selectedFile.size / 1024;
      const cost = fileSizeInKB * COST_PER_KB;
      setEstimatedCost(parseFloat(cost.toFixed(4)));
      
      setError(null);
      setIsSuccess(false);
    }
  };

  const handleUpload = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedWallet) {
      setError('Selecione uma carteira para armazenar o arquivo');
      return;
    }

    if (!file) {
      setError('Selecione um arquivo para armazenar');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Converter arquivo para base64
      const fileBase64 = await readFileAsBase64(file);
      
      // Preparar e enviar transação
      const senderAddress = walletToHexAddress(selectedWallet);
      const senderBytes = new TextEncoder().encode(senderAddress);
      
      const txHash = await sendStorageTransaction(senderBytes, fileBase64);
      
      setTransactionHash(txHash);
      setIsSuccess(true);
      setFile(null);
      
      // Limpar input de arquivo
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    } catch (err: any) {
      setError(`Erro ao armazenar arquivo: ${err.message || 'Tente novamente mais tarde.'}`);
      console.error('Erro ao armazenar arquivo:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Função para converter arquivo para base64
  const readFileAsBase64 = (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsDataURL(file);
      reader.onload = () => {
        if (typeof reader.result === 'string') {
          // Remover prefixo "data:*/*;base64," para obter apenas o conteúdo base64
          const base64Content = reader.result.split(',')[1];
          resolve(base64Content);
        } else {
          reject(new Error('Falha ao converter arquivo para base64'));
        }
      };
      reader.onerror = error => reject(error);
    });
  };

  // Função para atualizar a lista de arquivos após upload bem-sucedido
  const handleSuccessfulUpload = () => {
    // Adicionar o novo arquivo à lista de arquivos armazenados
    if (file && transactionHash) {
      const newFile: StoredFile = {
        hash: transactionHash,
        name: file.name,
        type: file.type,
        size: file.size,
        timestamp: Date.now()
      };
      
      setStoredFiles(prevFiles => [newFile, ...prevFiles]);
    }
  };

  return (
    <div>
      <div className="p-6 bg-white rounded-lg shadow-md mb-6">
        <h2 className="text-2xl font-bold mb-4">Armazenar Arquivo na Blockchain</h2>
        
        {!selectedWallet ? (
          <div className="text-center py-8 text-gray-500">
            <p>Selecione uma carteira para armazenar arquivos.</p>
          </div>
        ) : isSuccess ? (
        <div className="text-center py-8">
          <div className="text-green-600 font-bold text-xl mb-2">Arquivo armazenado com sucesso!</div>
          <p className="mb-4">
            O arquivo <span className="font-bold">{file?.name}</span> foi armazenado na blockchain.
          </p>
          <div className="mb-4 text-sm">
            <span className="text-gray-600">Hash da transação:</span>
            <div className="font-mono bg-gray-50 p-2 rounded mt-1 break-all">
              {transactionHash}
            </div>
          </div>
          <button
            onClick={() => {
              setIsSuccess(false);
              handleSuccessfulUpload(); // Atualizar lista de arquivos
            }}
            className="py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors"
          >
            Armazenar Outro Arquivo
          </button>
        </div>
      ) : (
        <form onSubmit={handleUpload} className="space-y-4">
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
          
          <div>
            <label htmlFor="file" className="block text-sm font-medium text-gray-700 mb-1">
              Selecione um Arquivo
            </label>
            <input
              type="file"
              id="file"
              ref={fileInputRef}
              onChange={handleFileChange}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              disabled={isLoading}
            />
            {file && (
              <div className="mt-2 text-sm">
                <p>Nome: <span className="font-medium">{file.name}</span></p>
                <p>Tamanho: <span className="font-medium">{(file.size / 1024).toFixed(2)} KB</span></p>
                <p>Tipo: <span className="font-medium">{file.type || 'Desconhecido'}</span></p>
              </div>
            )}
          </div>
          
          {file && (
            <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
              <h3 className="text-lg font-bold text-blue-800 mb-2">Custo de Armazenamento</h3>
              <p className="text-blue-700">
                Custo estimado: <span className="font-bold">{estimatedCost} COINS</span>
              </p>
              <p className="text-sm text-blue-600 mt-1">
                O custo é calculado com base no tamanho do arquivo ({COST_PER_KB} COINS por KB).
              </p>
            </div>
          )}
          
          {error && (
            <div className="text-red-600 text-sm">
              {error}
            </div>
          )}
          
          <button
            type="submit"
            disabled={isLoading || !file}
            className="w-full py-2 px-4 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 transition-colors disabled:bg-blue-300 disabled:cursor-not-allowed"
          >
            {isLoading ? 'Armazenando...' : 'Armazenar Arquivo'}
          </button>
        </form>
      )}
      </div>
      
      {/* Exibir a lista de arquivos armazenados */}
      {selectedWallet && (
        <div className="bg-white rounded-lg shadow-md p-6">
          <StoredFilesList 
            selectedWallet={selectedWallet}
            storedFiles={storedFiles}
            isLoading={loadingFiles}
          />
        </div>
      )}
    </div>
  );
};

export default FileStorage;
