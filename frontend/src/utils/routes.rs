use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
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
        Route::Home => html! {
            <Home></Home>
        },
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
