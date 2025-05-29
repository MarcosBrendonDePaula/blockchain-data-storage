# Pesquisa sobre Blockchains Especializadas em Armazenamento de Dados

## Introdução

Com o objetivo de embasar a criação de uma nova blockchain focada em armazenamento de dados, realizamos uma pesquisa sobre soluções existentes que abordam desafios semelhantes. Analisamos três projetos proeminentes neste espaço: Filecoin, Arweave e Sia. Cada um apresenta abordagens distintas em termos de arquitetura, mecanismos de consenso, modelos econômicos e garantias de armazenamento, oferecendo insights valiosos para o desenvolvimento da nossa própria solução.

## Filecoin (FIL)

Filecoin, desenvolvido pela Protocol Labs (a mesma equipe por trás do IPFS), posiciona-se como uma rede de armazenamento descentralizada que cria um mercado global peer-to-peer para compra e venda de espaço de armazenamento digital. Sua proposta é oferecer uma alternativa mais eficiente e econômica aos serviços de nuvem centralizados.

Tecnicamente, Filecoin utiliza uma combinação de provas criptográficas para garantir a integridade e a disponibilidade dos dados. A **Proof-of-Replication (PoRep)** demonstra que um minerador de armazenamento está armazenando uma cópia única dos dados, enquanto a **Proof-of-Spacetime (PoSt)** prova que esses dados foram armazenados continuamente ao longo do tempo. O consenso principal é denominado **Storage Power Consensus (SPC)**, que pondera o poder de voto de um minerador com base na quantidade de armazenamento ativo e comprovado que ele contribui para a rede (Quality-Adjusted Power). Este poder considera o tamanho do setor, a duração do armazenamento prometido e o tipo de acordo (Committed Capacity, Regular Deal ou Verified Client Deal). A rede distingue entre mineradores de armazenamento (que fornecem espaço) e mineradores de blocos (que validam transações e estendem a blockchain), embora a eleição de líderes para minerar blocos seja baseada no poder de armazenamento, incentivando mineradores de armazenamento a também participarem da mineração de blocos. Filecoin também integra o **drand**, uma rede de aleatoriedade distribuída, para fornecer sementes de aleatoriedade imparciais para processos como a eleição de líderes e provas de armazenamento.

O modelo econômico gira em torno do token nativo, FIL. Usuários pagam taxas em FIL aos provedores de armazenamento (Storage Providers - SPs) para guardar seus dados e aos provedores de recuperação (Recovery Providers - RPs) para recuperá-los rapidamente. Os mineradores, por sua vez, são recompensados com FIL recém-criados (recompensas de bloco) e taxas de transação por fornecerem armazenamento, validarem transações e manterem a segurança da rede. Existem mecanismos de vesting para as recompensas dos mineradores, incentivando o comprometimento de longo prazo, e também mecanismos de queima de tokens para taxas de rede ou penalidades.

*Fontes Consultadas:*
*   *https://spec.filecoin.io/systems/filecoin_blockchain/storage_power_consensus/*
*   *https://crypto.com/pt/university/what-is-filecoin-fil*

## Arweave (AR)

Arweave adota uma abordagem focada na permanência dos dados, buscando oferecer uma solução para o armazenamento "eterno" de informações. Sua arquitetura central é o **blockweave**, uma estrutura de dados semelhante a uma blockchain, mas onde cada novo bloco se conecta não apenas ao bloco imediatamente anterior, mas também a um bloco aleatório do histórico da rede (o "recall block").

Essa estrutura permite o mecanismo de consenso **Proof-of-Access (PoA)**. Para minerar um novo bloco, os mineradores precisam provar que têm acesso a esse bloco de recall aleatório. Isso incentiva os mineradores a armazenar mais dados históricos da rede, embora não necessariamente a cadeia completa, distribuindo o ônus do armazenamento.

O modelo econômico da Arweave é singular: os usuários pagam uma **taxa única e inicial** em tokens AR para armazenar dados permanentemente. Essa taxa é calculada para cobrir não apenas os custos imediatos, mas também para contribuir para um **fundo de dotação (endowment)**. A premissa é que o custo do armazenamento de dados diminuirá ao longo do tempo devido aos avanços tecnológicos, e os rendimentos gerados por essa dotação serão suficientes para incentivar os mineradores a continuar armazenando os dados indefinidamente no futuro. Os mineradores são recompensados com AR provenientes dessas taxas e da dotação por armazenarem os dados e participarem do consenso. O fornecimento total de AR é fixo em 66 milhões de tokens.

*Fontes Consultadas:*
*   *https://crypto.com/pt/university/what-is-arweave-and-how-to-buy-ar*
*   *https://www.arweave.com.br/custo-do-armazenamento-perpetuo-de-dados/*

## Sia (SC)

Sia opera como um mercado de armazenamento em nuvem descentralizado, conectando usuários que precisam de espaço (locatários ou renters) com aqueles que têm capacidade de disco para oferecer (hospedeiros ou hosts). O foco principal da Sia é a privacidade e a acessibilidade, buscando ser significativamente mais barata que as soluções de nuvem tradicionais.

Quando um usuário armazena um arquivo na Sia, ele é primeiro **criptografado no lado do cliente**, garantindo que apenas o usuário com a chave possa acessá-lo. Em seguida, o arquivo é dividido em múltiplos segmentos, e cópias redundantes desses segmentos são distribuídas entre vários hosts na rede. Essa redundância elimina pontos únicos de falha e aumenta a disponibilidade. A relação entre locatários e hospedeiros é gerenciada por **contratos de arquivo (file contracts)** registrados na blockchain da Sia. Esses contratos definem os termos do armazenamento, incluindo duração e preço, e são garantidos por colaterais em Siacoin (SC) depositados pelos hosts. Os hosts devem periodicamente submeter **provas de armazenamento (proofs of storage)** à blockchain para demonstrar que continuam armazenando os dados corretamente e receber o pagamento.

A blockchain da Sia utiliza um mecanismo de consenso **Proof-of-Work (PoW)** para validar transações e proteger a rede. A moeda nativa, Siacoin (SC), é usada para todas as transações dentro do ecossistema: locatários pagam SC aos hospedeiros por armazenamento e largura de banda, e os hospedeiros usam SC como colateral e recebem SC como pagamento. A Sia enfatiza sua natureza de código aberto e sua API modular para facilitar a integração com outras aplicações.

*Fontes Consultadas:*
*   *https://sia.tech/*
*   *https://www.securities.io/pt/investing-in-siacoin/*
*   *https://learn.bybit.com/pt-pt/altcoins/what-is-siacoin/*

## Conclusão Preliminar

Filecoin, Arweave e Sia demonstram a diversidade de abordagens possíveis para o armazenamento descentralizado baseado em blockchain. Filecoin foca em um mercado dinâmico com provas complexas de armazenamento. Arweave aposta na permanência através de um modelo de pagamento único e dotação. Sia prioriza a privacidade e o baixo custo com criptografia do lado do cliente e contratos inteligentes. Essas referências serão fundamentais para definir a arquitetura, o mecanismo de consenso e o modelo de taxas da nossa blockchain personalizada, buscando combinar os pontos fortes observados e adaptá-los aos requisitos específicos do nosso projeto.

