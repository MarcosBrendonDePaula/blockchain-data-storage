import CryptoJS from 'crypto-js';

/**
 * Interface para representar uma carteira na blockchain
 */
export interface Wallet {
  privateKey: string;
  publicKey: string;
  address: string;
  balance: number;
  createdAt: Date;
}

/**
 * Gera uma nova carteira com chave privada aleatória
 * @returns Uma nova instância de Wallet
 */
export function generateWallet(): Wallet {
  // Gera uma chave privada aleatória usando valores aleatórios
  const privateKey = CryptoJS.lib.WordArray.random(32).toString();
  
  // Deriva a chave pública a partir da chave privada (usando SHA-256 como exemplo)
  const publicKey = CryptoJS.SHA256(privateKey).toString();
  
  // Gera um endereço a partir da chave pública (usando os primeiros 40 caracteres do hash)
  const address = CryptoJS.RIPEMD160(publicKey).toString();
  
  return {
    privateKey,
    publicKey,
    address,
    balance: 0,
    createdAt: new Date()
  };
}

/**
 * Salva a carteira no armazenamento local do navegador
 * @param wallet A carteira a ser salva
 */
export function saveWallet(wallet: Wallet): void {
  const wallets = getWallets();
  wallets.push(wallet);
  localStorage.setItem('blockchain_wallets', JSON.stringify(wallets));
}

/**
 * Recupera todas as carteiras salvas no armazenamento local
 * @returns Array de carteiras salvas
 */
export function getWallets(): Wallet[] {
  const walletsJson = localStorage.getItem('blockchain_wallets');
  if (!walletsJson) {
    return [];
  }
  
  try {
    const wallets = JSON.parse(walletsJson);
    return wallets.map((wallet: any) => ({
      ...wallet,
      createdAt: new Date(wallet.createdAt)
    }));
  } catch (error) {
    console.error('Erro ao recuperar carteiras:', error);
    return [];
  }
}

/**
 * Recupera uma carteira específica pelo endereço
 * @param address Endereço da carteira a ser recuperada
 * @returns A carteira encontrada ou undefined se não existir
 */
export function getWalletByAddress(address: string): Wallet | undefined {
  const wallets = getWallets();
  return wallets.find(wallet => wallet.address === address);
}

/**
 * Verifica se uma chave privada é válida para um determinado endereço
 * @param privateKey Chave privada a ser verificada
 * @param address Endereço da carteira
 * @returns true se a chave privada corresponder ao endereço, false caso contrário
 */
export function validatePrivateKey(privateKey: string, address: string): boolean {
  const publicKey = CryptoJS.SHA256(privateKey).toString();
  const derivedAddress = CryptoJS.RIPEMD160(publicKey).toString();
  return derivedAddress === address;
}

/**
 * Atualiza o saldo de uma carteira específica
 * @param address Endereço da carteira
 * @param newBalance Novo saldo
 */
export function updateWalletBalance(address: string, newBalance: number): void {
  const wallets = getWallets();
  const updatedWallets = wallets.map(wallet => {
    if (wallet.address === address) {
      return { ...wallet, balance: newBalance };
    }
    return wallet;
  });
  
  localStorage.setItem('blockchain_wallets', JSON.stringify(updatedWallets));
}
