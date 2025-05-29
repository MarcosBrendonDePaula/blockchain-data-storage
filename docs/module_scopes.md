# Detalhamento do Escopo dos Módulos Rust

## Introdução

Este documento descreve o escopo inicial e as responsabilidades propostas para os principais módulos do núcleo da blockchain `blockchain-data-storage`, implementada em Rust. A organização modular visa promover a clareza, a manutenibilidade e a escalabilidade do código, permitindo um desenvolvimento mais estruturado e facilitando a colaboração.

## Módulo `core`

O módulo `core` servirá como o coração da lógica fundamental da blockchain, contendo as definições das estruturas de dados primárias e as regras de validação básicas que são independentes do mecanismo de consenso específico ou da camada de rede. Sua principal responsabilidade é definir o estado e as transições de estado válidas da blockchain.

Componentes chave incluirão as estruturas `Block`, `BlockHeader`, e `Transaction`, conforme já esboçado em `core/mod.rs`. Este módulo será responsável por definir a serialização e deserialização dessas estruturas para armazenamento e transmissão pela rede. Também conterá a lógica para calcular hashes de transações e blocos, bem como a construção e verificação de árvores de Merkle para garantir a integridade das transações dentro de um bloco. Funções de validação de transações (verificação de formato, talvez limites de valores, mas sem checar gasto duplo, que depende do estado geral) e validação de blocos (verificação da estrutura, timestamp, consistência interna) residirão aqui. A estrutura `Blockchain` inicial, que gerencia a cadeia de blocos em memória e as transações pendentes, também pertence a este módulo, embora sua interação com o armazenamento persistente seja coordenada pelo módulo `storage`.

## Módulo `consensus`

O módulo `consensus` será dedicado à implementação das regras que permitem aos nós da rede chegarem a um acordo sobre o estado válido da blockchain. Dado que escolhemos Proof-of-Work (PoW), este módulo conterá toda a lógica específica do PoW.

Isso inclui a implementação do algoritmo de hashing selecionado para a mineração, a função que verifica se um hash de bloco atende à dificuldade atual da rede, e a lógica para ajustar a dificuldade periodicamente com base no tempo médio de geração dos blocos anteriores, visando manter um intervalo de bloco alvo. O processo de mineração em si, onde um nó tenta encontrar um nonce válido para um novo bloco, será orquestrado aqui. Este módulo também definirá as regras para seleção da cadeia canônica em caso de forks (geralmente, a cadeia com maior trabalho acumulado/mais longa no PoW). Ele interagirá com o módulo `core` para obter os dados do bloco a ser minerado e para validar o PoW de blocos recebidos.

## Módulo `network`

O módulo `network` será responsável por toda a comunicação peer-to-peer (P2P) entre os nós da blockchain. Sua função é permitir que os nós descubram uns aos outros, sincronizem o estado da blockchain e propaguem novas transações e blocos pela rede.

Implementará o protocolo de rede P2P, incluindo mecanismos para descoberta de peers (vizinhos na rede), estabelecimento e manutenção de conexões. Definirá os formatos das mensagens trocadas entre os nós (por exemplo, solicitação de blocos, envio de transações, anúncio de novos blocos). Gerenciará a lógica de sincronização da cadeia, permitindo que um novo nó (ou um nó que esteve offline) obtenha o histórico completo e validado da blockchain a partir de seus peers. Também será responsável por transmitir eficientemente novas transações para o mempool da rede e novos blocos válidos assim que forem minerados ou recebidos. Este módulo utilizará bibliotecas Rust de rede assíncrona (como `tokio` ou `async-std`) e possivelmente frameworks P2P (como `libp2p`) para lidar com a complexidade da comunicação distribuída.

## Módulo `storage`

O módulo `storage` cuidará da persistência dos dados da blockchain no disco local do nó e, futuramente, da interação com a camada de armazenamento off-chain distribuída.

Inicialmente, sua responsabilidade será gerenciar o armazenamento eficiente da cadeia de blocos (a sequência de `Block`s) em um banco de dados local ou sistema de arquivos. Isso inclui a capacidade de ler blocos específicos, obter o último bloco, e armazenar novos blocos de forma segura. Também pode gerenciar o armazenamento do estado atual da blockchain (por exemplo, o conjunto UTXO em modelos como o Bitcoin, ou saldos de contas) para permitir a validação eficiente de novas transações (como a prevenção de gasto duplo). Futuramente, este módulo será expandido para interagir com a rede de nós de armazenamento off-chain, gerenciando o envio de dados criptografados para esses nós, o recebimento de provas de armazenamento e a recuperação de dados quando solicitado, coordenando com as informações registradas na camada on-chain (via módulo `core`).

## Conclusão

Esta divisão modular fornece uma base sólida para o desenvolvimento. O módulo `core` define *o quê* é a blockchain, `consensus` define *como* os nós concordam sobre ela, `network` define *como* os nós se comunicam, e `storage` define *onde* a informação é mantida. Essa separação de responsabilidades facilitará a implementação, teste e evolução de cada componente de forma mais independente.

