# Detalhamento de Tarefas Iniciais - Módulo Storage (Persistência)

## Introdução

Este documento detalha as tarefas iniciais para o desenvolvimento do módulo `storage`, responsável pela persistência dos dados da blockchain (principalmente os blocos) no disco local. O objetivo é garantir que o estado da cadeia possa ser recuperado após o nó ser reiniciado.

## Escolha da Tecnologia

Para a persistência local, utilizaremos o **RocksDB**, um banco de dados key-value embarcado de alta performance, otimizado para armazenamento rápido e eficiente, comumente usado em sistemas blockchain. Usaremos o crate `rust-rocksdb`.

## Tarefas Iniciais

1.  **Configuração da Dependência e Estrutura Básica:**
    *   **Tarefa:** Adicionar a dependência `rocksdb` ao `Cargo.toml`.
    *   **Tarefa:** Criar a estrutura `StorageManager` (ou similar) no módulo `storage` que encapsulará a instância do banco de dados RocksDB.
    *   **Tarefa:** Implementar a lógica para abrir (ou criar, se não existir) o banco de dados RocksDB em um diretório específico (ex: `./db`).
    *   **Objetivo:** Ter a conexão com o banco de dados estabelecida e gerenciada.

2.  **Definição do Esquema de Chaves (Key Schema):**
    *   **Tarefa:** Definir como os dados serão armazenados no RocksDB. Um esquema simples pode ser:
        *   `b'h' + block_height (u64 big-endian)` -> `block_hash ([u8; 32])` (Mapeia altura para hash)
        *   `b'b' + block_hash ([u8; 32])` -> `serialized_block (Vec<u8>)` (Armazena o bloco serializado pelo hash)
        *   `b't' + tx_hash ([u8; 32])` -> `serialized_tx (Vec<u8>)` (Opcional: indexar transações por hash)
        *   `b'l'` -> `last_block_hash ([u8; 32])` (Armazena o hash do último bloco da cadeia principal)
        *   `b'H'` -> `chain_height (u64 big-endian)` (Armazena a altura atual da cadeia)
    *   **Objetivo:** Ter um plano claro de como os dados serão organizados no banco de dados key-value.

3.  **Implementação das Funções de Persistência:**
    *   **Tarefa:** Implementar a função `save_block(block: &Block)` que serializa o bloco (usando `serde` e `bincode` ou `serde_json`) e o salva no RocksDB de acordo com o esquema definido (armazenar por hash, atualizar mapeamento altura->hash, atualizar último hash e altura).
    *   **Tarefa:** Implementar a função `get_block_by_hash(hash: &Hash) -> Option<Block>` que busca um bloco serializado pelo seu hash e o desserializa.
    *   **Tarefa:** Implementar a função `get_block_by_height(height: u64) -> Option<Block>` que busca o hash pela altura e depois busca o bloco pelo hash.
    *   **Tarefa:** Implementar funções para obter o último hash (`get_last_block_hash()`) e a altura atual (`get_chain_height()`).
    *   **Objetivo:** Ser capaz de salvar e recuperar blocos e informações essenciais da cadeia.

4.  **Integração com o Módulo Core:**
    *   **Tarefa:** Modificar a struct `Blockchain` no módulo `core` para utilizar o `StorageManager` em vez de manter a cadeia (`Vec<Block>`) apenas em memória.
    *   **Tarefa:** Atualizar métodos como `add_block` (ou similar, após mineração), `get_last_block_hash`, `is_chain_valid` para interagir com a camada de armazenamento.
    *   **Objetivo:** A lógica principal da blockchain operar sobre dados persistidos.

## Próximos Passos Após Tarefas Iniciais

*   Implementar tratamento de erros mais robusto.
*   Otimizar operações de escrita (batch writes).
*   Considerar o uso de Column Families no RocksDB para organizar melhor os diferentes tipos de dados.
*   Implementar a persistência do estado da pool de transações pendentes (se necessário).
*   Adicionar lógica para lidar com forks e reorganizações da cadeia.

