use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::utils::routes::Route;

use crate::services::auth::{self, AuthStruct, LoginResult};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();

    let username = use_state(String::new);
    let password = use_state(String::new);
    let login_pressed = use_state(|| false);

    let onclick = {
        let username = (*username).clone();
        let password = (*password).clone();
        let navigator = navigator.clone();
        // let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            let username = username.clone();
            let password = password.clone();
            let navigator = navigator.clone();
            let login_pressed = login_pressed.clone();
            if *login_pressed {
                return;
            }
            spawn_local(async move {
                login_pressed.set(true);
                let login_info: AuthStruct = AuthStruct::new(username, password);
                let login_response = auth::login(&login_info).await;
                match login_response {
                    LoginResult::Success => {
                        navigator.push(&Route::Home);
                    }
                    LoginResult::IncorrectCredentials => {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Credenciais erradas.")
                            .unwrap();
                    }
                    LoginResult::NetworkError => {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Backend off do line")
                            .unwrap();
                    }
                }
                login_pressed.set(false);
            });
        })
    };

    let onclick_register: Callback<MouseEvent> = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Register);
        })
    };

    let on_username_input_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value.clone());
            web_sys::console::log_1(&value.into());
        })
    };

    let on_password_input_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            password.set(value.clone());

            web_sys::console::log_1(&value.into());
        })
    };

    html! {
        <div class="login-page-wrapper">
                <div class="login-father-container">
                    // Windows 98 Header Bar
                    <div class="login-header">
                        <span class="login-header-title">{"Login"}</span>
                        <div class="login-header-controls">
                            <button class="login-control-button minimize" type="button"></button>
                            <button class="login-control-button maximize" type="button"></button>
                            <button class="login-control-button close" type="button"></button>
                        </div>
                    </div>
                    
                    // Main Content Container
                    <div class="login-content-container">
                        <div class="right-login-container">
                            <h1>{ "Bem vindo de volta!" } </h1>
                            <form class="login-form">
                                <div class="login-form-container">
                                    <label class="login-form-label"> {"Insira seu nome de usuário"} </label>
                                    <input value={(*username).clone()}
                                        oninput={on_username_input_change} class="login-input" type="text" />
                                    <label class="login-form-label"> {"Insira sua senha"} </label>
                                    <input value={(*password).clone()}
                                        oninput={on_password_input_change} class="login-input-password" type="password" />
                                    <button {onclick} class="login-button" type="button"> {"Entrar"} </button>
                                    <a class="login-register-link" onclick={onclick_register}> {"Ainda não possui uma conta? Clique aqui."} </a>
                                </div>
                            </form>
                        </div>
                        <div class="left-login-container">
                            <img src="login.avif" alt="login image" class="login-image" />
                        </div>
                    </div>
                </div>
        </div>
    }
}
