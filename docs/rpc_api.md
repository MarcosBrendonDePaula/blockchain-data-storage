# Documentação da API JSON-RPC

Esta documentação descreve a interface JSON-RPC 2.0 sobre HTTP para interagir com o nó da blockchain.

## Endpoint

Todas as requisições devem ser feitas via `POST` para o endpoint raiz `/` do servidor RPC (a porta padrão ainda precisa ser definida, mas geralmente é algo como `8080` ou `8545`).

## Formato da Requisição

As requisições devem seguir o padrão JSON-RPC 2.0:

```json
{
  "jsonrpc": "2.0",
  "method": "nome_do_metodo",
  "params": { /* parâmetros específicos do método */ },
  "id": 1 /* um ID único para a requisição (número ou string) */
}
```

## Formato da Resposta

As respostas também seguem o padrão JSON-RPC 2.0:

**Sucesso:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* resultado específico do método */ },
  "error": null,
  "id": 1 /* mesmo ID da requisição */
}
```

**Erro:**
```json
{
  "jsonrpc": "2.0",
  "result": null,
  "error": {
    "code": -32xxx, /* código de erro JSON-RPC */
    "message": "Descrição do erro",
    "data": null /* dados adicionais opcionais sobre o erro */
  },
  "id": 1 /* mesmo ID da requisição */
}
```

## Métodos Disponíveis

### `send_transaction`

Envia uma nova transação para ser adicionada ao mempool do nó.

*   **Parâmetros (`params`):**
    ```json
    {
      "transaction": { /* Objeto Transaction serializado em JSON */
        "sender": [/* array de bytes (u8) */],
        "recipient": [/* array de bytes (u8) */],
        "amount": 100,
        "data": null, /* ou { "Storage": { "payload": "dados...", "payload_hash": [/* hash */] } } */
        "timestamp": 1678886400,
        "signature": null /* Assinatura (ainda não implementada) */
      }
    }
    ```
*   **Resultado (`result`):**
    *   `string`: O hash da transação (hexadecimal) se ela foi aceita no mempool (ou já existia).
*   **Exemplo de Requisição:**
    ```bash
    curl -X POST -H "Content-Type: application/json" --data 
    {\"jsonrpc\":\"2.0\",\"method\":\"send_transaction\",\"params\":{\"transaction\":{\"sender\":[1],\"recipient\":[2],\"amount\":100,\"data\":null,\"timestamp\":1678886400,\"signature\":null}},\"id\":1}
     http://localhost:8080/
    ```
*   **Exemplo de Resposta (Sucesso):**
    ```json
    {
      "jsonrpc": "2.0",
      "result": "a1b2c3...",
      "error": null,
      "id": 1
    }
    ```

### `get_chain_height`

Retorna a altura atual da blockchain (o índice do último bloco).

*   **Parâmetros (`params`):** `{}` (Objeto vazio)
*   **Resultado (`result`):**
    *   `number` (u64): A altura atual da cadeia (0 para o bloco gênesis).
*   **Exemplo de Requisição:**
    ```bash
    curl -X POST -H "Content-Type: application/json" --data 
    {\"jsonrpc\":\"2.0\",\"method\":\"get_chain_height\",\"params\":{},\"id\":2}
     http://localhost:8080/
    ```
*   **Exemplo de Resposta (Sucesso):**
    ```json
    {
      "jsonrpc": "2.0",
      "result": 5,
      "error": null,
      "id": 2
    }
    ```

### `get_block_by_height`

Retorna um bloco específico pela sua altura.

*   **Parâmetros (`params`):**
    ```json
    {
      "height": 5 /* número (u64) */
    }
    ```
*   **Resultado (`result`):**
    *   `object` (Block): O objeto do bloco serializado em JSON, ou `null` se o bloco não for encontrado.
*   **Exemplo de Requisição:**
    ```bash
    curl -X POST -H "Content-Type: application/json" --data 
    {\"jsonrpc\":\"2.0\",\"method\":\"get_block_by_height\",\"params\":{\"height\":5},\"id\":3}
     http://localhost:8080/
    ```
*   **Exemplo de Resposta (Sucesso - Bloco Encontrado):**
    ```json
    {
      "jsonrpc": "2.0",
      "result": {
        "header": { /* ... */ },
        "transactions": [ /* ... */ ]
      },
      "error": null,
      "id": 3
    }
    ```
*   **Exemplo de Resposta (Sucesso - Bloco Não Encontrado):**
    ```json
    {
      "jsonrpc": "2.0",
      "result": null,
      "error": null,
      "id": 3
    }
    ```

### `get_block_by_hash`

Retorna um bloco específico pelo seu hash.

*   **Parâmetros (`params`):**
    ```json
    {
      "hash": "a1b2c3..." /* string hexadecimal de 64 caracteres */
    }
    ```
*   **Resultado (`result`):**
    *   `object` (Block): O objeto do bloco serializado em JSON, ou `null` se o bloco não for encontrado.
*   **Exemplo de Requisição:**
    ```bash
    curl -X POST -H "Content-Type: application/json" --data 
    {\"jsonrpc\":\"2.0\",\"method\":\"get_block_by_hash\",\"params\":{\"hash\":\"a1b2c3...\"},\"id\":4}
     http://localhost:8080/
    ```
*   **Exemplo de Resposta (Sucesso - Bloco Encontrado):**
    ```json
    {
      "jsonrpc": "2.0",
      "result": {
        "header": { /* ... */ },
        "transactions": [ /* ... */ ]
      },
      "error": null,
      "id": 4
    }
    ```

### (Futuro) `get_transaction_by_hash`

Retorna uma transação específica pelo seu hash.

### (Futuro) `get_mempool_info`

Retorna informações sobre o estado atual do mempool.

