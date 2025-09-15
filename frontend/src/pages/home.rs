use crate::components::{calendar_app::CalendarApp, report_app::ReportApp, taskbar::Taskbar};
use yew::{prelude::*, Callback};

#[function_component(Home)]
pub fn home_component() -> Html {
    let calendar_visible = use_state(|| true);
    let report_visible = use_state(|| false);

    let toggle_calendar = {
        let calendar_visible = calendar_visible.clone();
        let report_visible = report_visible.clone();
        Callback::from(move |_: ()| {
            if !*calendar_visible {
                report_visible.set(false); // Fecha relatórios se estiver abrindo calendário
            }
            calendar_visible.set(!*calendar_visible);
        })
    };

    let toggle_report = {
        let report_visible = report_visible.clone();
        let calendar_visible = calendar_visible.clone();
        Callback::from(move |_: ()| {
            if !*report_visible {
                calendar_visible.set(false); // Fecha calendário se estiver abrindo relatórios
            }
            report_visible.set(!*report_visible);
        })
    };

    let close_calendar = {
        let calendar_visible = calendar_visible.clone();
        Callback::from(move |_: ()| {
            calendar_visible.set(false);
        })
    };

    let close_report = {
        let report_visible = report_visible.clone();
        Callback::from(move |_: ()| {
            report_visible.set(false);
        })
    };

    html! {
        <div class="app-container">
            <CalendarApp visible={*calendar_visible} on_close={close_calendar} />
            <ReportApp visible={*report_visible} on_close={close_report} />
            <Taskbar
                on_calendar_toggle={toggle_calendar}
                calendar_visible={*calendar_visible}
                on_report_toggle={toggle_report}
                report_visible={*report_visible}
            />
        </div>
    }
}
