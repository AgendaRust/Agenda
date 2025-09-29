use gloo_net::http::Request;
use web_sys::console;
use super::auth::get_token;
use crate::config::get_api_url;
use crate::types::report::{StatsYearResponse, StatsMonthResponse, StatsWeekResponse};
pub struct ReportService;

impl ReportService {



    /// Busca estatísticas anuais
    pub async fn fetch_year_stats(year: i32) -> Result<StatsYearResponse, String> {

        let url = format!("{}/reports/stats/year/{}", get_api_url(), year);

        console::log_1(&format!("Buscando estatísticas do ano: {}", year).into());
        let token = get_token();
        if token.token.is_empty() {
            return Err("No authentication token found".to_string());
        }


        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", token.token))
            .send()
            .await
            .map_err(|e| format!("Erro na requisição: {:?}", e))?;

        if response.ok() {
            let stats = response
                .json::<StatsYearResponse>()
                .await
                .map_err(|e| format!("Erro ao processar resposta: {:?}", e))?;

            console::log_1(&format!("Estatísticas do ano recebidas com sucesso").into());
            Ok(stats)
        } else {
            let status = response.status();
            console::error_1(&format!("Erro na resposta: status {}", status).into());

            // Se houver erro, retorna valores default
            Ok(StatsYearResponse {
                year,
                ..Default::default()
            })
        }
    }

    /// Busca estatísticas mensais
    pub async fn fetch_month_stats(year: i32, month: i32) -> Result<StatsMonthResponse, String> {
        let token = get_token();
        if token.token.is_empty() {
            return Err("No authentication token found".to_string());
        }


        let url = format!("{}/reports/stats/month/{}/{}", get_api_url(), year, month);

        console::log_1(&format!("Buscando estatísticas do mês: {}/{}", month, year).into());

        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", token.token))
            .send()
            .await
            .map_err(|e| format!("Erro na requisição: {:?}", e))?;

        if response.ok() {
            let stats = response
                .json::<StatsMonthResponse>()
                .await
                .map_err(|e| format!("Erro ao processar resposta: {:?}", e))?;

            console::log_1(&format!("Estatísticas do mês recebidas com sucesso").into());
            Ok(stats)
        } else {
            let status = response.status();
            console::error_1(&format!("Erro na resposta: status {}", status).into());

            // Se houver erro, retorna valores default
            Ok(StatsMonthResponse {
                year,
                month,
                ..Default::default()
            })
        }
    }

    /// Busca estatísticas semanais
    pub async fn fetch_week_stats(year: i32, week: i32) -> Result<StatsWeekResponse, String> {
        let token = get_token();
        if token.token.is_empty() {
            return Err("No authentication token found".to_string());
        }

        let url = format!("{}/reports/stats/week/{}/{}", get_api_url(), year, week);

        console::log_1(&format!("Buscando estatísticas da semana: {}/{}", week, year).into());

        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", token.token))
            .send()
            .await
            .map_err(|e| format!("Erro na requisição: {:?}", e))?;

        if response.ok() {
            let stats = response
                .json::<StatsWeekResponse>()
                .await
                .map_err(|e| format!("Erro ao processar resposta: {:?}", e))?;

            console::log_1(&format!("Estatísticas da semana recebidas com sucesso").into());
            Ok(stats)
        } else {
            let status = response.status();
            console::error_1(&format!("Erro na resposta: status {}", status).into());

            // Se houver erro, retorna valores default
            Ok(StatsWeekResponse {
                year,
                week,
                ..Default::default()
            })
        }
    }

    // Função auxiliar para calcular o número da semana a partir de uma data
    // pub fn calculate_week_number(date: &str) -> Result<(i32, i32), String> {
    //     // Formato esperado: "2024-W15" (ISO week date)
    //     if date.len() >= 8 && date.contains("-W") {
    //         let parts: Vec<&str> = date.split("-W").collect();
    //         if parts.len() == 2 {
    //             let year = parts[0].parse::<i32>()
    //                 .map_err(|_| "Ano inválido".to_string())?;
    //             let week = parts[1].parse::<i32>()
    //                 .map_err(|_| "Semana inválida".to_string())?;
    //             return Ok((year, week));
    //         }
    //     }
    //     Err("Formato de data inválido".to_string())
    // }

    // /// Função auxiliar para extrair ano e mês de uma string de data
    // pub fn parse_month_input(date: &str) -> Result<(i32, i32), String> {
    //     // Formato esperado: "2024-03"
    //     let parts: Vec<&str> = date.split('-').collect();
    //     if parts.len() == 2 {
    //         let year = parts[0].parse::<i32>()
    //             .map_err(|_| "Ano inválido".to_string())?;
    //         let month = parts[1].parse::<i32>()
    //             .map_err(|_| "Mês inválido".to_string())?;

    //         if month >= 1 && month <= 12 {
    //             return Ok((year, month));
    //         }
    //     }
    //     Err("Formato de data inválido".to_string())
    // }
}

// Enum para representar os diferentes tipos de estatísticas
// #[derive(Debug, Clone, PartialEq)]
// pub enum StatsResponse {
//     Year(StatsYearResponse),
//     Month(StatsMonthResponse),
//     Week(StatsWeekResponse),
// }

// impl StatsResponse {
//     /// Retorna os dados de tarefas para uso em gráficos
//     pub fn get_tasks_chart_data(&self) -> Vec<(String, f64)> {
//         match self {
//             StatsResponse::Year(stats) => vec![
//                 ("Concluídas".to_string(), stats.executed_tasks as f64),
//                 ("Pendentes".to_string(), stats.pendent_tasks as f64),
//                 ("Adiadas".to_string(), stats.delayed_tasks as f64),
//             ],
//             StatsResponse::Month(stats) => vec![
//                 ("Concluídas".to_string(), stats.executed_tasks as f64),
//                 ("Pendentes".to_string(), stats.pendent_tasks as f64),
//                 ("Adiadas".to_string(), stats.delayed_tasks as f64),
//             ],
//             StatsResponse::Week(stats) => vec![
//                 ("Concluídas".to_string(), stats.executed_tasks as f64),
//                 ("Pendentes".to_string(), stats.pendent_tasks as f64),
//                 ("Adiadas".to_string(), stats.delayed_tasks as f64),
//             ],
//         }
//     }

//     /// Retorna os dados de metas para uso em gráficos
//     pub fn get_goals_chart_data(&self) -> Vec<(String, f64)> {
//         match self {
//             StatsResponse::Year(stats) => vec![
//                 ("Executadas".to_string(), stats.executed_goals as f64),
//                 ("Pendentes".to_string(), stats.pendent_goals as f64),
//                 ("Adiadas".to_string(), stats.delayed_goals as f64),
//             ],
//             StatsResponse::Month(stats) => vec![
//                 ("Executadas".to_string(), stats.executed_goals as f64),
//                 ("Pendentes".to_string(), stats.pendent_goals as f64),
//                 ("Adiadas".to_string(), stats.delayed_goals as f64),
//             ],
//             StatsResponse::Week(stats) => vec![
//                 ("Executadas".to_string(), stats.executed_goals as f64),
//                 ("Pendentes".to_string(), stats.pendent_goals as f64),
//                 ("Adiadas".to_string(), stats.delayed_goals as f64),
//             ],
//         }
//     }
// }