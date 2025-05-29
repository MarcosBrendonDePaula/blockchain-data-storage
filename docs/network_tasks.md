# Detalhamento de Tarefas Iniciais - Módulo Network (P2P)

## Introdução

Este documento detalha as tarefas iniciais para o desenvolvimento do módulo `network`, responsável pela comunicação peer-to-peer (P2P) da blockchain `blockchain-data-storage`. O objetivo é estabelecer a base para que os nós possam se conectar, descobrir uns aos outros e trocar informações essenciais como blocos e transações.

## Tarefas Iniciais

1.  **Seleção e Configuração da Biblioteca P2P:**
    *   **Tarefa:** Adicionar a dependência `libp2p` ao `Cargo.toml`.
    *   **Tarefa:** Configurar a estrutura básica do `libp2p` no módulo `network`, incluindo a criação de um `Transport` (provavelmente TCP + Yamux/Mplex), um `NetworkBehaviour` customizado e um `Swarm`.
    *   **Objetivo:** Ter a infraestrutura básica do `libp2p` pronta para ser utilizada.

2.  **Identidade do Nó e Geração de Chaves:**
    *   **Tarefa:** Implementar a geração ou carregamento de um par de chaves Ed25519 para identificar unicamente cada nó na rede (`PeerId`).
    *   **Tarefa:** Garantir que a identidade do nó seja persistida localmente para que não mude a cada reinicialização.
    *   **Objetivo:** Cada nó ter uma identidade P2P estável.

3.  **Descoberta de Peers (Kademlia DHT):**
    *   **Tarefa:** Integrar o `libp2p-kad` (Kademlia DHT) ao `NetworkBehaviour`.
    *   **Tarefa:** Configurar nós de bootstrap (bootstrap nodes) iniciais para permitir que novos nós se conectem à rede DHT.
    *   **Tarefa:** Implementar a lógica para que um nó anuncie sua presença na DHT e busque por outros peers.
    *   **Objetivo:** Nós serem capazes de encontrar outros nós na rede de forma descentralizada.

4.  **Definição de Protocolos de Comunicação (Request-Response / Gossipsub):**
    *   **Tarefa:** Definir os formatos das mensagens iniciais usando `serde` e um formato binário (como `bincode` ou protobufs via `prost`). Mensagens iniciais podem incluir: `Ping`, `GetBlocksRequest`, `BlocksResponse`, `NewTransaction`.
    *   **Tarefa:** Avaliar e escolher protocolos `libp2p` adequados para diferentes tipos de comunicação:
        *   `libp2p-request-response` para solicitações diretas (ex: pedir blocos específicos).
        *   `libp2p-gossipsub` para propagação eficiente de mensagens para múltiplos peers (ex: anunciar novas transações ou blocos).
    *   **Tarefa:** Integrar os protocolos escolhidos ao `NetworkBehaviour`.
    *   **Objetivo:** Ter os canais de comunicação definidos para a troca de informações específicas da blockchain.

5.  **Loop de Eventos do Swarm:**
    *   **Tarefa:** Implementar o loop principal que escuta por eventos do `Swarm` (novas conexões, peers descobertos, mensagens recebidas, etc.) e os processa.
    *   **Tarefa:** Integrar este loop com o restante da aplicação (por exemplo, usando canais `tokio::sync::mpsc` para passar informações entre a camada de rede e os módulos `core` e `consensus`).
    *   **Objetivo:** Ter a camada de rede funcional, capaz de reagir a eventos P2P.

## Próximos Passos Após Tarefas Iniciais

Após a conclusão destas tarefas iniciais, o módulo `network` terá a capacidade básica de conectar nós e trocar mensagens simples. Os próximos passos envolverão:

*   Implementar a lógica específica para solicitar e enviar blocos para sincronização.
*   Implementar a lógica para propagar novas transações e blocos minerados usando Gossipsub.
*   Adicionar tratamento de erros e resiliência à rede.
*   Integrar mais profundamente com os módulos `core`, `consensus` e `storage`.

