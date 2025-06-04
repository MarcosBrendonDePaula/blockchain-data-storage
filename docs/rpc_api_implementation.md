# Implementação de Endpoints RPC para o Frontend

## Endpoints Existentes
- `send_transaction` - Envio de transações (transferência e armazenamento)
- `get_chain_height` - Altura atual da blockchain
- `get_block_by_height` - Consulta blocos por altura
- `get_block_by_hash` - Consulta blocos por hash
- `get_offchain_data` - Recupera dados armazenados off-chain

## Endpoints a Implementar
- `get_balance` - Consulta o saldo de um endereço específico
- `create_token` - Cria moedas/tokens customizados

## Estruturas de Dados Necessárias
- `GetBalanceParams` - Parâmetros para consulta de saldo (endereço)
- `CreateTokenParams` - Parâmetros para criação de token (nome, símbolo, suprimento inicial)

## Implementação
1. Adicionar estruturas de parâmetros no arquivo `src/rpc.rs`
2. Implementar handlers para os novos endpoints
3. Adicionar os novos handlers ao match do `rpc_handler`
4. Implementar funções auxiliares no módulo core para suportar as novas operações
