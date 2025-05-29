# Blockchain Data Storage

Este repositório contém o código fonte para uma blockchain experimental focada em armazenamento de dados descentralizado, implementada em Rust.

## Visão Geral

O objetivo deste projeto é criar uma rede blockchain segura e eficiente que permita aos usuários armazenar dados (como texto e JSON) de forma privada e resistente à censura. A rede utiliza um mecanismo de consenso Proof-of-Work (PoW) e uma arquitetura híbrida onde os metadados são registrados on-chain e os dados brutos são armazenados em uma camada off-chain distribuída. As taxas de transação para armazenamento são proporcionais ao tamanho dos dados.

**Status Atual:** Desenvolvimento em andamento. O nó possui módulos básicos para core, consenso (PoW + ajuste), rede (P2P com libp2p) e armazenamento (RocksDB). O nó agora pode ser executado como um binário.

## Arquitetura Modular

O núcleo da blockchain está sendo desenvolvido com uma arquitetura modular:

*   **`src/core`**: Estruturas de dados (Bloco, Transação), lógica de validação, serialização, hashing, mempool e gerenciamento da cadeia (`Blockchain`).
*   **`src/consensus`**: Lógica do Proof-of-Work, verificação e ajuste de dificuldade.
*   **`src/network`**: Comunicação P2P (libp2p), descoberta de peers, propagação de blocos/transações (Gossipsub).
*   **`src/storage`**: Persistência da blockchain no disco (RocksDB).
*   **`src/mempool.rs`**: Gerenciamento de transações pendentes.
*   **`src/main.rs`**: Ponto de entrada executável do nó, inicialização e loop principal.

Documentos de design e tarefas podem ser encontrados no diretório `docs/`.

## Começando (Getting Started)

**Pré-requisitos:**
*   Rust e Cargo instalados (veja [rustup.rs](https://rustup.rs/))
*   Dependências do RocksDB (geralmente `libclang`, `clang`, `llvm`, `cmake` - consulte a documentação do `rust-rocksdb` para seu sistema operacional).

**Compilação:**
```bash
cargo build
```

**Execução do Nó:**
```bash
# Executa o nó usando o diretório de dados padrão (.blockchain_data)
cargo run

# Especifica um diretório de dados diferente
cargo run -- --data-dir /caminho/para/seu/diretorio
```
O nó iniciará, inicializará (ou carregará) a blockchain e começará a escutar por conexões P2P.

**Execução de Testes:**
```bash
cargo test
```

## Contribuindo

Este projeto está em desenvolvimento ativo. Se você deseja contribuir:

1.  Consulte os documentos no diretório `docs/` para entender o design e as tarefas planejadas.
2.  Verifique as [Issues](https://github.com/MarcosBrendonDePaula/blockchain-data-storage/issues) do repositório para tarefas abertas e discussões.
3.  Siga as boas práticas de desenvolvimento em Rust e contribua com código claro, testado e documentado.

## Licença

(A definir - Considerar MIT ou Apache-2.0)

