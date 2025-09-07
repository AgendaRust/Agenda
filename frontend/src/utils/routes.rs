use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::services::auth::{self, verify_token, Token};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::reminder::ReminderPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,

     #[at("/reminder")]
     Reminder,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            let token: Token = auth::get_token();
            web_sys::console::log_1(&format!("{}", token.token).into());
            // nao tem token então vai para registrar
            if token.token.is_empty() {
                return {
                    html! {
                        <Register/>
                    }
                };
            }
            // possui token mas está invalido, entao vai para login
            if !verify_token(&token) {
                return {
                    html! {
                        <Login/>
                    }
                };
            }
            html! {
                <Home/>
            }
        }
        Route::Login => html! {
            <Login/>
        },
        Route::Register => html! {
            <Register/>
        },
         Route::Reminder => html! { <ReminderPage/> }
    }
}

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
