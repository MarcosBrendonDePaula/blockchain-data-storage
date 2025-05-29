# Rascunho Inicial: Seções do Whitepaper

## 3. Arquitetura da Rede

A arquitetura da [Nome da Blockchain] foi concebida para oferecer uma solução robusta e segura para o armazenamento descentralizado de dados, combinando uma camada de consenso baseada em blockchain com uma camada dedicada ao armazenamento físico dos dados. Esta abordagem híbrida visa equilibrar segurança, eficiência e escalabilidade.

### 3.1. Camada Blockchain (On-Chain)

A fundação da rede é uma blockchain que segue uma estrutura de dados linear e cronológica, onde cada bloco é imutavelmente ligado ao anterior através de hashes criptográficos. Esta camada on-chain é responsável por manter o registro canônico de todas as transações, incluindo transferências da moeda nativa e, crucialmente, os metadados que atestam a existência e a integridade dos dados armazenados na camada off-chain. Cada bloco contém um cabeçalho com informações essenciais como o timestamp, o hash do bloco precedente, a raiz de Merkle que sumariza as transações incluídas, e um nonce utilizado no processo de consenso Proof-of-Work. O corpo do bloco agrega a lista de transações validadas pelos mineradores.

O mecanismo de consenso que garante a integridade e a ordem das transações nesta camada é o Proof-of-Work (PoW). Seguindo este protocolo, mineradores competem para resolver um desafio computacional, buscando um nonce que, quando combinado com os dados do cabeçalho do bloco, resulta em um hash que satisfaz um determinado nível de dificuldade definido pela rede. O minerador que primeiro encontra uma solução válida propaga o bloco candidato para a rede. Após verificação e validação pelos demais nós participantes, o bloco é adicionado à cadeia, estendendo o registro histórico. Este modelo, consagrado por redes como Bitcoin, oferece alta segurança contra adulterações, desde que haja poder computacional suficiente e distribuído assegurando a rede. Contudo, reconhecemos os desafios inerentes ao PoW, nomeadamente o seu consumo energético e as limitações de escalabilidade em termos de transações por segundo. A escolha específica do algoritmo de hashing (por exemplo, SHA-256, Scrypt, ou um algoritmo customizado resistente a ASICs) será um ponto de definição técnica crucial, buscando otimizar a segurança e a descentralização da mineração. A segurança contra ataques de 51% será diretamente proporcional ao poder computacional agregado e honesto dedicado à rede, um fator crítico especialmente nas fases iniciais.

As funcionalidades de contrato inteligente nesta camada serão, inicialmente, focadas nas operações essenciais da rede, sem buscar compatibilidade direta com a EVM para simplificar o desenvolvimento e otimizar o desempenho para as tarefas principais. Isso inclui a transferência da moeda nativa, o processamento de transações específicas para registrar metadados de armazenamento (incluindo hash, tamanho, tipo de dado) e a execução da lógica associada ao pagamento das taxas correspondentes. Exploraremos também a implementação de mecanismos contratuais simples para permitir a verificação periódica da disponibilidade e integridade dos dados armazenados na camada off-chain, inspirados em conceitos como Proof-of-Storage, mas adaptados à nossa arquitetura.

### 3.2. Camada de Armazenamento (Off-Chain)

Reconhecendo a inviabilidade de armazenar grandes volumes de dados diretamente na blockchain PoW, a [Nome da Blockchain] adota uma arquitetura de armazenamento híbrida. Os dados brutos dos usuários (arquivos de texto, JSON, etc.) são processados e armazenados em uma camada distribuída separada, off-chain, composta por uma rede de nós de armazenamento dedicados. A blockchain (on-chain) atua como uma camada de controle e verificação, registrando apenas os metadados essenciais e as provas criptográficas relativas a esses dados.

O fluxo proposto para um usuário armazenar dados inicia-se no cliente: os dados são primeiramente criptografados localmente utilizando a chave privada do usuário, garantindo que apenas ele possa descriptografá-los posteriormente. Este blob de dados criptografados é então transmitido para a rede de nós de armazenamento off-chain. Estes nós, operados por participantes da rede incentivados economicamente (detalhes na seção de Modelo Econômico), são responsáveis por manter os dados disponíveis e íntegros. A transação correspondente, registrada na blockchain on-chain, conterá o identificador único dos dados, o hash criptográfico do blob de dados (permitindo a verificação de integridade a qualquer momento), metadados descritivos (tamanho, tipo, timestamp) e informações sobre as políticas de redundância ou localização dos dados na rede off-chain.

A garantia da disponibilidade e recuperabilidade dos dados nesta camada off-chain é um desafio central. Serão necessários mecanismos robustos de redundância, como a divisão dos dados criptografados em múltiplos fragmentos e sua distribuição por diversos nós de armazenamento independentes, utilizando técnicas como codificação de apagamento (erasure coding). Adicionalmente, mecanismos de prova periódica (semelhantes a PoSt ou outras formas de Proof-of-Storage) serão implementados para que os nós de armazenamento demonstrem continuamente que ainda possuem os dados pelos quais são responsáveis, condicionando seus incentivos econômicos a essa comprovação. A integração técnica e a comunicação segura entre a camada on-chain e a camada off-chain são vitais para o funcionamento coeso do sistema.

### 3.3. Segurança e Privacidade

A segurança e a privacidade dos dados do usuário são pilares fundamentais da arquitetura. A criptografia ponta-a-ponta, realizada no cliente antes que os dados deixem o controle do usuário, assegura a confidencialidade contra todas as outras partes, incluindo mineradores e operadores de nós de armazenamento. Apenas o detentor da chave privada correspondente pode acessar o conteúdo original. A integridade dos dados é garantida pelo hash criptográfico armazenado imutavelmente na blockchain on-chain; qualquer tentativa de alteração dos dados off-chain invalidaria a correspondência com o hash registrado.

A segurança geral da rede on-chain depende da força do algoritmo PoW escolhido e da distribuição do poder computacional entre os mineradores. A segurança da camada off-chain depende da robustez dos mecanismos de redundância, das provas de armazenamento e dos incentivos econômicos que garantem a participação honesta dos nós de armazenamento.

## 4. Modelo Econômico (Tokenomics)

O modelo econômico da [Nome da Blockchain] é projetado para criar um ecossistema autossustentável, alinhando os incentivos de todos os participantes – usuários, mineradores e provedores de armazenamento off-chain – através da moeda nativa da rede, denominada [Símbolo].

### 4.1. Moeda Nativa ([Símbolo])

A moeda [Símbolo] é o centro das interações econômicas na rede. Sua principal utilidade é servir como meio de troca para o pagamento de todas as taxas de transação, incluindo as taxas base e as taxas específicas de armazenamento. Além disso, [Símbolo] é a unidade de conta para as recompensas distribuídas aos mineradores que asseguram a camada de consenso PoW. Futuramente, dependendo do modelo adotado para a camada off-chain, [Símbolo] poderá também ser utilizada como colateral exigido dos nós de armazenamento ou como meio de pagamento direto entre usuários e provedores de armazenamento. O fornecimento total de [Símbolo] e sua distribuição inicial (por exemplo, através de pré-mineração, venda pública ou outros mecanismos) serão definidos em uma fase posterior, considerando a necessidade de financiar o desenvolvimento e incentivar a adoção inicial, ao mesmo tempo que se busca uma distribuição justa e descentralizada.

### 4.2. Taxas de Transação

Para compensar os mineradores pelo processamento e inclusão de transações nos blocos e para gerenciar o uso dos recursos da rede, um sistema de taxas é implementado. Toda transação incorre em uma **Taxa Base**, calculada proporcionalmente ao seu tamanho em bytes (`Taxa_Base = Custo_Por_Byte_Tx * Tamanho_Tx_em_Bytes`). Esta taxa cobre o custo de processamento e armazenamento da própria transação na blockchain.

Adicionalmente, transações que registram metadados para o armazenamento de dados off-chain pagam uma **Taxa de Armazenamento**, calculada proporcionalmente ao tamanho dos dados originais sendo referenciados (`Taxa_Armazenamento = Custo_Por_Byte_Dados * Tamanho_Dados_Originais_em_Bytes`). Este componente implementa o requisito central de que o custo do serviço de registro seja maior para volumes maiores de dados. Os parâmetros `Custo_Por_Byte_Tx` e `Custo_Por_Byte_Dados` são definidos pela rede e podem ser ajustáveis via mecanismos de governança.

Os usuários podem optar por pagar taxas totais superiores aos mínimos calculados para incentivar a inclusão mais rápida de suas transações pelos mineradores, especialmente em períodos de congestionamento da rede, criando assim um mercado de taxas dinâmico.

### 4.3. Recompensas aos Mineradores (PoW)

Os mineradores que dedicam poder computacional para resolver o desafio PoW e adicionar novos blocos válidos à cadeia são recompensados por seu trabalho essencial para a segurança e funcionamento da rede. A recompensa por bloco consiste em duas partes: um **Subsídio de Bloco**, que corresponde a uma quantidade fixa de [Símbolo] recém-criados, servindo como mecanismo primário de inflação controlada da moeda; e a **soma de todas as Taxas de Transação** (Base + Armazenamento) contidas no bloco minerado. O Subsídio de Bloco será programado para diminuir ao longo do tempo (através de eventos de halving periódicos) para garantir a escassez da moeda a longo prazo, fazendo com que as taxas de transação se tornem progressivamente a fonte dominante de receita para os mineradores, alinhando seus incentivos com a utilidade e o volume de transações da rede.

### 4.4. Incentivos para Armazenamento Off-Chain

Um ponto crítico para a viabilidade do projeto é o modelo econômico que sustenta a camada de armazenamento off-chain. As taxas on-chain descritas acima compensam os mineradores PoW pelo trabalho de validação e registro na blockchain, mas não remuneram diretamente os nós que efetivamente armazenam os dados off-chain de forma contínua. É imperativo definir um mecanismo robusto e sustentável para incentivar os operadores de nós de armazenamento a fornecerem espaço de disco, largura de banda e garantias de disponibilidade e durabilidade. Várias abordagens podem ser consideradas, inspiradas em modelos existentes como Sia ou Filecoin, e adaptadas à nossa arquitetura:

*   **Contratos Diretos:** Usuários poderiam estabelecer contratos diretos (facilitados por smart contracts na camada on-chain) com um conjunto de nós de armazenamento, pagando-lhes periodicamente em [Símbolo] pela duração do contrato, condicionado à apresentação de provas de armazenamento válidas.
*   **Pool de Recompensas:** Uma porção das taxas de armazenamento on-chain (ou uma parte do subsídio de bloco) poderia ser direcionada a um pool de recompensas distribuído aos nós de armazenamento ativos e que comprovam regularmente a posse dos dados através dos mecanismos de prova.
*   **Mercado Aberto:** Poderia existir um mercado mais dinâmico onde nós de armazenamento competem por preço e reputação para hospedar os dados dos usuários.

A definição precisa deste mecanismo é um ponto crucial a ser detalhado e validado, pois a sustentabilidade econômica desta camada é essencial para a funcionalidade principal da rede.

### 4.5. Mecanismo de "Take Profit"

(Esta seção será detalhada após recebermos mais informações sobre o significado e a funcionalidade desejada para o termo "Take Profit" no contexto específico deste projeto. Pode referir-se a mecanismos de staking, yield farming, estratégias de mineração, ou outras funcionalidades que precisam ser clarificadas.)


