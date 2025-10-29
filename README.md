# 🗓️ Planner Virtual em Rust com Rocket

Este projeto é uma implementação de um planner virtual, desenvolvido para a disciplina de Paradigmas de Linguagens de Programação da Universidade Federal do Agreste de Pernambuco (UFAPE). O sistema foi construído utilizando a linguagem de programação Rust e o framework web Rocket.

| [<img loading="lazy" src="https://avatars.githubusercontent.com/u/52945665?v=4" width="115" style="border-radius: 50%;"><br><sub>Gabriel Silva</sub>](https://github.com/gabrielZZ231) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/62724100?v=4" width="115" style="border-radius: 50%;"><br><sub>Raylandson Cesário</sub>](https://github.com/Raylandson) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/102694110?v=4" width="115" style="border-radius: 50%;"><br><sub>Jorge Ribeiro</sub>](https://github.com/JorgRibeiro) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/117954648?v=4" width="115" style="border-radius: 50%;"><br><sub>Clívisson Barbosa</sub>](https://github.com/clivissonjose) |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |

## 🎯 Sobre o Projeto

O objetivo principal é criar um sistema de planner que permita aos usuários organizar suas metas e tarefas diárias de forma eficiente. O planner oferece funcionalidades para criação, acompanhamento e análise de produtividade, com os dados sendo armazenados de forma persistente.

## ⚡ Início Rápido

### Opção 1: Docker (Recomendado)

```bash
# Clone o repositório
git clone https://github.com/AgendaRust/Agenda.git
cd Agenda

# Configure as variáveis de ambiente
cat > .env << EOF
JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum
DB_USER=agenda_user
DB_PASSWORD=secure_password_change_me
DB_NAME=agenda_db
DB_HOST=db
DB_PORT=5432
DATABASE_URL=postgresql://\${DB_USER}:\${DB_PASSWORD}@\${DB_HOST}:\${DB_PORT}/\${DB_NAME}
EOF

# Compile o frontend
cd frontend
trunk build --release
cd ..

# Inicie todos os serviços com Docker
docker compose up --build
```

**URLs:**
- Frontend: http://localhost:8000
- Backend API: http://localhost:8000/api
- Adminer (Database UI): http://localhost:8080 (only with `--profile debug`)

### Opção 2: Desenvolvimento Local

```bash
# Clone o repositório
git clone https://github.com/AgendaRust/Agenda.git
cd Agenda

# Inicie o PostgreSQL (backend)
cd backend
docker-compose up -d

# Configure e execute as migrações
echo "DATABASE_URL=postgresql://agenda_user:secure_password_change_me@localhost:5432/agenda_db" > .env
echo "JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum" >> .env
cargo install sea-orm-cli
cd migration
cargo run
cd ..

# Execute o backend
cargo run

# Em outro terminal, execute o frontend
cd ../frontend
trunk serve --port 8081
```

**URLs:**
- Backend API: http://localhost:8000
- Frontend: http://localhost:8081
- Adminer (Database UI): http://localhost:8080

## 🚀 Como Executar o Projeto

### Pré-requisitos

**Para executar com Docker (Recomendado):**
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

**Para desenvolvimento local:**
- [Rust](https://www.rust-lang.org/tools/install) (versão mais recente)
- [Docker](https://docs.docker.com/get-docker/) (apenas para PostgreSQL)
- [Trunk](https://trunkrs.dev/) para build do frontend
- [cargo-watch](https://crates.io/crates/cargo-watch) (opcional) para desenvolvimento com hot reload
- Compilador C ([Clang](https://clang.llvm.org/) ou [GCC](https://gcc.gnu.org/)): O frontend possui dependências (como a crate ring usada para criptografia em jsonwebtoken) que necessitam de um compilador C para serem compiladas.

### Passo a Passo

#### Opção A: Usando Docker (Produção)

1. **Clone o repositório**

   ```bash
   git clone https://github.com/AgendaRust/Agenda.git
   cd Agenda
   ```

2. **Configure as variáveis de ambiente**

   Crie o arquivo `.env` na raiz do projeto:

   ```bash
   cat > .env << EOF
   JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum
   DB_USER=agenda_user
   DB_PASSWORD=secure_password_change_me
   DB_NAME=agenda_db
   DB_HOST=db
   DB_PORT=5432
   DATABASE_URL=postgresql://\${DB_USER}:\${DB_PASSWORD}@\${DB_HOST}:\${DB_PORT}/\${DB_NAME}
   EOF
   ```

3. **Compile o frontend**

   ⚠️ **IMPORTANTE:** Sempre use `--release` para produção!

   ```bash
   cd frontend
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   trunk build --release  # NÃO use apenas "trunk build"
   cd ..
   ```

   **Por quê `--release` é obrigatório?**
   - `trunk build` (sem --release) = build de desenvolvimento com WebSocket para hot-reload
   - `trunk build --release` = build otimizado para produção sem código de desenvolvimento
   
   Se você usar apenas `trunk build`, verá erros de WebSocket no navegador.

4. **Inicie todos os serviços**

   ```bash
   docker compose up --build
   ```

   Isso irá:
   - Criar e iniciar o banco de dados PostgreSQL
   - Executar as migrações automaticamente
   - Iniciar o backend (API Rust/Rocket)
   - Servir o frontend compilado
   - Iniciar o Adminer (interface web para gerenciar o banco)

5. **Acesse a aplicação**
   - Frontend: http://localhost:8000
   - Backend API: http://localhost:8000/api
   - Adminer: http://localhost:8080 (opcional, apenas para debug)

   **Nota:** Por padrão, o Adminer não é iniciado. Para usá-lo, execute:
   ```bash
   docker compose --profile debug up -d adminer
   ```

#### Opção B: Desenvolvimento Local

1. **Clone o repositório**

   ```bash
   git clone https://github.com/AgendaRust/Agenda.git
   cd Agenda
   ```

2. **Inicie o PostgreSQL**

   ```bash
   cd backend
   docker-compose up -d
   ```

   Isso irá iniciar:
   - PostgreSQL na porta 5432
   - Adminer na porta 8080

3. **Configure o backend**

   ```bash
   cd backend
   ```

   **Crie o arquivo .env:**

   ```bash
   cat > .env << EOF
   DATABASE_URL=postgresql://agenda_user:secure_password_change_me@localhost:5432/agenda_db
   JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum
   DB_USER=agenda_user
   DB_PASSWORD=secure_password_change_me
   DB_NAME=agenda_db
   EOF
   ```

   **Instale o SeaORM CLI:**

   ```bash
   cargo install sea-orm-cli
   ```

   **Execute as migrations:**

   ```bash
   cd migration
   cargo run
   cd ..
   ```

4. **Execute o backend**

   **Para desenvolvimento (com auto-reload):**

   ```bash
   cargo install cargo-watch  # se ainda não tiver
   cargo watch -x run
   ```

   **Para execução simples:**

   ```bash
   cargo run
   ```

5. **Execute o frontend** (em outro terminal)

   ```bash
   cd frontend
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   trunk serve --port 8081
   ```

6. **Acesse a aplicação**
   - Backend API: http://localhost:8000
   - Frontend: http://localhost:8081
   - Adminer: http://localhost:8080

**Nota:** O frontend em modo de desenvolvimento (`trunk serve --port 8081`) automaticamente se conectará ao backend em `http://localhost:8000/api`. Em produção (Docker), usa o caminho relativo `/api`.

## 🌐 Deploy em Produção

### Pré-requisitos para Produção

1. **Servidor Linux** com Docker e Docker Compose instalados
2. **Domínio configurado** (exemplo: DuckDNS)
3. **Nginx** instalado como reverse proxy
4. **Certificado SSL** (Let's Encrypt recomendado)

### Passo a Passo para Deploy

1. **Clone o repositório no servidor**

   ```bash
   ssh user@your-server
   git clone https://github.com/AgendaRust/Agenda.git
   cd Agenda
   ```

2. **Configure variáveis de ambiente de produção**

   ⚠️ **IMPORTANTE:** Gere senhas fortes e únicas!

   ```bash
   # Gere um JWT secret forte
   JWT_SECRET=$(openssl rand -base64 32)
   
   # Gere uma senha forte para o banco
   DB_PASSWORD=$(openssl rand -base64 24)
   
   # Crie o arquivo .env
   cat > .env << EOF
   JWT_SECRET_KEY=${JWT_SECRET}
   DB_USER=agenda_user
   DB_PASSWORD=${DB_PASSWORD}
   DB_NAME=agenda_db
   DB_HOST=db
   DB_PORT=5432
   DATABASE_URL=postgresql://\${DB_USER}:\${DB_PASSWORD}@\${DB_HOST}:\${DB_PORT}/\${DB_NAME}
   EOF
   
   # Proteja o arquivo
   chmod 600 .env
   ```

3. **Compile o frontend no servidor (ou em sua máquina local)**

   ```bash
   cd frontend
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   trunk build --release
   cd ..
   ```

4. **Configure o Nginx como reverse proxy**

   ```bash
   sudo nano /etc/nginx/sites-available/agenda
   ```

   Cole a seguinte configuração:

   ```nginx
   server {
       listen 80;
       server_name seu-dominio.duckdns.org;
       
       # Redirect HTTP to HTTPS
       return 301 https://$server_name$request_uri;
   }

   server {
       listen 443 ssl http2;
       server_name seu-dominio.duckdns.org;

       # SSL Configuration (Let's Encrypt)
       ssl_certificate /etc/letsencrypt/live/seu-dominio.duckdns.org/fullchain.pem;
       ssl_certificate_key /etc/letsencrypt/live/seu-dominio.duckdns.org/privkey.pem;
       
       # Strong SSL settings
       ssl_protocols TLSv1.2 TLSv1.3;
       ssl_ciphers HIGH:!aNULL:!MD5;
       ssl_prefer_server_ciphers on;

       # Security headers
       add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
       add_header X-Frame-Options "SAMEORIGIN" always;
       add_header X-Content-Type-Options "nosniff" always;

       location / {
           proxy_pass http://127.0.0.1:8000;
           
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
           proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
           proxy_set_header X-Forwarded-Proto $scheme;
           
           # WebSocket support (if needed)
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection "upgrade";
       }
   }
   ```

   Ative o site:
   ```bash
   sudo ln -s /etc/nginx/sites-available/agenda /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl reload nginx
   ```

5. **Obtenha certificado SSL com Let's Encrypt**

   ```bash
   sudo apt update
   sudo apt install certbot python3-certbot-nginx
   sudo certbot --nginx -d seu-dominio.duckdns.org
   ```

   O certbot irá:
   - Gerar o certificado SSL
   - Configurar renovação automática
   - Atualizar sua configuração do Nginx

6. **Inicie a aplicação**

   ```bash
   docker compose up -d --build
   ```

7. **Verifique os logs**

   ```bash
   docker compose logs -f
   ```

8. **Acesse sua aplicação**

   ```
   https://seu-dominio.duckdns.org
   ```

### Configuração do Roteador

Certifique-se de que as seguintes portas estão abertas no seu roteador:

- **Porta 80** (HTTP - para redirecionamento e Let's Encrypt)
- **Porta 443** (HTTPS - para acesso seguro)

### Comandos Úteis para Produção

```bash
# Ver logs em tempo real
docker compose logs -f app

# Reiniciar apenas o backend
docker compose restart app

# Parar tudo
docker compose down

# Parar e remover volumes (cuidado! apaga dados)
docker compose down -v

# Backup do banco de dados
docker compose exec db pg_dump -U agenda_user agenda_db > backup_$(date +%Y%m%d).sql

# Restaurar backup
cat backup_20231025.sql | docker compose exec -T db psql -U agenda_user -d agenda_db

# Atualizar aplicação
git pull
cd frontend && trunk build --release && cd ..
docker compose up -d --build
```

### Segurança em Produção

✅ **Checklist de Segurança:**

- [ ] JWT_SECRET_KEY com pelo menos 32 caracteres aleatórios
- [ ] Senha forte do banco de dados (DB_PASSWORD)
- [ ] Arquivo `.env` com permissões restritas (`chmod 600 .env`)
- [ ] HTTPS configurado com certificado válido
- [ ] Adminer desabilitado (não inicie com `--profile debug` em produção)
- [ ] Firewall configurado (apenas portas 80, 443 e SSH abertas)
- [ ] Backups automáticos do banco de dados configurados
- [ ] Monitoramento de logs ativo

### Manutenção

**Renovação automática do SSL:**
O certbot configura automaticamente a renovação. Teste com:
```bash
sudo certbot renew --dry-run
```

**Backup automático:**
Crie um cron job para backup diário:
```bash
crontab -e
```

Adicione:
```cron
0 2 * * * cd /caminho/para/Agenda && docker compose exec -T db pg_dump -U agenda_user agenda_db > backup_$(date +\%Y\%m\%d).sql
```

## 🗄️ Configuração do Banco de Dados

Este projeto utiliza **PostgreSQL** com SeaORM para gerenciamento do banco de dados e migrations.

### Variáveis de Ambiente Necessárias

O arquivo `.env` deve conter as seguintes variáveis:

**Para Docker (raiz do projeto - .env):**
```bash
JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum
DB_USER=agenda_user
DB_PASSWORD=secure_password_change_me
DB_NAME=agenda_db
DB_HOST=db
DB_PORT=5432
DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
```

**Para desenvolvimento local (backend/.env):**
```bash
DATABASE_URL=postgresql://agenda_user:secure_password_change_me@localhost:5432/agenda_db
JWT_SECRET_KEY=your-super-secret-jwt-key-here-must-be-32-chars-minimum
DB_USER=agenda_user
DB_PASSWORD=secure_password_change_me
DB_NAME=agenda_db
DB_HOST=localhost
DB_PORT=5432
```

**Importante**:
- `DATABASE_URL`: String de conexão com PostgreSQL
- `JWT_SECRET_KEY`: Chave secreta para assinatura de tokens JWT (deve ter pelo menos 32 caracteres para segurança)
- `DB_HOST`: Use `db` para Docker ou `localhost` para desenvolvimento local

### Comandos de Migration Úteis

```bash
# Aplicar todas as migrations pendentes (via Docker)
docker compose run migrations ./migration-cli up

# Aplicar migrations localmente
cd backend/migration
cargo run

# Verificar status das migrations
sea-orm-cli migrate status

# Reverter a última migration
sea-orm-cli migrate down

# Resetar o banco (cuidado! apaga todos os dados)
sea-orm-cli migrate reset

# Gerar uma nova migration
sea-orm-cli migrate generate nome_da_migration

# Criar as entidades
sea-orm-cli generate entity -u postgresql://user:password@localhost:5432/agenda_db -o src/entity
```

### Estrutura do Banco de Dados

**Tabelas principais:**
- `user` - Usuários do sistema
- `task` - Tarefas diárias (begin_date, complete_date, category, type)
- `goal` - Metas (date_start, date_end, category, type)
- `reminder` - Lembretes semanais (date_end, category)
- `notes` - Notas (created_at)

**Nota:** Todas as colunas de data/hora utilizam `TIMESTAMPTZ` (timestamp with timezone) para compatibilidade com `DateTimeUtc` do Rust.

### Acessando o Banco de Dados

**Via Adminer (Interface Web):**
1. Acesse http://localhost:8080
2. Faça login com:
   - System: `PostgreSQL`
   - Server: `db` (Docker) ou `localhost` (local)
   - Username: valor de `DB_USER`
   - Password: valor de `DB_PASSWORD`
   - Database: valor de `DB_NAME`

**Via linha de comando:**
```bash
# Usando Docker
docker compose exec db psql -U agenda_user -d agenda_db

# Localmente (se PostgreSQL estiver instalado)
psql postgresql://agenda_user:secure_password_change_me@localhost:5432/agenda_db
```

### Solução de Problemas

**Erro de conexão com PostgreSQL:**
- Verifique se o Docker está rodando: `docker ps`
- Confirme que o PostgreSQL está ativo: `docker compose ps`
- Verifique se as variáveis de ambiente estão corretas no `.env`
- Para Docker, use `DB_HOST=db`; para local, use `DB_HOST=localhost`

**Migration não funciona:**
- Certifique-se de que o PostgreSQL está rodando
- Verifique a string de conexão em `DATABASE_URL`
- Execute `cargo install sea-orm-cli` para instalar a CLI
- No Docker, as migrations são executadas automaticamente no serviço `migrations`

**Erros de WebSocket no frontend (`{{__TRUNK_ADDRESS__}}` etc.):**
- Certifique-se de usar `trunk build --release` (não apenas `trunk build`)
- O build de desenvolvimento não deve ser usado no Docker
- Verifique se não há placeholders Trunk no `frontend/dist/index.html`:
  ```bash
  grep -c "{{__TRUNK" frontend/dist/index.html  # Deve retornar 0
  ```

**Resetar o banco de dados completamente:**
```bash
# Parar containers e remover volumes
docker compose down -v

# Reiniciar tudo do zero
docker compose up --build
```

**⚠️ IMPORTANTE: Mudança de senha no .env**
Se você alterar a senha do banco no `.env`, precisa remover o volume antigo:
```bash
docker compose down -v  # O -v remove os volumes
docker compose up --build
```

**Acessar Adminer para debug:**
```bash
# Iniciar Adminer
docker compose --profile debug up -d adminer

# Acessar em http://localhost:8080
# Server: db
# Username: valor de DB_USER
# Password: valor de DB_PASSWORD
# Database: valor de DB_NAME

# Parar Adminer
docker compose --profile debug down
```

### Estrutura do Projeto

```
Agenda/
├── backend/              # API em Rust com Rocket
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   ├── entity/
│   │   ├── dto/
│   │   └── service/
│   ├── migration/        # Migrations do banco de dados
│   ├── docker-compose.yml # PostgreSQL + Adminer (dev local)
│   └── Cargo.toml
├── frontend/             # Interface web em Yew (WebAssembly)
│   ├── src/
│   │   ├── main.rs
│   │   ├── components/
│   │   ├── pages/
│   │   └── services/
│   ├── dist/            # Build de produção (gerado pelo Trunk)
│   ├── index.html
│   └── Cargo.toml
├── .dockerfile          # Multi-stage build para produção
├── docker-compose.yml   # Orquestração completa da aplicação
├── .env                 # Variáveis de ambiente
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
      <img src="https://wiki.postgresql.org/images/a/a4/PostgreSQL_logo.3colors.svg" width="40" alt="PostgreSQL"/>
      <br/>
      <a href="https://www.postgresql.org/" target="_blank">PostgreSQL</a>
    </td>
    <td align="center">
      <img src="https://www.docker.com/wp-content/uploads/2022/03/vertical-logo-monochromatic.png" width="40" alt="Docker"/>
      <br/>
      <a href="https://www.docker.com/" target="_blank">Docker</a>
    </td>
  </tr>
</table>

### Stack Completa

- **Backend**: Rust + Rocket (Web Framework)
- **Frontend**: Rust + Yew (WebAssembly Framework)
- **ORM**: SeaORM (Migrations e Query Builder)
- **Banco de Dados**: PostgreSQL 16
- **Containerização**: Docker + Docker Compose
- **Admin DB**: Adminer (Interface web para PostgreSQL)

## 🎓 Agradecimentos

- Projeto acadêmico desenvolvido para a disciplina de Paradigmas de Linguagens de Programação.
- Professor: **Dimas Cassimiro do Nascimento Filho**.
- Instituição: Universidade Federal do Agreste de Pernambuco.

## Status do Projeto

Concluído ✅
