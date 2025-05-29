# Definição de Tarefas Incrementais Iniciais

## Introdução

Com base no escopo detalhado dos módulos (`core`, `consensus`, `network`, `storage`), definimos um conjunto inicial de tarefas incrementais para guiar o desenvolvimento do núcleo da blockchain `blockchain-data-storage`. Estas tarefas são projetadas para serem gerenciáveis e fornecerem marcos progressivos na construção da funcionalidade base.

## Tarefas do Módulo `core`

O foco inicial no módulo `core` será solidificar as estruturas de dados e suas operações fundamentais.

1.  **Serialização/Deserialização:** Implementar mecanismos robustos de serialização e deserialização para as estruturas `Block`, `BlockHeader` e `Transaction`. A biblioteca `serde` é uma escolha padrão e recomendada no ecossistema Rust para esta finalidade, garantindo compatibilidade e eficiência para armazenamento e transmissão.
2.  **Cálculo de Hash:** Substituir os cálculos de hash placeholder por implementações reais utilizando uma biblioteca criptográfica padrão (como `sha2` ou `blake3`). Garantir que os hashes sejam calculados de forma consistente sobre os dados serializados das estruturas.
3.  **Árvore de Merkle:** Implementar a lógica para construir a árvore de Merkle a partir da lista de transações de um bloco e para verificar a inclusão de uma transação na árvore utilizando uma prova de Merkle. Isso é essencial para a validação eficiente de blocos.
4.  **Validação Básica:** Refinar as funções de validação inicial para transações e blocos dentro do módulo `core`, verificando a consistência interna das estruturas e regras básicas (ex: formato, timestamps lógicos), independentemente do estado global ou consenso.

## Tarefas do Módulo `consensus`

As tarefas iniciais do módulo `consensus` se concentrarão na implementação da mecânica central do Proof-of-Work.

1.  **Seleção e Implementação do Hashing PoW:** Escolher e implementar o algoritmo de hashing específico que será usado para o Proof-of-Work. Considerar opções como SHA-256, Scrypt, ou algoritmos resistentes a ASIC, e integrar a biblioteca correspondente.
2.  **Verificação de Dificuldade:** Implementar a função que compara o hash de um cabeçalho de bloco com o alvo de dificuldade atual da rede para determinar se o PoW é válido.
3.  **Loop de Mineração Básico:** Implementar o loop fundamental da mineração, onde o `nonce` no cabeçalho do bloco é incrementado repetidamente, e o hash é recalculado até que um hash válido (que atenda à dificuldade) seja encontrado.
4.  **Algoritmo de Ajuste de Dificuldade:** Implementar a lógica para recalcular periodicamente a dificuldade da rede com base no tempo médio de mineração dos blocos recentes, visando manter um intervalo de bloco alvo estável.

## Tarefas do Módulo `network`

O desenvolvimento inicial do módulo `network` focará em estabelecer a comunicação P2P básica.

1.  **Configuração da Biblioteca P2P:** Escolher e configurar uma biblioteca de rede P2P. `libp2p` é uma opção poderosa e flexível, amplamente utilizada em projetos blockchain, que abstrai muitas das complexidades da comunicação distribuída.
2.  **Descoberta de Peers:** Implementar mecanismos básicos para que um nó possa descobrir e conectar-se a outros nós na rede (por exemplo, usando nós de bootstrap ou protocolos de descoberta como Kademlia DHT, se usar `libp2p`).
3.  **Definição e Troca de Mensagens Iniciais:** Definir os formatos para mensagens essenciais (usando, por exemplo, Protocol Buffers ou `serde` com `bincode`) como solicitação/envio de blocos (`GetBlocks`/`Blocks`), anúncio/envio de novas transações (`NewTransaction`/`TransactionData`), e implementar a lógica básica para enviar e receber essas mensagens.
4.  **Propagação Básica:** Implementar a funcionalidade inicial para que um nó possa retransmitir transações recebidas e blocos recém-minerados ou validados para seus peers conectados.

## Tarefas do Módulo `storage`

As tarefas iniciais do módulo `storage` se concentrarão na persistência local da blockchain.

1.  **Seleção do Mecanismo de Armazenamento:** Escolher uma solução para armazenar os dados da blockchain no disco. Pode-se começar com um formato simples baseado em arquivos ou adotar um banco de dados chave-valor embarcado como `RocksDB` ou `sled` para melhor desempenho e gerenciamento.
2.  **Persistência de Blocos:** Implementar as funções para serializar e salvar blocos no mecanismo de armazenamento escolhido e para carregar blocos do disco.
3.  **Acesso a Blocos:** Implementar funções para recuperar blocos específicos (por altura ou hash) e para obter informações como o último bloco da cadeia armazenada.

## Próximos Passos

Estas tarefas iniciais fornecem um roteiro para começar a construir os componentes fundamentais da blockchain. A próxima etapa seria refinar essas tarefas, criar issues correspondentes no GitHub para rastreamento e começar a implementação, possivelmente priorizando as tarefas do módulo `core` que servem de base para os demais.

