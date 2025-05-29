# Requisitos Preliminares: Blockchain para Armazenamento de Dados

## Introdução

Este documento descreve os requisitos iniciais para o desenvolvimento de uma nova blockchain especializada em armazenamento de dados. A visão geral do projeto envolve a criação de uma rede descentralizada que permita aos usuários não apenas realizar transações financeiras com uma moeda nativa, mas também armazenar dados de forma segura e permanente. Um aspecto central do modelo econômico proposto é a implementação de taxas de transação variáveis, calculadas com base no tamanho dos dados a serem armazenados, com uma parcela dessas taxas sendo distribuída aos participantes responsáveis pela validação e manutenção da rede.

## Funcionalidades Essenciais

A blockchain deverá suportar as funcionalidades padrão esperadas de uma rede distribuída, incluindo:

*   **Transferência de Moeda Nativa:** Permitir a transferência de valor entre carteiras na rede utilizando uma moeda digital própria.
*   **Armazenamento de Dados:** Oferecer a capacidade de registrar dados na blockchain. Os dados primariamente suportados serão em formato de texto, incluindo estruturas como JSON. A intenção é permitir o armazenamento de informações diversas, desde metadados até conteúdos textuais mais extensos.
*   **Criação e Gerenciamento de Contratos:** Possibilitar a implementação de lógica programável na rede, embora os detalhes específicos e a complexidade desses contratos ainda precisem ser definidos. Inicialmente, não há requisito de compatibilidade com a Ethereum Virtual Machine (EVM).

## Mecanismo de Consenso

Para garantir a integridade e a segurança da rede, o mecanismo de consenso adotado será o **Proof-of-Work (PoW)**. Neste modelo, os participantes da rede, conhecidos como mineradores (ou validadores, no contexto do PoW), competem para resolver problemas computacionais complexos. O primeiro a encontrar a solução válida tem o direito de adicionar o próximo bloco de transações (e dados) à cadeia, sendo recompensado por seu esforço.

## Modelo de Taxas e Incentivos

As transações na rede, especialmente aquelas que envolvem o armazenamento de dados, estarão sujeitas a taxas. A característica distintiva deste modelo é que o valor da taxa será **proporcional ao tamanho dos dados** que estão sendo registrados na blockchain. Quanto maior o volume de dados, maior será a taxa cobrada.

Uma porcentagem dessas taxas coletadas será direcionada aos **mineradores/validadores** da rede, como forma de incentivo pela sua contribuição na validação das transações, na adição de novos blocos e na manutenção da segurança e operacionalidade da blockchain.

## Segurança e Privacidade dos Dados

Um requisito fundamental é a **criptografia dos dados** armazenados na blockchain. Os dados submetidos para armazenamento deverão ser criptografados de forma que apenas o proprietário original dos dados, de posse da chave privada correspondente, possa descriptografá-los e acessar seu conteúdo. Isso garante a confidencialidade e o controle do usuário sobre suas informações.

## Pontos a Detalhar

Diversos aspectos ainda necessitam de maior detalhamento para a completa especificação do projeto:

*   **Limites de Dados:** Qual seria o tamanho máximo de dados permitido por transação ou por bloco? Definir esses limites é crucial para o design da rede e do modelo de taxas.
*   **Funcionalidade de Contratos Inteligentes:** Além das transferências e armazenamento, que outras operações ou lógicas os contratos inteligentes deveriam suportar?
*   **Desempenho e Escalabilidade:** Qual a expectativa de volume de transações por segundo (TPS) e de capacidade total de armazenamento que a rede deve almejar?
*   **Mecanismo de "Take Profit":** É necessário clarificar o conceito de "Take Profit" mencionado inicialmente. Refere-se a um mecanismo específico dentro da blockchain, a estratégias de investimento na moeda, ou a outro aspecto?
*   **Governança:** Como serão tomadas as decisões sobre futuras atualizações, mudanças nas regras da rede ou ajustes no modelo econômico?

