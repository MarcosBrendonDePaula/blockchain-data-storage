# Definição do Modelo de Taxas e Distribuição de Recompensas

## Introdução

Este documento detalha o modelo proposto para as taxas de transação e a distribuição de recompensas na blockchain personalizada para armazenamento de dados. O objetivo é criar um sistema econômico sustentável que incentive a participação dos mineradores na validação de transações e na segurança da rede, ao mesmo tempo que implementa a cobrança de taxas proporcionais ao uso dos recursos de armazenamento, conforme os requisitos definidos.

## Estrutura das Taxas de Transação

As taxas de transação são um componente essencial para compensar os mineradores pelo trabalho de incluir transações nos blocos e para prevenir o spam na rede. Propomos uma estrutura de taxas composta por dois elementos principais:

1.  **Taxa Base de Transação:** Toda transação submetida à rede incorrerá em uma taxa base. Esta taxa será calculada com base no tamanho da transação em bytes, refletindo o custo de processamento e armazenamento dos dados da própria transação na blockchain. Uma fórmula simples seria `Taxa_Base = Custo_Por_Byte_Tx * Tamanho_Tx_em_Bytes`. O `Custo_Por_Byte_Tx` seria um parâmetro da rede, potencialmente ajustável via governança no futuro. Isso se aplica a todas as transações, incluindo simples transferências da moeda nativa.

2.  **Taxa de Armazenamento de Dados:** Para transações que especificamente registram metadados referentes ao armazenamento de dados off-chain, haverá uma taxa adicional. Esta taxa será diretamente proporcional ao tamanho dos dados originais que estão sendo armazenados, conforme declarado nos metadados da transação. A fórmula seria `Taxa_Armazenamento = Custo_Por_Byte_Dados * Tamanho_Dados_Originais_em_Bytes`. O `Custo_Por_Byte_Dados` seria outro parâmetro fundamental da rede, refletindo o valor atribuído ao registro da existência e integridade desses dados na blockchain. Este mecanismo implementa diretamente o requisito de que o custo aumente com o volume de dados guardados.

O **Custo Total da Transação** para um usuário que armazena dados será a soma da Taxa Base e da Taxa de Armazenamento. Para transações que não envolvem armazenamento (como transferências simples), apenas a Taxa Base será aplicada.

## Mecanismo de Priorização e Mercado de Taxas

Embora a rede defina custos mínimos por byte (para Taxa Base e Taxa de Armazenamento), os usuários terão a opção de oferecer taxas totais mais altas para suas transações. Os mineradores, ao construírem um novo bloco, são economicamente incentivados a priorizar transações que oferecem taxas mais elevadas, pois isso maximiza sua recompensa. Isso cria um mercado de taxas dinâmico, onde transações urgentes podem ser aceleradas mediante o pagamento de uma taxa maior, especialmente em períodos de alta demanda da rede.

## Distribuição de Recompensas aos Mineradores

No contexto do consenso Proof-of-Work (PoW), os mineradores são os responsáveis por validar transações, agrupá-las em blocos e adicionar esses blocos à cadeia, consumindo poder computacional no processo. Para compensar esse esforço e garantir a segurança da rede, os mineradores que conseguirem minerar um bloco com sucesso receberão recompensas compostas por duas fontes:

1.  **Subsídio de Bloco (Block Subsidy):** Uma quantidade pré-definida de moedas nativas recém-criadas será concedida ao minerador do bloco. Este subsídio serve como a principal forma de introduzir novas moedas na economia da rede. Seguindo práticas comuns em outras blockchains PoW, este subsídio pode ser programado para diminuir em intervalos regulares (eventos de halving) para controlar a inflação a longo prazo.

2.  **Taxas de Transação:** O minerador que minerou o bloco coletará a totalidade das taxas (Taxa Base + Taxa de Armazenamento, quando aplicável) de todas as transações incluídas naquele bloco específico.

A **Recompensa Total do Minerador** por bloco será, portanto, a soma do Subsídio de Bloco e o total das Taxas de Transação contidas no bloco minerado.

## Considerações Adicionais

É importante notar que este modelo de taxas cobre os custos associados ao registro e validação na *blockchain* (on-chain). Os custos relacionados ao armazenamento físico dos dados na camada distribuída *off-chain* não estão diretamente cobertos por esta taxa on-chain. Um mecanismo separado, possivelmente envolvendo contratos diretos ou um sistema de incentivos específico para os nós de armazenamento off-chain, precisará ser detalhado para garantir a sustentabilidade dessa camada, inspirado em modelos como os da Sia ou Filecoin, mas adaptado à nossa arquitetura.

## Conclusão

O modelo de taxas e recompensas proposto visa criar um ciclo econômico virtuoso, onde os usuários pagam pelo uso dos recursos da rede (incluindo o registro de armazenamento proporcional ao tamanho), e os mineradores são devidamente compensados por fornecerem a segurança e a capacidade de processamento necessárias. A definição exata dos parâmetros (custos por byte, subsídio inicial, cronograma de halving) exigirá análise cuidadosa e modelagem econômica para garantir o equilíbrio e a viabilidade a longo prazo da blockchain. Esta definição servirá de base para a validação técnica e econômica subsequente.

