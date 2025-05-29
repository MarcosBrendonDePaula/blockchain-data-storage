# Detalhamento de Tarefas - Nó Executável e Main Loop

Este documento detalha as tarefas necessárias para criar o ponto de entrada executável (`src/main.rs`) e o loop principal do nó da blockchain.

## Tarefas

1.  **[X] Definir Estrutura e Dependências:**
    *   Criar o arquivo `src/main.rs`.
    *   Adicionar dependências necessárias ao `Cargo.toml` (ex: `tokio` para async runtime, `clap` para argumentos de linha de comando, `env_logger` para logging).
    *   Definir a estrutura básica da função `main` assíncrona.

2.  **[ ] Inicialização dos Módulos:**
    *   Inicializar o logger (`env_logger`).
    *   Implementar parsing básico de argumentos de linha de comando (usando `clap`) para configurações como diretório de dados e endereço de escuta.
    *   Inicializar o `StorageManager` com o diretório de dados fornecido.
    *   Inicializar a `Blockchain` usando o `StorageManager`.
    *   Garantir que o bloco gênesis seja criado se necessário.
    *   Envolver a `Blockchain` em `Arc<Mutex>` para compartilhamento seguro entre threads/tasks.

3.  **[ ] Iniciar Loop de Rede:**
    *   Chamar a função `network::start_network_node`, passando a `Blockchain` compartilhada.
    *   Garantir que o `start_network_node` execute o loop principal de eventos da rede (já implementado).

4.  **[ ] Adicionar Tarefas Adicionais (Opcional/Futuro):**
    *   Integrar um gatilho para iniciar a mineração (ex: periodicamente ou via comando).
    *   Integrar a inicialização de uma interface RPC/API se implementada.

5.  **[ ] Testes (Integração):**
    *   Criar testes de integração (se possível) que iniciem o nó e verifiquem a inicialização básica e a escuta na rede.
    *   Testar manualmente a execução do nó com diferentes argumentos.

6.  **[ ] Documentação:**
    *   Atualizar o `README.md` com instruções sobre como compilar e executar o nó, incluindo os argumentos de linha de comando disponíveis.
    *   Adicionar documentação (doc comments) ao `main.rs`.

7.  **[ ] Commit Incremental:**
    *   Realizar o commit das alterações no repositório Git.

