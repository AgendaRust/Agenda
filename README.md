# 🗓️ Planner Virtual em Rust com Rocket

Este projeto é uma implementação de um planner virtual, desenvolvido para a disciplina de Paradigmas de Linguagens de Programação da Universidade Federal do Agreste de Pernambuco (UFAPE). O sistema foi construído utilizando a linguagem de programação Rust e o framework web Rocket.

| [<img loading="lazy" src="https://avatars.githubusercontent.com/u/52945665?v=4" width="115" style="border-radius: 50%;"><br><sub>Gabriel Silva</sub>](https://github.com/gabrielZZ231) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/194650600?v=4" width="115" style="border-radius: 50%;"><br><sub>Lucas Marques</sub>](https://github.com/marquesdiff) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/62724100?v=4" width="115" style="border-radius: 50%;"><br><sub>Raylandson Cesário</sub>](https://github.com/Raylandson) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/102694110?v=4" width="115" style="border-radius: 50%;"><br><sub>Jorge Ribeiro</sub>](https://github.com/JorgRibeiro) | [<img loading="lazy" src="https://avatars.githubusercontent.com/u/117954648?v=4" width="115" style="border-radius: 50%;"><br><sub>Clívisson Barbosa</sub>](https://github.com/clivissonjose) |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |

## 🎯 Sobre o Projeto

O objetivo principal é criar um sistema de planner que permita aos usuários organizar suas metas e tarefas diárias de forma eficiente. O planner oferece funcionalidades para criação, acompanhamento e análise de produtividade, com os dados sendo armazenados de forma persistente.

## 🚀 Como Executar o Projeto

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão mais recente)
- [cargo-watch](https://crates.io/crates/cargo-watch) para desenvolvimento com hot reload

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

3. **Configure o banco de dados**

   ```bash
   cd backend
   # Certifique-se de que existe um arquivo .env na pasta backend com as configurações do banco
   ```

4. **Execute o backend**

   **Para desenvolvimento (com auto-reload):**

   ```bash
   cd backend
   cargo watch -x run
   ```

   **Para execução simples:**

   ```bash
   cd backend
   cargo run
   ```

5. **Acesse a API**
   ```
   http://localhost:8000
   ```

```

## ✨ Funcionalidades

O sistema oferece uma gama de funcionalidades para ajudar o usuário a se organizar e acompanhar seu progresso.

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
      <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png" width="40" alt="Rust"/>
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
      <img src="https://avatars.githubusercontent.com/u/20165699?s=200&v=4" width="40" alt="SeaORM"/>
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

Em desenvolvimento 🚧
```
