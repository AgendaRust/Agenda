# 🗓️ Planner Virtual em Rust com Rocket

Este projeto é uma implementação de um planner virtual, desenvolvido para a disciplina de Paradigmas de Linguagens de Programação da Universidade Federal do Agreste de Pernambuco (UFAPE). O sistema foi construído utilizando a linguagem de programação Rust e o framework web Rocket.

| [<img loading="lazy" src="https://avatars.githubusercontent.com/u/52945665?v=4" width="115" style="border-radius: 50%;"><br><sub>Gabriel Silva</sub>](https://github.com/gabrielZZ231) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/62724100?v=4" width="115" style="border-radius: 50%;"><br><sub>Raylandson Cesário</sub>](https://github.com/Raylandson) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/102694110?v=4" width="115" style="border-radius: 50%;"><br><sub>Jorge Ribeiro</sub>](https://github.com/JorgRibeiro) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/117954648?v=4" width="115" style="border-radius: 50%;"><br><sub>Clívisson Barbosa</sub>](https://github.com/clivissonjose) |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |

## 🎯 Sobre o Projeto

O objetivo principal é criar um sistema de planner que permita aos usuários organizar suas metas e tarefas diárias de forma eficiente. O planner oferece funcionalidades para criação, acompanhamento e análise de produtividade, com os dados sendo armazenados de forma persistente.

## ⚡ Início Rápido

```bash
# Clone o repositório
git clone https://github.com/AgendaRust/Agenda.git
cd Agenda

# Configure e execute o backend
cd backend
echo "DATABASE_URL=sqlite:./database.db" > .env
echo "JWT_SECRET_KEY=your-super-secret-jwt-key-here" >> .env
touch database.db
cargo install sea-orm-cli
sea-orm-cli migrate up
cargo run

# Em outro terminal, execute o frontend
cd ../frontend
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

**URLs:**

- Backend API: http://localhost:8000
- Frontend: http://localhost:8080

## 🚀 Como Executar o Projeto

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão mais recente)
- [cargo-watch](https://crates.io/crates/cargo-watch) para desenvolvimento com hot reload
- Compilador C ([Clang](https://clang.llvm.org/) ou [GCC](https://gcc.gnu.org/)): O frontend possui dependências (como a crate ring usada para criptografia em jsonwebtoken) que necessitam de um compilador C para serem compiladas.

### Passo a Passo

1. **Clone o repositório**

   ```bash
   git clone https://github.com/AgendaRust/Agenda.git
   cd Agenda
   ```

2. **Instale o cargo-watch** (opcional, para desenvolvimento com auto-reload)

   ```bash
   cargo install cargo-watch
   ```

3. **Configure o banco de dados e migrations**

   ```bash
   cd backend
   ```

   **Crie o arquivo .env:**

   ```bash
   echo "DATABASE_URL=sqlite:./database.db" > .env
   echo "JWT_SECRET_KEY=your-super-secret-jwt-key-here" >> .env
   ```

   **Crie o banco de dados SQLite:**

   ```bash
   touch database.db
   ```

   **Instale o SeaORM CLI (se ainda não tiver):**

   ```bash
   cargo install sea-orm-cli
   ```

   **Execute as migrations para criar o banco de dados e tabelas:**

   ```bash
   sea-orm-cli migrate up
   ```

   **Verifique se o banco foi criado corretamente:**

   ```bash
   sqlite3 database.db ".tables"
   # Deve mostrar: notes seaql_migrations
   ```

4. **Execute o backend**

   **Para desenvolvimento (com auto-reload):**

   ```bash
   cargo watch -x run
   ```

   **Para execução simples:**

   ```bash
   cargo run
   ```

5. **Acesse a API**
   ```
   http://localhost:8000
   ```

### Executando o Frontend

6. **Volte para o diretório raiz e navegue para o frontend**

   ```bash
   cd ..
   cd frontend
   ```

7. **Instale as dependências do WebAssembly**

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

8. **Instale o Trunk** (ferramenta para build e servir aplicações Yew)

   ```bash
   cargo install trunk
   ```

9. **Execute o frontend**

   **Para desenvolvimento (com hot-reload):**

   ```bash
   trunk serve
   ```

   **Para build de produção:**

   ```bash
   trunk build --release
   ```

10. **Acesse a aplicação web**
    ```
    http://localhost:8080
    ```

## 🗄️ Configuração do Banco de Dados

Este projeto utiliza SQLite com SeaORM para gerenciamento do banco de dados e migrations.

### Variáveis de Ambiente Necessárias

O arquivo `.env` deve conter as seguintes variáveis:

```bash
DATABASE_URL=sqlite:./database.db
JWT_SECRET_KEY=your-super-secret-jwt-key-here
```

**Importante**:

- `DATABASE_URL`: Define a localização do banco de dados SQLite
- `JWT_SECRET_KEY`: Chave secreta para assinatura de tokens JWT (deve ter pelo menos 32 caracteres para segurança)

### Comandos de Migration Úteis

```bash
# Verificar status das migrations
sea-orm-cli migrate status

# Aplicar todas as migrations pendentes
sea-orm-cli migrate up

# Reverter a última migration
sea-orm-cli migrate down

# Resetar o banco (cuidado! apaga todos os dados)
sea-orm-cli migrate reset

# Gerar uma nova migration
sea-orm-cli migrate generate nome_da_migration

# Criar as entidades
sea-orm-cli generate entity -u sqlite:./database.db  -o src/entity
```

### Estrutura do Banco de Dados

**Tabela: notes**

- `id` - INTEGER PRIMARY KEY AUTOINCREMENT
- `text` - TEXT NOT NULL
- `created_at` - TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

### Solução de Problemas

**Erro de conexão com SQLite:**

- Verifique se o arquivo `.env` existe em `backend/.env`
- Confirme que `DATABASE_URL=sqlite:./database.db` está correto
- Confirme que `JWT_SECRET_KEY` está definida com pelo menos 32 caracteres
- **Crie o banco de dados antes das migrations:** `touch database.db` ou `sqlite3 database.db "VACUUM;"`
- Certifique-se de que as features SQLite estão habilitadas no `Cargo.toml`

**Migration não funciona:**

- Execute `touch database.db` ou `sqlite3 database.db "VACUUM;"` para criar o arquivo do banco
- Execute `cargo install sea-orm-cli` para instalar a CLI
- Verifique se está no diretório `backend/` ao executar comandos
- Use `sea-orm-cli migrate status` para ver o estado atual

### Estrutura do Projeto

```
Agenda/
├── backend/          # API em Rust com Rocket
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   ├── entity/
│   │   └── dto/
│   └── Cargo.toml
├── frontend/         # Interface web em Yew
│   ├── src/
│   │   └── main.rs
│   ├── index.html
│   ├── style.css
│   └── Cargo.toml
└── README.md
```

## ✨ Funcionalidades

### 📈 Gestão de Metas

- **Criação de Metas**: O usuário pode criar metas para a semana, mês e ano.
- **Detalhes da Meta**: Cada meta é composta por uma descrição e uma categoria.
- **Acompanhamento**: É possível selecionar se as metas foram atingidas com sucesso, parcialmente atingidas ou não atingidas.

### 📑 Planejamento de Tarefas

- **Agendamento Diário**: Permite criar um planejamento de atividades para um dia específico.
- **Duração da Tarefa**: As tarefas podem ser alocadas em blocos de tempo de meia hora, uma hora ou um turno do dia (manhã, tarde, noite).
- **Detalhes da Tarefa**: Assim como as metas, uma tarefa também possui uma descrição e uma categoria.
- **Status da Tarefa**: O usuário pode marcar as tarefas como executadas, parcialmente executadas ou adiadas.

### 🗂️ Organização e Visualização

- **Destaque por Categoria**: Tarefas e metas podem ser destacadas por categoria, como por exemplo, exibindo itens da mesma categoria com a mesma cor para facilitar a visualização.

### 📆 Lembretes Semanais

- O sistema permite a criação de lembretes semanais para atividades recorrentes, tais como:
  - Ligações importantes.
  - Reuniões.
  - Compras.

### 📊 Relatórios de Produtividade

- **Geração de Relatórios**: O usuário pode gerar relatórios de desempenho semanais, mensais ou anuais.
- **Análise de Desempenho**: Os relatórios incluem:
  - Quantidade e porcentagem de metas cumpridas.
  - Quantidade e porcentagem de tarefas executadas.
  - Destaque para as semanas e os meses mais produtivos.
  - Identificação dos turnos do dia mais produtivos.
  - As categorias de tarefas e metas mais realizadas.

### 💻 Requisito Técnico

- Persistência de Dados: Todos os dados do usuário, como metas e tarefas, são armazenados de maneira persistente, seja em arquivos ou em um banco de dados.

## 🚀 Tecnologias Usadas

<table>
  <tr>
    <td align="center">
      <img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="40" alt="Rust"/>
      <br/>
      <a href="https://www.rust-lang.org/" target="_blank">Rust</a>
    </td>
    <td align="center">
      <img src="https://avatars.githubusercontent.com/u/106361765?v=4" width="40" alt="Rocket"/>
      <br/>
      <a href="https://github.com/rwf2/Rocket" target="_blank">Rocket</a>
    </td>
    <td align="center">
      <img src="https://camo.githubusercontent.com/761eeed6db010be43e40e9a8dc73616a48690f91931e2fb81948c741c3fd7165/68747470733a2f2f7965772e72732f696d672f6c6f676f2e706e67" width="40" alt="Yew"/>
      <br/>
      <a href="https://yew.rs/" target="_blank">Yew</a>
    </td>
    <td align="center">
      <img src="https://www.sea-ql.org/SeaORM-X/img/SeaQL.png" width="40" alt="SeaORM"/>
      <br/>
      <a href="https://github.com/SeaQL/sea-orm" target="_blank">SeaORM</a>
    </td>
    <td align="center">
      <img src="https://www.sqlite.org/images/sqlite370_banner.gif" width="40" alt="SQLite"/>
      <br/>
      <a href="https://www.sqlite.org/" target="_blank">SQLite</a>
    </td>
  </tr>
</table>

## 🎓 Agradecimentos

- Projeto acadêmico desenvolvido para a disciplina de Paradigmas de Linguagens de Programação.
- Professor: **Dimas Cassimiro do Nascimento Filho**.
- Instituição: Universidade Federal do Agreste de Pernambuco.

## Status do Projeto

Concluído ✅
