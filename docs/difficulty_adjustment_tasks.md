# Detalhamento de Tarefas - Algoritmo de Ajuste de Dificuldade

## Introdução

Este documento detalha as tarefas para implementar o algoritmo de ajuste de dificuldade no módulo `consensus`. O objetivo é manter o tempo médio de geração de blocos próximo a um valor alvo, ajustando a dificuldade do Proof-of-Work (PoW) dinamicamente.

## Parâmetros do Algoritmo

*   **`TARGET_BLOCK_TIME_SECS`**: Tempo alvo para a geração de um bloco (ex: 600 segundos = 10 minutos).
*   **`ADJUSTMENT_INTERVAL_BLOCKS`**: Número de blocos após o qual a dificuldade será recalculada (ex: 2016 blocos, similar ao Bitcoin).
*   **`MIN_DIFFICULTY`**: Um valor mínimo para a dificuldade (evita que a dificuldade caia para zero).
*   **`MAX_DIFFICULTY_CHANGE_FACTOR`**: Limite para o quanto a dificuldade pode aumentar ou diminuir em um único ajuste (ex: fator 4, para evitar oscilações extremas).

## Lógica do Algoritmo

1.  **Verificação do Intervalo:** A cada bloco, verificar se a altura do bloco atual é um múltiplo de `ADJUSTMENT_INTERVAL_BLOCKS` (ou se é o bloco `ADJUSTMENT_INTERVAL_BLOCKS`, `2 * ADJUSTMENT_INTERVAL_BLOCKS`, etc.). O ajuste só ocorre nesses blocos.
2.  **Cálculo do Tempo Real:** Se for um bloco de ajuste, obter o timestamp do bloco atual e o timestamp do bloco `ADJUSTMENT_INTERVAL_BLOCKS` blocos atrás (bloco `altura_atual - ADJUSTMENT_INTERVAL_BLOCKS`). Calcular o tempo real gasto para minerar esses `ADJUSTMENT_INTERVAL_BLOCKS` blocos.
3.  **Cálculo do Tempo Esperado:** Calcular o tempo esperado para minerar o intervalo: `ADJUSTMENT_INTERVAL_BLOCKS * TARGET_BLOCK_TIME_SECS`.
4.  **Cálculo do Fator de Ajuste:** Calcular a razão entre o tempo esperado e o tempo real: `fator = tempo_esperado / tempo_real`.
5.  **Limitação do Fator:** Limitar o `fator` para que não seja maior que `MAX_DIFFICULTY_CHANGE_FACTOR` nem menor que `1 / MAX_DIFFICULTY_CHANGE_FACTOR`.
6.  **Cálculo da Nova Dificuldade:** Multiplicar a dificuldade *anterior* pelo `fator` limitado. A dificuldade aqui pode ser representada como o número de bits zero ou como um valor numérico de "target" (inversamente proporcional à dificuldade). Se usarmos bits zero, um fator > 1 significa *aumentar* a dificuldade (mais bits zero), e um fator < 1 significa *diminuir* a dificuldade (menos bits zero). Precisamos definir como a "dificuldade" é representada (bits ou target) e ajustar a multiplicação/divisão de acordo.
    *   *Assumindo dificuldade como número de bits zero:* `nova_dificuldade_float = dificuldade_anterior * fator`. Arredondar para o `u32` mais próximo.
    *   *Assumindo dificuldade como target numérico (menor = mais difícil):* `novo_target = target_anterior / fator`. (Requer representação BigInt/U256).
    *   **Vamos começar com a representação de bits zero (u32) por simplicidade.**
7.  **Aplicação da Nova Dificuldade:** A nova dificuldade calculada será usada para a mineração dos próximos `ADJUSTMENT_INTERVAL_BLOCKS` blocos.
8.  **Persistência:** A dificuldade atual precisa ser armazenada no cabeçalho do bloco para que os nós possam verificá-la e usá-la para calcular o próximo ajuste.

## Tarefas de Implementação

1.  **Definir Constantes:** Adicionar as constantes (`TARGET_BLOCK_TIME_SECS`, `ADJUSTMENT_INTERVAL_BLOCKS`, etc.) ao módulo `consensus` ou a um módulo de configuração.
2.  **Modificar `BlockHeader`:** Garantir que o `BlockHeader` armazene a `difficulty` (já está lá, mas confirmar se é a dificuldade *usada* para minerar *este* bloco).
3.  **Implementar `calculate_next_difficulty`:** Criar uma função `calculate_next_difficulty(current_height: u64, storage: &StorageManager) -> Result<u32, Error>` (ou similar) que:
    *   Verifica se `current_height + 1` é um bloco de ajuste.
    *   Se não for, retorna a dificuldade do bloco atual (obtida do storage).
    *   Se for, busca os timestamps necessários do `storage` (bloco `current_height` e bloco `current_height - ADJUSTMENT_INTERVAL_BLOCKS + 1`).
    *   Calcula o tempo real e o tempo esperado.
    *   Calcula e limita o fator de ajuste.
    *   Calcula a nova dificuldade (em bits zero) a partir da dificuldade do bloco `current_height`.
    *   Aplica limites (`MIN_DIFFICULTY`, etc.).
    *   Retorna a nova dificuldade calculada.
4.  **Integrar com Mineração/Validação:**
    *   Ao iniciar a mineração de um novo bloco, chamar `calculate_next_difficulty` para determinar a dificuldade a ser usada.
    *   Ao validar um bloco recebido, verificar se a dificuldade armazenada no cabeçalho corresponde à dificuldade que *deveria* ter sido calculada para aquele bloco com base nos blocos anteriores.
5.  **Atualizar `StorageManager` (se necessário):** Garantir que seja possível buscar blocos/cabeçalhos por altura de forma eficiente para obter os timestamps.

## Próximos Passos Após Tarefas Iniciais

*   Refinar a representação da dificuldade (talvez mudar para target U256 para maior precisão).
*   Adicionar mais testes de borda e cenários complexos.
*   Otimizar o acesso ao storage para buscar timestamps.

