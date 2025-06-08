import axios from 'axios';

// URL base da API RPC da blockchain
const API_URL = 'http://localhost:8000'; // Ajuste conforme necessário

// Interface para os parâmetros da requisição JSON-RPC
interface JsonRpcRequest {
  jsonrpc: string;
  method: string;
  params: any;
  id: number;
}

// Interface para a resposta JSON-RPC
interface JsonRpcResponse<T> {
  jsonrpc: string;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: any;
  };
  id: number;
}

// Interface para o resultado da consulta de saldo
interface BalanceResult {
  balance: number;
}

// Interface para o resultado da criação de token
interface TokenCreationResult {
  token_name: string;
  token_symbol: string;
  initial_supply: number;
  transaction_hash: string;
  metadata_hash: string;
}

// Interface para metadados de token (usado em list_tokens)
export interface TokenMetadata {
  name: string;
  symbol: string;
  total_supply: number;
  creator: string; // Assuming address is returned as hex string
  creation_timestamp: number;
  metadata_hash: string; // Hex-encoded hash
}

// Interface para o resultado da consulta de saldo de token
interface TokenBalanceResult {
  balance: number;
}

/**
 * Função para fazer chamadas à API RPC da blockchain
 * @param method Nome do método RPC a ser chamado
 * @param params Parâmetros para o método
 * @returns Resultado da chamada RPC
 */
export async function callRpcMethod<T>(method: string, params: any): Promise<T> {
  const request: JsonRpcRequest = {
    jsonrpc: '2.0',
    method,
    params,
    id: Date.now()
  };

  try {
    const response = await axios.post<JsonRpcResponse<T>>(API_URL, request);
    
    if (response.data.error) {
      throw new Error(`Erro RPC: ${response.data.error.message} (código: ${response.data.error.code})`);
    }
    
    return response.data.result as T;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(`Erro de conexão: ${error.message}`);
    }
    throw error;
  }
}

/**
 * Envia uma transação para a blockchain
 * @param sender Endereço do remetente
 * @param recipient Endereço do destinatário
 * @param amount Quantidade a ser enviada
 * @returns Hash da transação
 */
export async function sendTransaction(sender: Uint8Array, recipient: Uint8Array, amount: number): Promise<string> {
  return callRpcMethod<string>('send_transaction', {
    sender: Array.from(sender),
    recipient: Array.from(recipient),
    amount
  });
}

/**
 * Envia uma transação de transferência de token para a blockchain
 * @param sender Endereço do remetente (Uint8Array)
 * @param recipient Endereço do destinatário (Uint8Array)
 * @param tokenId Hash do metadado do token (hex string)
 * @param amount Quantidade do token a ser enviada
 * @returns Hash da transação
 */
export async function sendTokenTransferTransaction(
  sender: Uint8Array,
  recipient: Uint8Array,
  tokenId: string,
  amount: number
): Promise<string> {
  return callRpcMethod<string>("send_transaction", {
    sender: Array.from(sender),
    token_recipient: Array.from(recipient), // Use token_recipient for token transfers
    token_id: tokenId,
    token_amount: amount,
  });
}

/**
 * Envia uma transação de armazenamento de dados para a blockchain
 * @param sender Endereço do remetente
 * @param payload_base64 Dados a serem armazenados (codificados em base64)
 * @returns Hash da transação
 */
export async function sendStorageTransaction(sender: Uint8Array, payload_base64: string): Promise<string> {
  return callRpcMethod<string>('send_transaction', {
    sender: Array.from(sender),
    payload_base64
  });
}

/**
 * Obtém a altura atual da blockchain
 * @returns Altura atual da blockchain
 */
export async function getChainHeight(): Promise<number> {
  return callRpcMethod<number>('get_chain_height', {});
}

/**
 * Obtém um bloco pelo seu hash
 * @param hash Hash do bloco
 * @returns Dados do bloco
 */
export async function getBlockByHash(hash: string): Promise<any> {
  return callRpcMethod<any>('get_block_by_hash', { hash });
}

/**
 * Obtém um bloco pela sua altura
 * @param height Altura do bloco
 * @returns Dados do bloco
 */
export async function getBlockByHeight(height: number): Promise<any> {
  return callRpcMethod<any>('get_block_by_height', { height });
}

/**
 * Recupera dados armazenados off-chain
 * @param hash Hash dos dados
 * @returns Dados recuperados (codificados em base64)
 */
export async function getOffchainData(hash: string): Promise<string | null> {
  return callRpcMethod<string | null>('get_offchain_data', { hash });
}

/**
 * Consulta o saldo de um endereço na blockchain
 * @param address Endereço da carteira em formato hexadecimal
 * @returns Saldo da carteira
 */
export async function getBalance(address: string): Promise<number> {
  const result = await callRpcMethod<BalanceResult>('get_balance', { address });
  return result.balance;
}

/**
 * Cria um novo token/moeda na blockchain
 * @param creatorAddress Endereço do criador do token em formato hexadecimal
 * @param tokenName Nome do token
 * @param tokenSymbol Símbolo do token (abreviação)
 * @param initialSupply Suprimento inicial do token
 * @returns Informações sobre o token criado
 */
export async function createToken(
  creatorAddress: string,
  tokenName: string,
  tokenSymbol: string,
  initialSupply: number
): Promise<TokenCreationResult> {
  return callRpcMethod<TokenCreationResult>('create_token', {
    creator_address: creatorAddress,
    token_name: tokenName,
    token_symbol: tokenSymbol,
    initial_supply: initialSupply
  });
}

/**
 * Converte um objeto Wallet para o formato de endereço hexadecimal
 * @param wallet Objeto da carteira
 * @returns Endereço da carteira em formato hexadecimal
 */
export function walletToHexAddress(wallet: { address: string }): string {
  // Se o endereço já estiver em formato hexadecimal, retorna diretamente
  if (wallet.address.startsWith('0x')) {
    return wallet.address.substring(2); // Remove o prefixo '0x'
  }
  return wallet.address;
}



/**
 * Lista todos os tokens registrados na blockchain.
 * @returns Uma promessa que resolve para um array de metadados de tokens.
 */
export async function listTokens(): Promise<TokenMetadata[]> {
  try {
    // Note: Assuming the backend returns the array directly in the result field
    const response = await callRpcMethod<TokenMetadata[]>('list_tokens', {});
    return response;
  } catch (error) {
    console.error("Erro ao listar tokens:", error);
    throw new Error(`Erro ao listar tokens: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Obtém o saldo de um token específico para uma carteira.
 * @param address Endereço da carteira (hex).
 * @param tokenId Hash do metadado do token (hex).
 * @returns Uma promessa que resolve para o saldo do token.
 */
export async function getTokenBalance(address: string, tokenId: string): Promise<number> {
  try {
    const response = await callRpcMethod<TokenBalanceResult>('get_token_balance', { address, token_id: tokenId });
    return response.balance;
  } catch (error) {
    console.error(`Erro ao obter saldo do token ${tokenId} para a carteira ${address}:`, error);
    throw new Error(`Erro ao obter saldo do token: ${error instanceof Error ? error.message : String(error)}`);
  }
}

