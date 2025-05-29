# Detalhamento de Tarefas - Armazenamento de Dados Off-Chain

Este documento detalha as tarefas necessárias para implementar o armazenamento de dados (ex: JSON, texto) fora da cadeia principal (off-chain), mas referenciados por transações on-chain.

## Objetivos

*   Permitir que transações específicas (ex: `TransactionData::Storage`) contenham uma referência (ex: hash) para dados maiores.
*   Implementar um mecanismo para armazenar esses dados maiores fora da blockchain principal.
*   Implementar um mecanismo para recuperar esses dados usando a referência on-chain.
*   Garantir que o armazenamento off-chain seja acessível (pelo menos localmente inicialmente) e ligado corretamente à transação.

## Escolha Tecnológica (Inicial)

*   **Mecanismo de Armazenamento:** Para simplificar a implementação inicial, usaremos o **sistema de arquivos local** do nó. Cada nó armazenará os dados off-chain que ele mesmo cria ou recebe (se a propagação for implementada). Uma abordagem distribuída (DHT, IPFS) pode ser considerada em fases futuras.
*   **Identificador:** Usaremos o **hash SHA-256** do conteúdo dos dados como identificador único. Este hash será armazenado no campo `payload_hash` da `TransactionData::Storage`.
*   **Localização:** Os dados serão armazenados em um subdiretório dedicado dentro do `data_dir` do nó (ex: `.blockchain_data/offchain_storage/`). O nome do arquivo será o hash hexadecimal do conteúdo.

## Tarefas

1.  **[ ] Modificar Estrutura de Transação:**
    *   Confirmar que a enum `TransactionData` (em `core/mod.rs`) possui a variante `Storage { payload_hash: [u8; 32], /* outros metadados? */ }`.
    *   Garantir que a `Transaction` possa ser criada com este tipo de dado.

2.  **[ ] Criar Módulo `offchain_storage`:**
    *   Criar um novo módulo `src/offchain_storage.rs`.
    *   Declarar o módulo em `src/lib.rs`.
    *   Definir uma struct `OffChainStorageManager`.

3.  **[ ] Implementar `OffChainStorageManager`:**
    *   Método `new(storage_path: PathBuf)`: Inicializa o gerenciador, criando o diretório de armazenamento se não existir.
    *   Método `store_payload(payload: &[u8]) -> Result<[u8; 32], OffChainStorageError>`: Calcula o hash SHA-256 do payload, salva o payload em um arquivo nomeado com o hash hexadecimal no diretório de armazenamento, e retorna o hash (array de bytes).
    *   Método `retrieve_payload(payload_hash: &[u8; 32]) -> Result<Vec<u8>, OffChainStorageError>`: Converte o hash para hexadecimal, busca o arquivo correspondente no diretório e retorna seu conteúdo como `Vec<u8>`.
    *   Definir `OffChainStorageError` para erros como I/O, hash inválido, não encontrado.

4.  **[ ] Integrar com `Blockchain` / Nó:**
    *   Instanciar o `OffChainStorageManager` no `main.rs` junto com os outros gerenciadores.
    *   Modificar o processamento de transações (provavelmente onde `add_pending_transaction` ou a validação de bloco ocorre) para:
        *   Quando uma transação `Storage` é recebida/processada, verificar se o `payload` correspondente ao `payload_hash` existe localmente (usando `retrieve_payload`). *Nota: Inicialmente, o nó só terá os payloads que ele mesmo criou. A recuperação de payloads de outros nós exigiria uma camada de rede adicional.*
    *   Modificar a lógica de criação de transações (seja via RPC ou internamente) para:
        *   Aceitar o payload bruto.
        *   Chamar `store_payload` no `OffChainStorageManager`.
        *   Criar a `Transaction` com a variante `Storage` contendo o `payload_hash` retornado.

5.  **[ ] Atualizar API RPC (se aplicável):**
    *   Modificar o endpoint `send_transaction` para aceitar opcionalmente um campo `payload_data` (ex: string base64).
    *   Se `payload_data` estiver presente, decodificar, chamar `store_payload`, criar a transação `Storage` com o hash, e adicionar ao mempool.
    *   Criar um novo endpoint `get_offchain_data(hash: String)` que chama `retrieve_payload` e retorna os dados (ex: como base64).

6.  **[ ] Testes:**
    *   Testes unitários para `OffChainStorageManager` (store, retrieve, erros).
    *   Testes de integração que simulam o envio de uma transação de armazenamento via RPC (se atualizada), verificam se o dado foi salvo no disco e se pode ser recuperado via hash (localmente ou via novo endpoint RPC).

7.  **[ ] Documentação:**
    *   Documentar o novo módulo `offchain_storage`.
    *   Atualizar a documentação da `TransactionData::Storage`.
    *   Atualizar a documentação da API RPC com as novas funcionalidades/endpoints.
    *   Atualizar o `README.md` explicando o mecanismo básico de armazenamento off-chain.

8.  **[ ] Commit Incremental:**
    *   Realizar o commit das alterações no repositório Git.

