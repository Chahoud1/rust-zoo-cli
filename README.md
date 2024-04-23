# Sistema de Gerenciamento de Animais

Esta é uma aplicação de linha de comando simples para gerenciar registros de animais usando Rust. A aplicação permite que você registre, liste, atualize e exclua registros de animais. Cada registro é armazenado em um arquivo JSON, facilitando a manutenção de dados persistentes.

## Funcionalidades

- **Registrar um novo animal:** Adicione um novo registro de animal com detalhes como ID, nome, idade e status de vacinação.
- **Listar todos os animais:** Exiba todos os animais registrados junto com seus detalhes.
- **Atualizar um animal:** Modifique os detalhes de um animal existente.
- **Excluir um animal:** Remova o registro de um animal do sistema.
- **Armazenamento persistente:** Todos os registros de animais são armazenados em um arquivo JSON, garantindo que os dados sejam salvos entre as sessões.

## Dependências

- `serde` e `serde_json`: Para serialização e desserialização de registros de animais em formato JSON.
- `std::io`, `std::fs`, `std::path`: Para operações de entrada/saída e manipulação de arquivos.

Certifique-se de ter o Rust e o Cargo instalados para gerenciar essas dependências e executar a aplicação.

## Configuração e Execução

1. Clone o repositório:
git clone <url-do-seu-repositorio>
cd <diretorio-do-seu-repositorio>

2. Construa o projeto:
cargo build

3. Execute a aplicação:
cargo run

## Uso

Ao executar a aplicação, será apresentado um menu:

1-Registrar um novo Animal <br>
2-Listar Animais <br>
3-Atualizar um Animal <br>
4-Excluir Animal <br>
5-Sair


Selecione uma opção digitando o número correspondente e siga as instruções para gerenciar os registros dos animais.

## Arquivo JSON

A aplicação lê e escreve em um arquivo `animals.json` no diretório raiz. Se o arquivo não existir, ele será criado ao adicionar o primeiro animal.
