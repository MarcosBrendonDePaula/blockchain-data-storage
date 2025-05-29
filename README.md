# Blockchain Data Storage

Este repositório contém o código fonte para uma blockchain experimental focada em armazenamento de dados descentralizado, implementada em Rust.

## Visão Geral

O objetivo deste projeto é criar uma rede blockchain segura e eficiente que permita aos usuários armazenar dados (como texto e JSON) de forma privada e resistente à censura. A rede utiliza um mecanismo de consenso Proof-of-Work (PoW) e uma arquitetura híbrida onde os metadados são registrados on-chain e os dados brutos são armazenados em uma camada off-chain distribuída. As taxas de transação para armazenamento são proporcionais ao tamanho dos dados.

**Status Atual:** Fase inicial de desenvolvimento. A estrutura básica do projeto e os tipos de dados fundamentais foram definidos.

## Arquitetura Modular

O núcleo da blockchain está sendo desenvolvido com uma arquitetura modular para promover clareza e manutenibilidade:

*   **`src/core`**: Contém as estruturas de dados principais (Bloco, Transação, Cabeçalho), lógica de serialização/deserialização, cálculo de hash e validação básica.
*   **`src/consensus`**: Implementará a lógica do Proof-of-Work, incluindo o algoritmo de hashing, verificação de dificuldade e ajuste de dificuldade.
*   **`src/network`**: Gerenciará a comunicação P2P entre os nós, incluindo descoberta de peers, sincronização e propagação de dados.
*   **`src/storage`**: Responsável pela persistência da blockchain no disco local e, futuramente, pela interação com a camada de armazenamento off-chain.

Para um detalhamento completo do escopo de cada módulo, consulte [docs/module_scopes.md](docs/module_scopes.md).

## Começando (Getting Started)

**Pré-requisitos:**
*   Rust e Cargo instalados (veja [rustup.rs](https://rustup.rs/))

**Compilação:**
```bash
cargo build
```

**Execução de Testes:**
```bash
cargo test
```
(Nota: Os testes atuais são placeholders e serão expandidos à medida que a implementação avança.)

## Contribuindo

Este projeto está em estágio inicial. Se você deseja contribuir:

1.  Consulte o documento [docs/initial_tasks.md](docs/initial_tasks.md) para ver as tarefas de desenvolvimento iniciais definidas.
2.  Verifique as [Issues](https://github.com/MarcosBrendonDePaula/blockchain-data-storage/issues) do repositório para tarefas abertas e discussões.
3.  Siga as boas práticas de desenvolvimento em Rust e contribua com código claro, testado e documentado.

## Licença

(A definir - Considerar MIT ou Apache-2.0)

