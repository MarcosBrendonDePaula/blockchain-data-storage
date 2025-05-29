# Detalhamento de Tarefas - Integração Blockchain-Network

## Introdução

Este documento detalha as tarefas para integrar a lógica da `Blockchain` (incluindo mempool e mineração) com o módulo `network` (libp2p), permitindo que o nó receba e propague blocos e transações pela rede P2P.

## Objetivos

*   Receber novas transações de peers e adicioná-las ao mempool local.
*   Receber novos blocos de peers, validá-los e adicioná-los à blockchain local.
*   Propagar transações recebidas (ou criadas localmente) para outros peers.
*   Propagar blocos minerados localmente para outros peers.

## Estrutura Proposta

*   **Mensagens de Rede:** Definir estruturas serializáveis (ex: usando `serde` e `bincode` ou `prost` para Protobuf) para representar mensagens de rede, como `NetworkMessage::NewTransaction(Transaction)` e `NetworkMessage::NewBlock(Block)`.
*   **Tópicos Gossipsub:** Utilizar tópicos distintos no Gossipsub para blocos e transações (ex: `blocks-topic`, `transactions-topic`).
*   **Gerenciamento de Eventos:** Modificar o loop de eventos do `libp2p` no módulo `network` (ou em um novo módulo `node` que coordene tudo) para lidar com:
    *   Eventos de recebimento de mensagens Gossipsub.
    *   Eventos gerados internamente (ex: bloco minerado localmente, nova transação local).
*   **Interação com `Blockchain`:** O manipulador de eventos de rede chamará os métodos apropriados da struct `Blockchain` (ex: `add_pending_transaction`, `process_mined_block`).

## Tarefas de Implementação

1.  **Definir Mensagens de Rede:**
    *   **Tarefa:** Criar um enum `NetworkMessage` (provavelmente em `network/protocols.rs` ou similar) com variantes para `NewTransaction` e `NewBlock`.
    *   **Tarefa:** Derivar `Serialize` e `Deserialize` (usando `serde` e `bincode`, por exemplo) para `NetworkMessage`, `Block`, e `Transaction` (já feito para Block/Transaction).
2.  **Configurar Tópicos Gossipsub:**
    *   **Tarefa:** No setup do `libp2p` (`network/mod.rs`), definir e inscrever-se em dois `IdentTopic`: um para blocos e outro para transações.
3.  **Implementar Manipulador de Eventos de Rede:**
    *   **Tarefa:** No loop principal de eventos do `libp2p` (`network::run_event_loop` ou similar):
        *   Identificar eventos `GossipsubEvent::Message`.
        *   Deserializar a `message.data` para `NetworkMessage`.
        *   Com base na variante da mensagem e no tópico:
            *   Se `NewTransaction` no tópico de transações: Chamar `blockchain.add_pending_transaction(tx)`.
            *   Se `NewBlock` no tópico de blocos: Chamar `blockchain.process_mined_block(block)`.
        *   Adicionar tratamento de erros robusto (falha na deserialização, falha ao adicionar à blockchain/mempool).
    *   **Objetivo:** Processar corretamente blocos e transações recebidos da rede.
4.  **Implementar Propagação de Transações:**
    *   **Tarefa:** Modificar `Blockchain::add_pending_transaction` ou criar um novo ponto de entrada para que, após adicionar uma transação *válida* ao mempool local, ela seja serializada como `NetworkMessage::NewTransaction` e publicada no tópico Gossipsub de transações.
    *   **Tarefa:** *Consideração:* Evitar propagar transações que já recebemos da rede (pode ser complexo, talvez adiar otimizações de flooding).
    *   **Objetivo:** Compartilhar novas transações com a rede.
5.  **Implementar Propagação de Blocos:**
    *   **Tarefa:** Após um bloco ser minerado *localmente* com sucesso (`mine_new_block`) e *antes* ou *depois* de ser processado (`process_mined_block`), serializá-lo como `NetworkMessage::NewBlock` e publicá-lo no tópico Gossipsub de blocos.
    *   **Objetivo:** Compartilhar novos blocos minerados com a rede.
6.  **Refatorar Loop Principal/Nó:**
    *   **Tarefa:** Estruturar melhor o ponto de entrada principal (talvez em `src/main.rs` ou um novo `src/node.rs`) para inicializar a `Blockchain`, o `libp2p Swarm`, e executar o loop de eventos que lida tanto com eventos de rede quanto com tarefas internas (como iniciar a mineração periodicamente ou via comando).

## Próximos Passos (Pós-Integração Básica)

*   Implementar Request/Response para sincronização inicial (obter histórico de blocos de peers).
*   Melhorar a validação de transações antes de propagá-las.
*   Adicionar lógica para evitar loops de propagação (ex: manter registro de hashes vistos).
*   Implementar mecanismos de descoberta de peers mais robustos (ex: bootnodes).

