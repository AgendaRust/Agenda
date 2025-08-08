# Arquitetura do Sistema de Planner

Este documento detalha a estrutura de diretórios, a filosofia de design e o fluxo de dados do nosso projeto. Ele serve como a fonte da verdade para a arquitetura da aplicação.

-----

## 💡 Filosofia  

Nossa arquitetura é baseada no princípio de **Separação de Responsabilidades**. O objetivo é manter a lógica de HTTP (`routes`), a lógica de negócio (`services`) e a manipulação de dados (`models`) o mais isolado possível. Isso torna o código mais modular, testável e fácil de manter.

## 📁 Estrutura de Diretórios {#filo}

A estrutura de alto nível do projeto é organizada da seguinte forma:

```bash
planner_system/
├── Cargo.toml      # Dependências Rust (crates) e metadados do projeto.
├── Rocket.toml     # Configurações do Rocket (ambiente, banco de dados, porta, etc.).
├── static/         # Arquivos estáticos servidos diretamente (CSS, JavaScript, Imagens).
├── templates/      # Templates HTML (Tera) para renderização no servidor.
└── src/            # O coração da nossa aplicação, contendo todo o código-fonte Rust.
```

## ⚙️ Componentes Principais da Aplicação (`src/`)

O diretório `src/` contém os pilares da nossa aplicação.

### Ponto de Entrada (`main.rs`)

É aqui que a aplicação é inicializada e configurada.

- **Responsabilidades:**
    - Configurar e iniciar o servidor web do Rocket.
    - Gerenciar o estado global da aplicação, como a pool de conexões com o banco de dados (usando `State<DbPool>`).
    - Montar todas as rotas definidas no módulo `routes/`.
    - Anexar "fairings" (middlewares) para tarefas como gerenciamento de transações de banco de dados ou logging.

### Banco de Dados (`db.rs`, `schema.rs`)

Gerencia toda a interação com o banco de dados para garantir a persistência dos dados.

- **`db.rs`:**
    - Define a pool de conexões (ex: usando `r2d2` com o ORM `Diesel`).
    - Cria um *request guard* (`DbConn`) para injetar de forma segura uma conexão de banco de dados em cada rota que precisar.
- **`schema.rs`:**
    - **Gerado automaticamente pelo `diesel_cli`**.
    - Mapeia as tabelas e colunas do banco de dados para o sistema de tipos do Rust, garantindo a segurança de tipos em todas as queries.

### Tratamento de Erros (`errors.rs`)

Define tipos de erro customizados para um tratamento centralizado e consistente.

- **Responsabilidades:**
    - Criar um `enum AppError` que represente todas as falhas possíveis (ex: `NotFound`, `DatabaseError`, `Unauthorized`).
    - Implementar o trait `Responder` do Rocket para nosso `AppError`. Isso permite que as rotas retornem `Result<T, AppError>` e o Rocket transforme o erro em uma resposta HTTP apropriada (ex: `AppError::NotFound` se torna um status `404`).

### Middlewares (`fairings.rs`)

Contém a lógica que é executada em diferentes fases do ciclo de vida de uma requisição.

- **Exemplos de Uso:**
    - Implementar um logger para registrar todas as requisições recebidas.
    - Adicionar cabeçalhos de segurança, como `Content-Security-Policy` (CSP), a todas as respostas.

## 🧩 Módulos de Lógica de Negócio

Estes módulos contêm a lógica específica do nosso sistema de planner.

### Modelos de Dados (`models/`)

Define as estruturas de dados que representam as entidades do nosso sistema e como elas são persistidas.

- **`mod.rs`:** Declara os submódulos de modelos (`pub mod user;`, `pub mod goal;`, etc.).
- **`user.rs`:** `struct User` - Representa um usuário (id, username, email, password\_hash).
- **`goal.rs`:** `struct Goal` - Representa uma meta (id, user\_id, descrição, período, status).
- **`task.rs`:** `struct Task` - Representa uma tarefa (id, user\_id, descrição, data, status).
- **`category.rs`:** `struct Category` - Representa uma categoria (id, user\_id, nome, cor).
- **`reminder.rs`:** `struct Reminder` - Representa um lembrete semanal (id, user\_id, descrição, dia\_da\_semana).

### Controladores/Endpoints (`routes/`)

Define os endpoints da nossa API ou aplicação web. A lógica aqui deve ser "magra", focando em orquestrar a interação entre o cliente e os serviços.

- **`mod.rs`:** Agrega e expõe todas as rotas para serem montadas no `main.rs`.
- **`auth.rs`:** Rotas de autenticação (`/register`, `/login`, `/logout`).
- **`goals.rs`:** Rotas CRUD para metas (`POST /goals`, `GET /goals?period=monthly`, `PUT /goals/{id}`).
- **`tasks.rs`:** Rotas CRUD para tarefas (`POST /tasks`, `GET /tasks?date=2024-08-10`).
- **`reminders.rs`:** Rotas CRUD para lembretes.
- **`reports.rs`:** Rota para gerar e exibir relatórios (`GET /reports?range=monthly`).

### Serviços (`services/`)

Isola a lógica de negócio complexa das rotas, tornando o código mais limpo e testável.

- **`mod.rs`:** Declara os submódulos de serviço.
- **`report_service.rs`:** Contém a lógica pesada para a geração dos relatórios.
    - **Funções:** `generate_weekly_report(user_id)`, `generate_monthly_report(user_id)`.
    - **Responsabilidades:**
        - Buscar dados de metas e tarefas.
        - Calcular porcentagens de conclusão.
        - Analisar picos de produtividade (turnos do dia, dias da semana).
        - Identificar as categorias mais utilizadas.
- **`category_service.rs`:** Lógica relacionada a categorias, como validações ou buscas específicas.

## 🌊 Exemplo de Fluxo de Requisição: Criar uma Tarefa

Para ilustrar como os componentes interagem, veja o fluxo para a criação de uma nova tarefa:

1.  **Frontend:** O usuário submete o formulário de "Nova Tarefa".
2.  **`routes/tasks.rs`:** A rota `POST /tasks` recebe a requisição. Ela usa os *request guards* do Rocket para validar e deserializar os dados do formulário em uma `struct`.
3.  **Delegação:** O handler da rota **não** executa a lógica de inserção diretamente. Ele chama uma função do modelo correspondente, como `Task::create(...)`.
4.  **`models/task.rs`:** O método `Task::create` recebe os dados, constrói a query SQL usando o Diesel e a executa, inserindo a nova tarefa no banco de dados.
5.  **Resposta:**
    - **Sucesso:** A rota redireciona o usuário para a página do planner diário.
    - **Erro:** O método no modelo retorna um `Err`. O handler da rota propaga esse erro, que é capturado pelo `Responder` do `errors.rs` e convertido em uma resposta HTTP apropriada (ex: `500 Internal Server Error`).