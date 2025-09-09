use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct ReportAppProps {
    pub visible: bool,
    pub on_close: Callback<()>,
}

#[function_component(ReportApp)]
pub fn report_app(props: &ReportAppProps) -> Html {
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
                            <li><button class="menu-item">{ "Semanal" }</button></li>
                            <li><button class="menu-item">{ "Mensal" }</button></li>
                            <li><button class="menu-item">{ "Anual" }</button></li>
                        </ul>
                    </div>
                    <div class="report-main">
                        <div class="report-view">
                            <h2>{ "Visualização de Relatório" }</h2>
                            <p>{ "Selecione um tipo de relatório no menu lateral para visualizar os dados." }</p>
                            // Área principal onde os relatórios serão exibidos
                        </div>
                        <div class="report-summary">
                            <div class="tasks-summary">
                                <h3>{ "Tarefas" }</h3>
                                <div class="summary-stats">
                                    <div class="stat-item">
                                        <span class="stat-label">{ "Total de Tarefas:" }</span>
                                        <span class="stat-value">{ "0" }</span>
                                    </div>
                                    <div class="stat-item">
                                        <span class="stat-label">{ "Concluídas:" }</span>
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
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
