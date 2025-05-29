# Detalhamento de Tarefas - Mempool e Mineração

## Introdução

Este documento detalha as tarefas para implementar o gerenciamento de transações pendentes (Mempool) e a lógica completa de mineração de blocos, integrando os módulos `core`, `consensus` e `storage`.

## Mempool

O Mempool (Memory Pool) armazena transações que foram recebidas pela rede mas ainda não foram incluídas em um bloco. Ele serve como fonte para os mineradores selecionarem transações ao construir um novo bloco.

### Estrutura Proposta

Criaremos uma struct `Mempool` (provavelmente em um novo módulo `mempool` ou dentro do `core`).

```rust
// Em src/mempool/mod.rs ou src/core/mempool.rs
use crate::core::{Transaction, Hash};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
pub struct Mempool {
    // Usar HashMap para acesso rápido por hash e evitar duplicatas
    transactions: HashMap<Hash, Transaction>,
    // Manter uma ordem (ex: FIFO ou por taxa - FIFO por enquanto)
    order: VecDeque<Hash>,
    // Limite máximo de transações no mempool
    max_size: usize,
}
```

### Tarefas de Implementação (Mempool)

1.  **Definir Struct `Mempool`:**
    *   **Tarefa:** Criar a struct `Mempool` com os campos `transactions`, `order` e `max_size`.
    *   **Tarefa:** Implementar `Mempool::new(max_size: usize)`.
2.  **Implementar `add_transaction`:**
    *   **Tarefa:** Criar `Mempool::add_transaction(&mut self, tx: Transaction) -> Result<(), String>`.
    *   **Tarefa:** Dentro do método:
        *   Validar a transação (regras básicas: formato, talvez assinatura no futuro - *adiado*).
        *   Verificar se a transação já existe no `transactions` (usando `tx.calculate_hash()`).
        *   Verificar se o mempool está cheio (`transactions.len() >= max_size`). Se sim, remover a mais antiga (`order.pop_front()`) ou rejeitar a nova (rejeitar por enquanto).
        *   Adicionar a transação ao `transactions` e seu hash ao `order`.
    *   **Objetivo:** Adicionar transações válidas e não duplicadas ao mempool, respeitando o limite de tamanho.
3.  **Implementar `get_transactions`:**
    *   **Tarefa:** Criar `Mempool::get_transactions(&self, max_count: usize) -> Vec<Transaction>`.
    *   **Tarefa:** Retornar até `max_count` transações do início da `order`, clonando-as do `transactions`.
    *   **Objetivo:** Permitir que o minerador obtenha um lote de transações pendentes para incluir em um bloco.
4.  **Implementar `remove_transactions`:**
    *   **Tarefa:** Criar `Mempool::remove_transactions(&mut self, tx_hashes: &[Hash])`.
    *   **Tarefa:** Remover as transações correspondentes aos hashes fornecidos do `transactions` e da `order`.
    *   **Objetivo:** Remover transações que foram incluídas com sucesso em um bloco.
5.  **Integrar com `Blockchain`:**
    *   **Tarefa:** Adicionar um campo `mempool: Mempool` à struct `Blockchain`.
    *   **Tarefa:** Criar um método `Blockchain::add_pending_transaction(&mut self, tx: Transaction)` que chama `self.mempool.add_transaction`.

## Lógica de Mineração

Esta lógica será responsável por construir um novo bloco candidato, minerá-lo e adicioná-lo à cadeia.

### Tarefas de Implementação (Mineração)

1.  **Criar Método `mine_new_block`:**
    *   **Tarefa:** Implementar `Blockchain::mine_new_block(&mut self /*, miner_address: Address */) -> Result<Block, BlockchainError>`.
    *   **Tarefa:** Dentro do método:
        *   Verificar se a blockchain está inicializada (`current_height.is_some()`).
        *   Obter o hash e a altura do último bloco (`self.get_last_block_hash()`, `self.get_chain_height()`).
        *   Obter transações pendentes do mempool (`self.mempool.get_transactions(MAX_TX_PER_BLOCK)`).
        *   *Opcional:* Adicionar uma transação Coinbase recompensando o minerador (requer definição de recompensa).
        *   Calcular a dificuldade para o novo bloco usando `consensus::calculate_next_difficulty`.
        *   Construir o cabeçalho do novo bloco (sem o nonce final).
        *   Criar a estrutura `Block` inicial.
        *   Chamar `consensus::mine(&mut block.header, difficulty)` para encontrar o nonce e obter o hash final.
        *   **NÃO** adicionar o bloco à cadeia ainda (apenas retornar o bloco minerado).
    *   **Objetivo:** Encapsular o processo de criação e mineração de um bloco candidato.

2.  **Criar Método `process_mined_block` (ou similar):**
    *   **Tarefa:** Implementar `Blockchain::process_mined_block(&mut self, mined_block: Block) -> Result<(), BlockchainError>`.
    *   **Tarefa:** Dentro do método:
        *   Chamar `self.add_block(mined_block.clone())` para validar e salvar o bloco.
        *   Se `add_block` for bem-sucedido, remover as transações incluídas no `mined_block` do mempool (`self.mempool.remove_transactions`).
    *   **Objetivo:** Processar um bloco que foi minerado (localmente ou recebido pela rede), adicionando-o à cadeia e limpando o mempool.

3.  **Refatorar/Criar Loop Principal (Simulado):**
    *   **Tarefa:** Criar uma função de teste ou um exemplo que simule o loop de um nó: receber transações, tentar minerar um bloco (`mine_new_block`), e processá-lo (`process_mined_block`).

## Próximos Passos

*   Implementar validação de assinaturas de transações.
*   Definir e implementar recompensas de bloco (Coinbase).
*   Implementar lógica de taxas de transação e seleção por taxa no mempool.
*   Integrar com o módulo `network` para receber/transmitir transações e blocos.

