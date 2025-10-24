use yew::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement, console};
use crate::components::bar_chart::BarChart;
use crate::services::report_service::{ReportService};
use crate::types::report::{StatsYearResponse, StatsMonthResponse, StatsWeekResponse};

#[derive(Properties, PartialEq)]
pub struct ReportAppProps {
    pub visible: bool,
    pub on_close: Callback<()>,
}

#[function_component(ReportApp)]
pub fn report_app(props: &ReportAppProps) -> Html {
    let selected_report_type = use_state(|| Option::<String>::None);
    let year_stats = use_state(|| Option::<StatsYearResponse>::None);
    let month_stats = use_state(|| Option::<StatsMonthResponse>::None);
    let week_stats = use_state(|| Option::<StatsWeekResponse>::None);
    let is_loading = use_state(|| false);

    let on_report_type_select = {
        let selected_report_type = selected_report_type.clone();
        Callback::from(move |report_type: String| {
            selected_report_type.set(Some(report_type));
        })
    };

    let on_date_dialog_close = {
        let selected_report_type = selected_report_type.clone();
        Callback::from(move |_| {
            selected_report_type.set(None);
        })
    };

    let on_generate_report = {
        let selected_report_type = selected_report_type.clone();
        let year_stats = year_stats.clone();
        let month_stats = month_stats.clone();
        let week_stats = week_stats.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_: MouseEvent| {
            let report_type = (*selected_report_type).clone();
            let year_stats = year_stats.clone();
            let month_stats = month_stats.clone();
            let week_stats = week_stats.clone();
            let is_loading = is_loading.clone();
            let selected_report_type_close = selected_report_type.clone();

            if let Some(report_type) = report_type {
                is_loading.set(true);

                year_stats.set(None);
                month_stats.set(None);
                week_stats.set(None);
                is_loading.set(true);

                match report_type.as_str() {
                    "Anual" => {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let select_element = document.get_element_by_id("report-year")
                            .unwrap()
                            .dyn_into::<HtmlSelectElement>()
                            .unwrap();
                        let year: i32 = select_element.value().parse().unwrap_or(2024);

                        wasm_bindgen_futures::spawn_local(async move {
                            match ReportService::fetch_year_stats(year).await {
                                Ok(stats) => {
                                    year_stats.set(Some(stats));
                                    console::log_1(&"Estatísticas anuais carregadas com sucesso".into());
                                },
                                Err(e) => {
                                    console::error_1(&format!("Erro ao carregar estatísticas anuais: {}", e).into());
                                    year_stats.set(Some(StatsYearResponse { year, ..Default::default() }));
                                }
                            }
                            is_loading.set(false);
                            selected_report_type_close.set(None);
                        });
                    },
                    "Mensal" => {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let input_element = document.get_element_by_id("report-month")
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap();
                        let date_value = input_element.value();

                        // Parse manual do formato "YYYY-MM"
                        if !date_value.is_empty() {
                            let parts: Vec<&str> = date_value.split('-').collect();
                            if parts.len() == 2 {
                                if let (Ok(year), Ok(month)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                                    console::log_1(&format!("Enviando para API: year={}, month={}", year, month).into());
                                    wasm_bindgen_futures::spawn_local(async move {
                                        match ReportService::fetch_month_stats(year, month).await {
                                            Ok(stats) => {
                                                month_stats.set(Some(stats));
                                                console::log_1(&"Estatísticas mensais carregadas com sucesso".into());
                                            },
                                            Err(e) => {
                                                console::error_1(&format!("Erro ao carregar estatísticas mensais: {}", e).into());
                                            }
                                        }
                                        is_loading.set(false);
                                        selected_report_type_close.set(None);
                                    });
                                } else {
                                    console::error_1(&"Erro no parse dos valores do mês".into());
                                    is_loading.set(false);
                                }
                            } else {
                                console::error_1(&"Formato de data inválido para mês".into());
                                is_loading.set(false);
                            }
                        } else {
                            console::error_1(&"Nenhum mês selecionado".into());
                            is_loading.set(false);
                        }
                    },
                    "Semanal" => {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let input_element = document.get_element_by_id("report-week")
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap();
                        let date_value = input_element.value();

                        // Parse manual do formato "YYYY-WXX"
                        if !date_value.is_empty() {
                            let parts: Vec<&str> = date_value.split("-W").collect();
                            if parts.len() == 2 {
                                if let (Ok(year), Ok(week)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                                    console::log_1(&format!("Enviando para API: year={}, week={}", year, week).into());
                                    wasm_bindgen_futures::spawn_local(async move {
                                        match ReportService::fetch_week_stats(year, week).await {
                                            Ok(stats) => {
                                                week_stats.set(Some(stats));
                                                console::log_1(&"Estatísticas semanais carregadas com sucesso".into());
                                            },
                                            Err(e) => {
                                                console::error_1(&format!("Erro ao carregar estatísticas semanais: {}", e).into());
                                            }
                                        }
                                        is_loading.set(false);
                                        selected_report_type_close.set(None);
                                    });
                                } else {
                                    console::error_1(&"Erro no parse dos valores da semana".into());
                                    is_loading.set(false);
                                }
                            } else {
                                console::error_1(&"Formato de data inválido para semana".into());
                                is_loading.set(false);
                            }
                        } else {
                            console::error_1(&"Nenhuma semana selecionada".into());
                            is_loading.set(false);
                        }
                    },

                    _ => {
                        is_loading.set(false);
                    }
                }
            }
        })
    };

    // Função auxiliar para renderizar o resumo com dados reais
    let render_report_summary = {
        let year_stats = (*year_stats).clone();
        let month_stats = (*month_stats).clone();
        let week_stats = (*week_stats).clone();

        if let Some(stats) = year_stats {
            html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Anual de Tarefas" }</h3>
                    <div  class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Ano Analisado:" }</span>
                            <span class="summary-value">{ stats.year }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ stats.total_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_tasks) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Mês:" }</span>
                            <span class="summary-value">{ &stats.most_productive_month_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Semana:" }</span>
                            <span class="summary-value">{ &stats.most_productive_week_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_tasks }</span>
                        </div>
                    </div>
                    <h3>{ "Resumo Anual de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Metas:" }</span>
                            <span class="summary-value">{ stats.total_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_goals) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Mês:" }</span>
                            <span class="summary-value">{ &stats.most_productive_month_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Semana:" }</span>
                            <span class="summary-value">{ &stats.most_productive_week_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_goals }</span>
                        </div>
                    </div>
                </div>
            }
        } else if let Some(stats) = month_stats {
            html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Mensal de Tarefas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Mês Analisado:" }</span>
                            <span class="summary-value">{ format!("{}/{}", stats.month, stats.year) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ stats.total_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_tasks) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Semana:" }</span>
                            <span class="summary-value">{ &stats.most_productive_week_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_tasks }</span>
                        </div>
                    </div>
                    <h3>{ "Resumo Mensal de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Metas:" }</span>
                            <span class="summary-value">{ stats.total_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_goals) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Semana:" }</span>
                            <span class="summary-value">{ &stats.most_productive_week_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_goals }</span>
                        </div>
                    </div>
                </div>
            }
        } else if let Some(stats) = week_stats {
            html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Semanal de Tarefas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana Analisada:" }</span>
                            <span class="summary-value">{ format!("Semana {} de {}", stats.week, stats.year) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ stats.total_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_tasks) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Dia:" }</span>
                            <span class="summary-value">{ &stats.most_productive_day_tasks }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_tasks }</span>
                        </div>
                    </div>
                     <h3>{ "Resumo Mensal de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Metas:" }</span>
                            <span class="summary-value">{ stats.total_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Porcentagem:" }</span>
                            <span class="summary-value">{ format!("{:.1}%", stats.percentage_goals) }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Horário:" }</span>
                            <span class="summary-value">{ &stats.most_productive_shift_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Categoria:" }</span>
                            <span class="summary-value">{ &stats.most_used_category_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Melhor Dia:" }</span>
                            <span class="summary-value">{ &stats.most_productive_day_goals }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Resumo geral:" }</span>
                            <span class="summary-value">{ &stats.classification_goals }</span>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="report-summary-section">
                    <h3>{ "Estatísticas do Relatório" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Selecione um tipo de relatório" }</span>
                        </div>
                    </div>
                </div>
            }
        }
    };

    let render_dynamic_summary = {
        let year_stats = (*year_stats).clone();
        let month_stats = (*month_stats).clone();
        let week_stats = (*week_stats).clone();

        if let Some(stats) = year_stats {
            html! {
                <>
                    <div class="tasks-summary">
                        <h3>{ "Tarefas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Pendentes:" }</span>
                                <span class="stat-value">{ stats.pendent_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Adiadas:" }</span>
                                <span class="stat-value">{ stats.delayed_tasks }</span>
                            </div>
                        </div>
                    </div>
                    <div class="goals-summary">
                        <h3>{ "Metas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Em andamento:" }</span>
                                <span class="stat-value">{ stats.pendent_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Canceladas:" }</span>
                                <span class="stat-value">{ stats.delayed_goals }</span>
                            </div>
                        </div>
                    </div>
                </>
            }
        } else if let Some(stats) = month_stats {
            html! {
                <>
                    <div class="tasks-summary">
                        <h3>{ "Tarefas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Pendentes:" }</span>
                                <span class="stat-value">{ stats.pendent_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Adiadas:" }</span>
                                <span class="stat-value">{ stats.delayed_tasks }</span>
                            </div>
                        </div>
                    </div>
                    <div class="goals-summary">
                        <h3>{ "Metas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Em andamento:" }</span>
                                <span class="stat-value">{ stats.pendent_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Canceladas:" }</span>
                                <span class="stat-value">{ stats.delayed_goals }</span>
                            </div>
                        </div>
                    </div>
                </>
            }
        } else if let Some(stats) = week_stats {
            html! {
                <>
                    <div class="tasks-summary">
                        <h3>{ "Tarefas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Pendentes:" }</span>
                                <span class="stat-value">{ stats.pendent_tasks }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Adiadas:" }</span>
                                <span class="stat-value">{ stats.delayed_tasks }</span>
                            </div>
                        </div>
                    </div>
                    <div class="goals-summary">
                        <h3>{ "Metas" }</h3>
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-label">{ "Concluídas:" }</span>
                                <span class="stat-value">{ stats.executed_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Em andamento:" }</span>
                                <span class="stat-value">{ stats.pendent_goals }</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{ "Canceladas:" }</span>
                                <span class="stat-value">{ stats.delayed_goals }</span>
                            </div>
                        </div>
                    </div>
                </>
            }
        } else {
            html! {
                <div class="empty-summary">
                    <h3>{ "Selecione um tipo de relatório para visualizar o resumo." }</h3>
                </div>
            }
        }
    };

    // Função para obter dados dos gráficos
    let get_chart_data = || {
        if let Some(stats) = (*year_stats).clone() {
            (
                vec![
                    ("Concluídas".to_string(), stats.executed_tasks as f64),
                    ("Pendentes".to_string(), stats.pendent_tasks as f64),
                    ("Adiadas".to_string(), stats.delayed_tasks as f64),
                ],
                vec![
                    ("Concluídas".to_string(), stats.executed_goals as f64),
                    ("Em andamento".to_string(), stats.pendent_goals as f64),
                    ("Canceladas".to_string(), stats.delayed_goals as f64),
                ]
            )
        } else if let Some(stats) = (*month_stats).clone() {
            (
                vec![
                    ("Concluídas".to_string(), stats.executed_tasks as f64),
                    ("Pendentes".to_string(), stats.pendent_tasks as f64),
                    ("Adiadas".to_string(), stats.delayed_tasks as f64),
                ],
                vec![
                    ("Concluídas".to_string(), stats.executed_goals as f64),
                    ("Em andamento".to_string(), stats.pendent_goals as f64),
                    ("Canceladas".to_string(), stats.delayed_goals as f64),
                ]
            )
        } else if let Some(stats) = (*week_stats).clone() {
            (
                vec![
                    ("Concluídas".to_string(), stats.executed_tasks as f64),
                    ("Pendentes".to_string(), stats.pendent_tasks as f64),
                    ("Adiadas".to_string(), stats.delayed_tasks as f64),
                ],
                vec![
                    ("Concluídas".to_string(), stats.executed_goals as f64),
                    ("Em andamento".to_string(), stats.pendent_goals as f64),
                    ("Canceladas".to_string(), stats.delayed_goals as f64),
                ]
            )
        } else {
            (
                vec![("Sem dados".to_string(), 0.0)],
                vec![("Sem dados".to_string(), 0.0)]
            )
        }
    };


    html! {
        if !props.visible {
            <div></div>
        } else {
            <div class="report-app">
                <div class="report-header">
                    <h1 class="report-title">{ "Relatório - Windows 98" }</h1>
                    <div class="report-header-controls">
                        <button class="control-button close-btn" type="button" onclick={
                            let on_close = props.on_close.clone();
                            Callback::from(move |_: MouseEvent| {
                                on_close.emit(());
                            })
                        }></button>
                    </div>
                </div>
                <div class="report-content">
                    <div class="report-sidebar">
                        <h3>{ "Tipos de Relatório" }</h3>
                        <ul class="report-menu">
                            <li><button class="menu-item" onclick={
                                let callback = on_report_type_select.clone();
                                Callback::from(move |_: MouseEvent| {
                                    callback.emit("Semanal".to_string());
                                })
                            }>{ "Semanal" }</button></li>
                            <li><button class="menu-item" onclick={
                                let callback = on_report_type_select.clone();
                                Callback::from(move |_: MouseEvent| {
                                    callback.emit("Mensal".to_string());
                                })
                            }>{ "Mensal" }</button></li>
                            <li><button class="menu-item" onclick={
                                let callback = on_report_type_select.clone();
                                Callback::from(move |_: MouseEvent| {
                                    callback.emit("Anual".to_string());
                                })
                            }>{ "Anual" }</button></li>
                        </ul>
                        { render_report_summary }
                    </div>
                    <div class="report-main">
                        <div class="report-summary">
                            { render_dynamic_summary }
                        </div>
                        <div class="report-view">
                            <h2>{ "Visualização de Relatório" }</h2>
                            {
                                if (*year_stats).is_some() || (*month_stats).is_some() || (*week_stats).is_some() {
                                    let (tasks_data, goals_data) = get_chart_data();
                                    html! {
                                        <>
                                            <div class="chart-container">
                                                <BarChart
                                                    data={tasks_data}
                                                    title="Tarefas"
                                                    width={600}
                                                    height={400}
                                                />
                                            </div>
                                            <div class="chart-container">
                                                <BarChart
                                                    data={goals_data}
                                                    title="Metas"
                                                    width={600}
                                                    height={400}
                                                />
                                            </div>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <div class="select-message">
                                            <p>{ if *is_loading { "Carregando..." } else { "Selecione um tipo de relatório para visualizar os gráficos" } }</p>
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>

                // Caixa de diálogo para seleção de data
                if let Some(report_type) = (*selected_report_type).clone() {
                    <div class="date-dialog-overlay">
                        <div class="date-dialog">
                            <div class="date-dialog-header">
                                <h3>{ format!("Relatório {}", report_type) }</h3>
                                <button class="close-dialog-btn" onclick={on_date_dialog_close.clone()}>{ "×" }</button>
                            </div>
                            <div class="date-dialog-content">
                                {
                                    match report_type.as_str() {
                                        "Anual" => html! {
                                            <>
                                                <label for="report-year">{ "Selecione o ano:" }</label>
                                                <select id="report-year" class="date-select">
                                                    { for (2000..=2025).map(|year| html! {
                                                        <option value={year.to_string()}>{ year }</option>
                                                    }) }
                                                </select>
                                            </>
                                        },
                                        "Mensal" => html! {
                                            <>
                                                <label for="report-month">{ "Selecione o mês:" }</label>
                                                <input type="month" id="report-month" class="date-input" />
                                            </>
                                        },
                                        "Semanal" => html! {
                                            <>
                                                <label for="report-week">{ "Selecione a semana (data inicial):" }</label>
                                                <input type="week" id="report-week" class="date-input" />
                                            </>
                                        },
                                        _ => html! { <></> }
                                    }
                                }
                                <div class="dialog-buttons">
                                    <button class="btn-primary" onclick={on_generate_report} disabled={*is_loading}>
                                        { if *is_loading { "Carregando..." } else { "Gerar Relatório" } }
                                    </button>
                                    <button class="btn-secondary" onclick={on_date_dialog_close}>{ "Cancelar" }</button>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            </div>
        }
    }
}
