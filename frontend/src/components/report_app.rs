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
                    <h1 class="report-title">{ "Relatórios - Windows 98" }</h1>
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
                    <h2>{ "Relatórios" }</h2>
                    <p>{ "Aqui você poderá visualizar relatórios de tarefas e lembretes." }</p>
                    // Conteúdo dos relatórios será implementado aqui
                </div>
            </div>
        }
    }
}