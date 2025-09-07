use chrono::{Local, Duration, Utc, TimeZone};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use sea_orm::*;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand_core::OsRng;

use crate::entity::{user, task, goal, notes, reminder};

pub struct DatabaseSeeder;

impl DatabaseSeeder {
    pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
        println!("🌱 Iniciando processo de seeding...");

        // Limpar dados existentes (opcional)
        Self::clear_tables(db).await?;

        // Criar dados em ordem de dependência
        let users = Self::seed_users(db).await?;
        println!("✅ {} usuários criados", users.len());

        let tasks = Self::seed_tasks(db, &users).await?;
        println!("✅ {} tarefas criadas", tasks.len());

        let goals = Self::seed_goals(db, &users).await?;
        println!("✅ {} metas criadas", goals.len());

        let notes = Self::seed_notes(db, &users).await?;
        println!("✅ {} notas criadas", notes.len());

        let reminders = Self::seed_reminders(db, &users).await?;
        println!("✅ {} lembretes criados", reminders.len());

        println!("🎉 Seeding concluído com sucesso!");
        Ok(())
    }

    async fn clear_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
        println!("🗑️ Limpando tabelas...");
        
        // Ordem inversa devido às dependências
        let _ = reminder::Entity::delete_many().exec(db).await;
        let _ = notes::Entity::delete_many().exec(db).await;
        let _ = goal::Entity::delete_many().exec(db).await;
        let _ = task::Entity::delete_many().exec(db).await;
        let _ = user::Entity::delete_many().exec(db).await;

        println!("✅ Tabelas limpas");
        Ok(())
    }

    async fn seed_users(db: &DatabaseConnection) -> Result<Vec<i32>, DbErr> {
        let mut user_ids = Vec::new();
        let mut rng = thread_rng();

        let first_names = vec![
            "Ana", "João", "Maria", "Pedro", "Carla", "Lucas", "Fernanda", "Rafael",
            "Juliana", "Gabriel", "Camila", "Diego", "Larissa", "Thiago", "Amanda",
            "Bruno", "Isabela", "Mateus", "Letícia", "Rodrigo", "Priscila", "André",
            "Beatriz", "Felipe", "Carolina"
        ];

        let last_names = vec![
            "Silva", "Santos", "Oliveira", "Souza", "Rodrigues", "Ferreira", "Alves",
            "Pereira", "Lima", "Gomes", "Costa", "Ribeiro", "Martins", "Carvalho",
            "Almeida", "Lopes", "Soares", "Fernandes", "Vieira", "Barbosa"
        ];

        for i in 1..=25 {
            let first_name = first_names.choose(&mut rng).unwrap();
            let last_name = last_names.choose(&mut rng).unwrap();
            let email = format!("{}{}@exemplo.com", 
                first_name.to_lowercase(), 
                rng.gen_range(1..=999)
            );


            let password = "senha123".as_bytes();

            // 1. Gera um "salt" aleatório e único para este usuário
            let salt = SaltString::generate(&mut OsRng);

            // 2. Cria uma instância do Argon2 com parâmetros padrão
            let argon2 = Argon2::default();

            // 3. Gera o hash da senha com o salt específico
            //    O .unwrap() é seguro aqui, pois uma falha no seeder deve parar o processo.
            let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
            // --- Fim da Lógica de Hashing ---

            let user_model = user::ActiveModel {
                id: NotSet,
                username: Set(email),
                password: Set(password_hash), // senha123
            };

            let result = user::Entity::insert(user_model).exec(db).await?;
            user_ids.push(result.last_insert_id);
        }

        Ok(user_ids)
    }

    async fn seed_tasks(db: &DatabaseConnection, user_ids: &[i32]) -> Result<Vec<i32>, DbErr> {
        let mut task_ids = Vec::new();
        let mut rng = thread_rng();

        let task_titles = vec![
            "Revisar apresentação mensal", "Enviar relatório ao cliente", "Agendar reunião de equipe",
            "Atualizar documentação do projeto", "Fazer backup dos arquivos", "Responder emails pendentes",
            "Preparar proposta comercial", "Organizar mesa de trabalho", "Estudar nova tecnologia",
            "Participar do treinamento online", "Revisar código fonte", "Testar nova funcionalidade",
            "Criar mockups da interface", "Configurar ambiente de desenvolvimento", "Implementar validações",
            "Escrever testes unitários", "Otimizar consultas do banco", "Documentar API REST",
            "Configurar pipeline CI/CD", "Monitorar performance da aplicação", "Corrigir bugs reportados",
            "Planejar sprint da próxima semana", "Fazer deploy em produção", "Revisar pull requests",
            "Atualizar dependências do projeto", "Criar dashboard de métricas", "Implementar autenticação",
            "Configurar logs da aplicação", "Fazer refatoração do código", "Preparar demo para stakeholders",
            "Estudar padrões de design", "Configurar testes automatizados", "Implementar cache Redis",
            "Otimizar imagens do site", "Configurar CDN", "Revisar arquitetura do sistema",
            "Implementar notificações push", "Criar documentação técnica", "Fazer análise de segurança",
            "Configurar monitoramento", "Implementar busca avançada", "Otimizar SEO da aplicação",
            "Configurar backup automático", "Implementar multi-tenancy", "Criar sistema de logs",
            "Fazer análise de performance", "Implementar websockets", "Configurar load balancer"
        ];

        let categories = vec![
            "Trabalho", "Estudos", "Pessoal", "Saúde", "Casa", "Finanças", 
            "Lazer", "Desenvolvimento", "Reuniões", "Projetos"
        ];

        let statuses = vec!["Pendente", "Executada", "ParcialmenteExecutada", "Adiada"];
        let priorities = vec!["Low", "Medium", "High", "Critical"];

        for _ in 1..=150 {
            let user_id = *user_ids.choose(&mut rng).unwrap();
            let title = task_titles.choose(&mut rng).unwrap();
            let category = categories.choose(&mut rng).unwrap();
            let status = statuses.choose(&mut rng).unwrap();
            let priority = priorities.choose(&mut rng).unwrap();

            let days_offset = rng.gen_range(-30..60); // Tarefas dos últimos 30 dias aos próximos 60
            let begin_date = Utc::now() + Duration::days(days_offset);

            let complete_date = if *status == "Completed" {
                Some(begin_date + Duration::days(rng.gen_range(1..7)))
            } else {
                None
            };

            let task_model = task::ActiveModel {
                id: NotSet,
                user_id: Set(user_id),
                title: Set(ToString::to_string(title)),
                description: Set(Some(format!("Descrição detalhada para: {}", title))),
                category: Set(ToString::to_string(category)),
                r#type: Set(ToString::to_string(priority)),
                status: Set(ToString::to_string(status)),
                begin_date: Set(begin_date),
                complete_date: Set(complete_date.unwrap_or_else(|| Utc::now())),
            };

            let result = task::Entity::insert(task_model).exec(db).await?;
            task_ids.push(result.last_insert_id);
        }

        Ok(task_ids)
    }

    async fn seed_goals(db: &DatabaseConnection, user_ids: &[i32]) -> Result<Vec<i32>, DbErr> {
        let mut goal_ids = Vec::new();
        let mut rng = thread_rng();

        let goal_titles = vec![
            "Aprender uma nova linguagem de programação", "Ler 12 livros técnicos este ano",
            "Fazer exercícios 4x por semana", "Economizar R$ 10.000", "Viajar para 3 países",
            "Completar curso de especialização", "Conseguir promoção no trabalho",
            "Desenvolver projeto pessoal", "Melhorar habilidades de comunicação",
            "Criar um blog técnico", "Participar de 5 eventos tech", "Aprender inglês fluente",
            "Fazer networking profissional", "Implementar hábito de meditação",
            "Organizar financas pessoais", "Aprender fotografia", "Criar startup",
            "Fazer curso de design", "Melhorar setup home office", "Criar canal no YouTube",
            "Aprender machine learning", "Fazer contribuições open source", "Criar aplicativo mobile",
            "Estudar arquitetura de software", "Fazer certificação AWS", "Aprender DevOps",
            "Criar portfólio online", "Fazer curso de UX/UI", "Estudar blockchain",
            "Criar podcast", "Aprender análise de dados", "Fazer MBA", "Estudar cibersegurança"
        ];

        let categories = vec![
            "Carreira", "Educação", "Saúde", "Finanças", "Pessoal", 
            "Tecnologia", "Criatividade", "Relacionamentos"
        ];

        let statuses = vec!["NotStarted", "InProgress", "Completed", "PartiallyCompleted", "NotCompleted"];

        for _ in 1..=120 {
            let user_id = *user_ids.choose(&mut rng).unwrap();
            let title = goal_titles.choose(&mut rng).unwrap();
            let category = categories.choose(&mut rng).unwrap();
            let status = statuses.choose(&mut rng).unwrap();

            let start_days_offset = rng.gen_range(-90..30);
            let date_start = Utc::now() + Duration::days(start_days_offset);
            
            let end_days_offset = rng.gen_range(30..365);
            let date_end = date_start + Duration::days(end_days_offset);

            let progress = match *status {
                "NotStarted" => 0,
                "InProgress" => rng.gen_range(10..80),
                "PartiallyCompleted" => rng.gen_range(50..90),
                "Completed" => 100,
                _ => rng.gen_range(0..40),
            };

            let goal_model = goal::ActiveModel {
                id: NotSet,
                user_id: Set(user_id),
                name: Set(ToString::to_string(title)),
                description: Set(Some(format!("Meta detalhada: {}", title))),
                category: Set(Some(ToString::to_string(category))),
                status: Set(ToString::to_string(status)),
                date_start: Set(date_start),
                date_end: Set(date_end),
                r#type: Set("Personal".to_string()),
            };

            let result = goal::Entity::insert(goal_model).exec(db).await?;
            goal_ids.push(result.last_insert_id);
        }

        Ok(goal_ids)
    }

    async fn seed_notes(db: &DatabaseConnection, user_ids: &[i32]) -> Result<Vec<i32>, DbErr> {
        let mut note_ids = Vec::new();
        let mut rng = thread_rng();

        let note_titles = vec![
            "Ideias para o próximo projeto", "Anotações da reunião", "Lista de compras",
            "Receita de bolo de chocolate", "Livros para ler", "Filmes recomendados",
            "Configurações do servidor", "Comandos Git úteis", "Snippets de código",
            "Contatos importantes", "Senhas temporárias", "Links interessantes",
            "Resumo do curso online", "Feedback do cliente", "Planejamento da viagem",
            "Ideias para blog posts", "Configuração do ambiente", "Bugs encontrados",
            "Melhorias sugeridas", "Análise de concorrência", "Estratégias de marketing",
            "Notas da palestra", "Resumo do livro", "Plano de estudos", "Metas mensais"
        ];

        let note_contents = vec![
            "Implementar sistema de notificações em tempo real usando WebSockets",
            "Reunião produtiva - decidimos usar React para o frontend",
            "Comprar: leite, pão, ovos, frutas, café, açúcar",
            "200g chocolate meio amargo, 3 ovos, 1 xícara açúcar...",
            "Clean Code, Design Patterns, Microservices Architecture",
            "The Matrix, Inception, Interstellar, Blade Runner 2049",
            "Nginx configurado na porta 80, SSL habilitado",
            "git rebase -i HEAD~3 para combinar commits",
            "const debounce = (fn, delay) => { let timeoutId; return (...args) => { clearTimeout(timeoutId); timeoutId = setTimeout(() => fn(...args), delay); }; }",
            "João Silva - joao@empresa.com - (11) 99999-9999",
            "Servidor prod: admin/temp123 (trocar urgente!)",
            "https://awesome-resource.com/programming-tips",
            "Arquitetura hexagonal separa domínio da infraestrutura",
            "Cliente gostou da interface, pediu para melhorar performance",
            "Voo: 15/12 às 14h30, Hotel: Reserve Inn, Roteiro: museus",
            "Como implementar cache distribuído com Redis",
            "Docker compose configurado, PostgreSQL rodando na porta 5432",
            "Bug: formulário não valida email corretamente na linha 45",
            "Adicionar busca por filtros, melhorar UX do carrinho",
            "Concorrente A: preço alto mas qualidade, Concorrente B: foco em velocidade",
            "SEO: otimizar meta tags, criar sitemap, melhorar velocidade",
            "Palestrante falou sobre SOLID, Clean Architecture e TDD",
            "Capítulo 3: dependency injection facilita testes unitários",
            "Segunda: React hooks, Terça: Redux, Quarta: TypeScript",
            "Janeiro: 5 livros, Fevereiro: curso AWS, Março: projeto pessoal"
        ];

        for _ in 1..=100 {
            let user_id = *user_ids.choose(&mut rng).unwrap();
            let title = note_titles.choose(&mut rng).unwrap();
            let content = note_contents.choose(&mut rng).unwrap();

            let days_offset = rng.gen_range(-60..0);
            let created_at = Utc::now() + Duration::days(days_offset);

            let note_model = notes::ActiveModel {
                id: NotSet,
                text: Set(format!("{}: {}", title, content)),
                bolsonar: Set(user_id.to_string()),
                created_at: Set(created_at),
            };

            let result = notes::Entity::insert(note_model).exec(db).await?;
            note_ids.push(result.last_insert_id);
        }

        Ok(note_ids)
    }

    async fn seed_reminders(db: &DatabaseConnection, user_ids: &[i32]) -> Result<Vec<i32>, DbErr> {
        let mut reminder_ids = Vec::new();
        let mut rng = thread_rng();

        let reminder_titles = vec![
            "Reunião com cliente", "Entrega do projeto", "Pagamento da fatura",
            "Consulta médica", "Aniversário da Maria", "Renovar CNH", "Backup dos dados",
            "Call com equipe remota", "Apresentação quarterly", "Review de código",
            "Dentista às 14h", "Comprar presente", "Enviar proposta", "Fazer exercícios",
            "Estudar para certificação", "Ligar para fornecedor", "Agendar manutenção",
            "Renovar contrato", "Fazer exames", "Revisar orçamento", "Deploy em produção",
            "Reunião de planejamento", "Treinamento de segurança", "Entrevista técnica",
            "Workshop de UX"
        ];

        let descriptions = vec![
            "Discussão sobre próximas funcionalidades e timeline do projeto",
            "Entrega final do MVP com todas as funcionalidades implementadas",
            "Vencimento da fatura de hosting e domínio - não esquecer!",
            "Checkup anual com Dr. Silva - levar exames anteriores",
            "Comprar bolo e organizar surpresa para a colega de trabalho",
            "Carteira vence em 2 meses, agendar no DETRAN online",
            "Backup semanal dos dados do servidor e banco de dados",
            "Daily standup com time de desenvolvimento remoto",
            "Apresentar métricas do trimestre para diretoria",
            "Code review da nova funcionalidade de pagamentos",
            "Limpeza dental de rotina - consultório no centro",
            "Aniversário do João semana que vem - pensar em algo legal",
            "Enviar proposta técnica e comercial para cliente novo",
            "Cardio 30min + musculação - manter rotina saudável",
            "Estudar AWS Solutions Architect - prova em 1 mês",
            "Negociar condições de pagamento com novo fornecedor",
            "Manutenção preventiva do ar condicionado do escritório",
            "Contrato de internet vence no final do mês",
            "Exames de sangue e check-up cardiológico anual",
            "Revisar gastos mensais e planejar próximo trimestre"
        ];

        for _ in 1..=110 {
            let user_id = *user_ids.choose(&mut rng).unwrap();
            let title = reminder_titles.choose(&mut rng).unwrap();
            let description = descriptions.choose(&mut rng).unwrap();

            let days_offset = rng.gen_range(-10..90);
            let date_end = Utc::now() + Duration::days(days_offset);

            let reminder_model = reminder::ActiveModel {
                id: NotSet,
                user_id: Set(user_id),
                name: Set(ToString::to_string(title)),
                date_end: Set(date_end),
                category: Set("Geral".to_string()),
            };

            let result = reminder::Entity::insert(reminder_model).exec(db).await?;
            reminder_ids.push(result.last_insert_id);
        }

        Ok(reminder_ids)
    }
}