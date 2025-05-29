# Proposta de Arquitetura: Blockchain para Armazenamento de Dados

## Introdução

Com base nos requisitos levantados e nas lições aprendidas com a análise de soluções existentes como Filecoin, Arweave e Sia, apresentamos uma proposta inicial para a arquitetura da blockchain personalizada focada em armazenamento de dados. Esta proposta busca equilibrar as funcionalidades desejadas pelo usuário com a viabilidade técnica e as melhores práticas observadas no mercado, mantendo a flexibilidade para futuras evoluções.

## Camada Base e Estrutura de Blocos

A fundação da rede será uma blockchain com estrutura tradicional, onde os blocos são encadeados cronologicamente através de hashes criptográficos. Cada bloco conterá um cabeçalho (com metadados como timestamp, hash do bloco anterior, nonce, raiz de Merkle das transações) e um corpo contendo a lista de transações validadas. Esta estrutura linear é bem compreendida e estabelecida, fornecendo uma base sólida para as funcionalidades da rede.

## Mecanismo de Consenso: Proof-of-Work (PoW)

Atendendo à solicitação inicial, o mecanismo de consenso adotado será o Proof-of-Work (PoW). Mineradores competirão para encontrar um valor (nonce) que, combinado com os dados do cabeçalho do bloco, produza um hash que atenda a um determinado critério de dificuldade. O primeiro minerador a encontrar uma solução válida transmite o bloco para a rede, e após validação pelos demais nós, o bloco é adicionado à cadeia. Este modelo, similar ao do Bitcoin e da Sia, incentiva a participação através da recompensa de bloco e taxas de transação, e sua segurança é dependente do poder computacional total da rede. A escolha específica do algoritmo de hashing (ex: SHA-256, Scrypt, Ethash ou um algoritmo customizado resistente a ASICs) deverá ser definida posteriormente, considerando o equilíbrio desejado entre segurança e acessibilidade da mineração.

## Tratamento e Armazenamento de Dados

Considerando a ineficiência e o custo de armazenar grandes volumes de dados diretamente na blockchain, especialmente em um modelo PoW, propomos uma abordagem híbrida. Os dados em si (arquivos de texto, JSON, etc.) serão armazenados fora da cadeia principal (off-chain), enquanto a blockchain registrará metadados essenciais e provas de existência.

O fluxo proposto para armazenamento seria: o usuário submete os dados desejados através de um cliente. Este cliente realiza a **criptografia dos dados localmente**, utilizando a chave privada do usuário. Apenas o proprietário da chave poderá descriptografar os dados posteriormente, garantindo a privacidade e o controle, como solicitado. O blob de dados criptografados seria então enviado para uma camada de armazenamento distribuído separada, composta por nós dedicados (semelhantes aos hosts da Sia ou SPs da Filecoin). A transação registrada na blockchain conteria informações cruciais: um identificador único para os dados, o hash do blob criptografado (para verificação de integridade), metadados relevantes (como tamanho do arquivo original, tipo, timestamp de criação), e potencialmente informações sobre a localização ou redundância na camada de armazenamento off-chain. A definição de limites máximos para o tamanho dos dados por transação será crucial para gerenciar a carga da rede e os custos associados, necessitando de análise mais aprofundada.

## Segurança e Privacidade

A segurança dos dados armazenados é primordial. Conforme a proposta de tratamento de dados, a criptografia ponta-a-ponta realizada no cliente antes da submissão à rede garante que nem os mineradores, nem os nós de armazenamento, nem outros usuários possam acessar o conteúdo original. A integridade é assegurada pelo hash registrado na blockchain. A segurança da rede em si dependerá da robustez do algoritmo PoW e do poder computacional agregado dos mineradores honestos.

## Funcionalidades de Contrato

Inicialmente, a capacidade de contratos inteligentes será focada nas operações essenciais da rede, sem a necessidade de compatibilidade com a EVM. As funcionalidades primárias incluirão: a transferência da moeda nativa da blockchain entre carteiras; a execução de transações específicas para registrar metadados de armazenamento e processar o pagamento das taxas correspondentes; e potencialmente, mecanismos simples para verificar a disponibilidade dos dados na camada off-chain (inspirado nas provas de armazenamento da Sia ou Filecoin, mas simplificado). Funcionalidades mais complexas podem ser avaliadas e adicionadas em fases futuras, conforme a evolução das necessidades.

## Modelo de Taxas e Recompensas

O modelo econômico seguirá a diretriz de taxas proporcionais ao tamanho dos dados. Haverá uma taxa base para transações simples (como transferências de moeda). Para transações de armazenamento de dados, uma taxa adicional será calculada com base no tamanho dos dados originais informados nos metadados. Ambas as taxas serão pagas na moeda nativa da blockchain. Os mineradores que validarem e adicionarem um novo bloco à cadeia serão recompensados com uma combinação de: uma recompensa de bloco fixa (subsídio de bloco, que pode diminuir ao longo do tempo para controlar a inflação) e a soma de todas as taxas de transação (base + armazenamento) incluídas no bloco. Isso alinha os incentivos dos mineradores com a segurança da rede e o processamento das operações de armazenamento.

## Questões em Aberto e Próximos Passos

Esta proposta inicial estabelece as bases da arquitetura. Pontos como a definição exata dos limites de dados, o algoritmo de hashing para o PoW, os detalhes da camada de armazenamento off-chain, o mecanismo exato de "Take Profit" (se diferente da mineração), e o modelo de governança da rede ainda precisam ser refinados. A próxima etapa envolverá detalhar o modelo de taxas e distribuição de recompensas, seguido por uma análise de viabilidade técnica e econômica mais aprofundada.

