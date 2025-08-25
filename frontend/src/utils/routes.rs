use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::services::auth::{self, Token};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            let token: Token = auth::get_token();
            web_sys::console::log_1(&format!("{}", token.token).into());
            if token.token.is_empty() {
                return {
                    html! {
                        <Login/>
                    }
                };
            }
            html! {
                <Home></Home>
            }
        }
        Route::Login => html! {
            <Login></Login>
        },
        Route::Register => html! {
            <Register/>
        },
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
