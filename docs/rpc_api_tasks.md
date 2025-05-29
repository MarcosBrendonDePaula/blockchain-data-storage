# Detalhamento de Tarefas - Interface RPC/API

Este documento detalha as tarefas necessárias para criar uma interface JSON-RPC sobre HTTP para interagir com o nó da blockchain.

## Objetivos

*   Permitir o envio de novas transações para o mempool do nó.
*   Permitir a consulta do estado da blockchain (altura, último bloco).
*   Permitir a consulta de blocos específicos (por altura ou hash).
*   Permitir a consulta de transações específicas (por hash).

## Escolha Tecnológica

*   **Framework Web:** `actix-web` ou `axum` (ambos são populares e performáticos no ecossistema Rust).
*   **Serialização/Deserialização:** `serde`, `serde_json`.
*   **Especificação:** JSON-RPC 2.0.

## Tarefas

1.  **[ ] Configuração e Dependências:**
    *   Adicionar as dependências necessárias ao `Cargo.toml` (ex: `actix-web`, `serde`, `serde_json`, `tokio`).
    *   Criar um novo módulo (ex: `src/rpc.rs`) para o código da API.
    *   Definir as estruturas de dados para as requisições e respostas JSON-RPC (usando `serde`).

2.  **[ ] Implementar Servidor HTTP Básico:**
    *   Configurar um servidor HTTP básico (usando `actix-web` ou `axum`) que escute em uma porta configurável (via argumento de linha de comando ou arquivo de configuração).
    *   Integrar a inicialização e execução do servidor RPC no `main.rs`, rodando-o concorrentemente com o loop de rede `libp2p`.

3.  **[ ] Implementar Endpoints RPC:**
    *   **`send_transaction`**: Recebe uma transação serializada (ex: JSON), a deserializa e a adiciona ao mempool da `Blockchain` (acessando via `Arc<Mutex>`). Retorna o hash da transação ou um erro.
    *   **`get_chain_height`**: Consulta a altura atual da `Blockchain`. Retorna a altura.
    *   **`get_block_by_height`**: Recebe uma altura, consulta o bloco correspondente na `Blockchain` (via `StorageManager`). Retorna o bloco serializado ou um erro se não encontrado.
    *   **`get_block_by_hash`**: Recebe um hash de bloco (string hexadecimal), consulta o bloco correspondente. Retorna o bloco serializado ou um erro.
    *   **`get_transaction_by_hash`**: Recebe um hash de transação, busca a transação (na cadeia ou talvez no mempool). Retorna a transação serializada ou um erro.
    *   **`get_mempool_info`**: Retorna informações sobre o estado do mempool (ex: número de transações pendentes).

4.  **[ ] Tratamento de Erros:**
    *   Mapear erros internos da blockchain (ex: `BlockchainError`, `StorageError`) para erros JSON-RPC padronizados.

5.  **[ ] Testes:**
    *   Criar testes unitários para a lógica de manipulação das requisições RPC.
    *   Criar testes de integração que iniciem o servidor RPC e enviem requisições reais (usando um cliente HTTP como `reqwest`), verificando as respostas.

6.  **[ ] Documentação:**
    *   Documentar os endpoints RPC disponíveis, seus parâmetros e formatos de resposta (ex: em um arquivo `docs/rpc_api.md` ou no `README.md`).
    *   Adicionar doc comments ao código do módulo `rpc`.

7.  **[ ] Commit Incremental:**
    *   Realizar o commit das alterações no repositório Git.

