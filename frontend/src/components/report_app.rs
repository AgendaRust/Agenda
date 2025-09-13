use yew::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};
use crate::components::bar_chart::BarChart; // Ajuste o caminho conforme sua estrutura

#[derive(Properties, PartialEq)]
pub struct ReportAppProps {
    pub visible: bool,
    pub on_close: Callback<()>,
}

#[function_component(ReportApp)]
pub fn report_app(props: &ReportAppProps) -> Html {
    let selected_report_type = use_state(|| Option::<String>::None);

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

    // Função auxiliar para renderizar o resumo com base no tipo selecionado
    let render_report_summary = {
        let selected_type = (*selected_report_type).clone();
        match selected_type.as_deref() {
            Some("Semanal") => html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Semanal de Tarefas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana Analisada:" }</span>
                            <span class="summary-value">{ "Semana 37" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "12" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Dia mais Produtivo:" }</span>
                            <span class="summary-value">{ "Quarta-feira" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Tempo Médio Diário:" }</span>
                            <span class="summary-value">{ "3h 30m" }</span>
                        </div>
                    </div>
                <h3>{ "Resumo Semanal de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana Analisada:" }</span>
                            <span class="summary-value">{ "Semana 37" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "12" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Dia mais Produtivo:" }</span>
                            <span class="summary-value">{ "Quarta-feira" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Tempo Médio Diário:" }</span>
                            <span class="summary-value">{ "3h 30m" }</span>
                        </div>
                    </div>
                </div>
            },
            Some("Mensal") => html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Mensal de Tarefas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Mês Analisado:" }</span>
                            <span class="summary-value">{ "Setembro" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "42" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana mais Produtiva:" }</span>
                            <span class="summary-value">{ "Segunda semana" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Categoria Dominante:" }</span>
                            <span class="summary-value">{ "Projetos" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Taxa de Conclusão:" }</span>
                            <span class="summary-value">{ "78%" }</span>
                        </div>
                    </div>
                 <h3>{ "Resumo Semanal de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana Analisada:" }</span>
                            <span class="summary-value">{ "Semana 37" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "12" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Dia mais Produtivo:" }</span>
                            <span class="summary-value">{ "Quarta-feira" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Tempo Médio Diário:" }</span>
                            <span class="summary-value">{ "3h 30m" }</span>
                        </div>
                    </div>
                </div>
            },
            Some("Anual") => html! {
                <div class="report-summary-section">
                    <h3>{ "Resumo Anual de Tarefas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Ano Analisado:" }</span>
                            <span class="summary-value">{ "2023" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "486" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Mês mais Produtivo:" }</span>
                            <span class="summary-value">{ "Março" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Média Mensal:" }</span>
                            <span class="summary-value">{ "40.5 tarefas" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Progresso Anual:" }</span>
                            <span class="summary-value">{ "92%" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Tendência:" }</span>
                            <span class="summary-value">{ "Crescente" }</span>
                        </div>
                    </div>
                 <h3>{ "Resumo Semanal de Metas" }</h3>
                    <div class="summary-data">
                        <div class="summary-item">
                            <span class="summary-label">{ "Semana Analisada:" }</span>
                            <span class="summary-value">{ "Semana 37" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Total de Tarefas:" }</span>
                            <span class="summary-value">{ "12" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Dia mais Produtivo:" }</span>
                            <span class="summary-value">{ "Quarta-feira" }</span>
                        </div>
                        <div class="summary-item">
                            <span class="summary-label">{ "Tempo Médio Diário:" }</span>
                            <span class="summary-value">{ "3h 30m" }</span>
                        </div>
                    </div>
                </div>
            },
            _ => html! {
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

    html! {
        if !props.visible {
            <div></div>
        } else {
            <div class="report-app">
                <div class="report-header">
                    <h1 class="report-title">{ "Relatório - Windows 98" }</h1>
                    <div class="report-header-controls">
                        <button class="control-button minimize-btn" type="button" onclick={
                            let on_close = props.on_close.clone();
                            Callback::from(move |_: MouseEvent| {
                                on_close.emit(());
                            })
                        }></button>
                        <button class="control-button maximize-btn" type="button"></button>
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
                        <div class="report-view">
                                <h2>{ "Visualização de Relatório" }</h2>
                                    <div class="chart-container">
                                        <BarChart
                                            data={vec![
                                                ("Concluídas".to_string(), 10 as f64),
                                                ("Pendentes".to_string(), 5 as f64),
                                                ("Adiadas".to_string(), 4 as f64),
                                            ]}
                                            title="Tarefas"
                                            width={600}
                                            height={400}
                                        />
                                    </div>
                                    <div class="chart-container">
                                        <BarChart
                                            data={vec![
                                                ("Concluídas".to_string(), 8.5),
                                                ("Pendentes".to_string(), 6.2),
                                                ("Adiadas".to_string(), 9.1),
                                            ]}
                                            title="Metas"
                                            width={600}
                                            height={400}
                                        />
                                    </div>
                            </div>
                                                <div class="report-summary">
                            {
                                if (*selected_report_type).is_some() {
                                    html! {
                                        <>
                                            <div class="tasks-summary">
                                                <h3>{ "Tarefas" }</h3>
                                                <div class="summary-stats">
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Concluídas:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Adiadas:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Pendentes:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="goals-summary">
                                                <h3>{ "Metas" }</h3>
                                                <div class="summary-stats">
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Total de Metas:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Alcançadas:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
                                                    </div>
                                                    <div class="stat-item">
                                                        <span class="stat-label">{ "Em Progresso:" }</span>
                                                        <span class="stat-value">{ "0" }</span>
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
                                        _ => html! {
                                            <>
                                                <label for="report-date">{ "Selecione a data:" }</label>
                                                <input type="date" id="report-date" class="date-input" />
                                            </>
                                        }
                                    }
                                }
                                <div class="dialog-buttons">
                                    <button class="btn-primary">{ "Gerar Relatório" }</button>
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
