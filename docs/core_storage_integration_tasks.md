# Detalhamento de Tarefas - Integração Core e Storage

## Introdução

Este documento detalha as tarefas para integrar o módulo `core` (lógica da blockchain) com o módulo `storage` (persistência em RocksDB). O objetivo principal é criar uma estrutura central, como `Blockchain` ou `ChainManager`, que gerencie o estado da cadeia utilizando o `StorageManager` para ler e escrever dados, em vez de mantê-los apenas em memória.

## Estrutura Proposta

Vamos criar uma struct `Blockchain` dentro do módulo `core` (ou em um novo módulo `chain` se a complexidade aumentar). Esta struct conterá uma instância do `StorageManager`.

```rust
// Em src/core/mod.rs ou src/chain/mod.rs
use crate::storage::StorageManager;
use crate::core::{Block, Hash, Transaction};
use crate::consensus; // Para acesso a funções de validação e cálculo de dificuldade

pub struct Blockchain {
    storage: StorageManager,
    // Talvez manter o hash/altura do último bloco em memória para acesso rápido?
    // current_tip_hash: Option<Hash>,
    // current_height: Option<u64>,
}
```

## Tarefas de Implementação

1.  **Definir a Struct `Blockchain`:**
    *   **Tarefa:** Criar a struct `Blockchain` no local apropriado (`core` ou `chain`).
    *   **Tarefa:** Implementar o método `new(storage: StorageManager)` para inicializar a struct. Este método pode carregar o último estado (hash/altura) do `storage` para a memória, se decidido.
    *   **Objetivo:** Ter a estrutura básica que encapsula o acesso ao storage.

2.  **Implementar Métodos de Acesso à Cadeia:**
    *   **Tarefa:** Implementar `get_block_by_hash(&self, hash: &Hash) -> Result<Option<Block>, Error>` que delega para `storage.get_block_by_hash`.
    *   **Tarefa:** Implementar `get_block_by_height(&self, height: u64) -> Result<Option<Block>, Error>` que delega para `storage.get_block_by_height`.
    *   **Tarefa:** Implementar `get_last_block_hash(&self) -> Result<Option<Hash>, Error>` que delega para `storage.get_last_block_hash` (ou retorna valor em memória).
    *   **Tarefa:** Implementar `get_chain_height(&self) -> Result<Option<u64>, Error>` que delega para `storage.get_chain_height` (ou retorna valor em memória).
    *   **Objetivo:** Fornecer uma API consistente para consultar a blockchain através da struct `Blockchain`.

3.  **Implementar Adição de Bloco (`add_block`):**
    *   **Tarefa:** Criar o método `add_block(&mut self, block: Block) -> Result<(), String>`.
    *   **Tarefa:** Dentro de `add_block`, realizar validações essenciais:
        *   Verificar se `block.header.previous_hash` corresponde ao hash do último bloco atual na cadeia (obtido via `get_last_block_hash`).
        *   Verificar se a altura do bloco (`block.header.height`) é a esperada (`current_height + 1`).
        *   Recalcular e verificar o Merkle Root das transações.
        *   Verificar o Proof-of-Work (`consensus::verify_pow`) usando a dificuldade armazenada no *próprio* cabeçalho do bloco.
        *   Verificar se a dificuldade armazenada no cabeçalho é a dificuldade *correta* que deveria ter sido calculada para este bloco usando `consensus::calculate_next_difficulty` com base nos blocos anteriores (obtidos do `storage`).
        *   Validar as transações dentro do bloco (regras de consenso, assinaturas, etc. - *adiado por enquanto*).
    *   **Tarefa:** Se todas as validações passarem, salvar o bloco usando `storage.save_block(&block)`.
    *   **Tarefa:** Atualizar o estado em memória (`current_tip_hash`, `current_height`), se aplicável.
    *   **Objetivo:** Ter um método seguro e validado para adicionar novos blocos à cadeia persistida.

4.  **Implementar Criação do Bloco Gênesis:**
    *   **Tarefa:** Criar uma função ou método (ex: `Blockchain::initialize_genesis()` ou `Blockchain::create_genesis_if_needed()`) que:
        *   Verifica se a cadeia já foi inicializada (ex: `get_chain_height().is_none()`).
        *   Se não, cria o bloco gênesis (altura 0, hash anterior zero, sem transações ou com transação coinbase inicial, dificuldade inicial definida).
        *   Salva o bloco gênesis usando `storage.save_block()`.
    *   **Objetivo:** Garantir que a blockchain possa ser inicializada corretamente.

5.  **Refatorar Testes Existentes:**
    *   **Tarefa:** Atualizar testes nos módulos `consensus` e `storage` que dependiam de uma cadeia em memória (`Vec<Block>`) para usar a nova struct `Blockchain` e seu acesso via `StorageManager`.
    *   **Objetivo:** Manter a cobertura de testes e garantir que os módulos funcionem corretamente com a persistência.

## Próximos Passos Após Tarefas Iniciais

*   Implementar validação de transações.
*   Gerenciar a pool de transações pendentes.
*   Integrar com o módulo `network` para receber e propagar blocos/transações.
*   Implementar lógica para lidar com forks (reorganização da cadeia).
*   Adicionar uma interface RPC/API para interagir com o nó.

