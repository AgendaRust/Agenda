use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::utils::{routes::Route, validation};

use crate::services::auth::{self, AuthStruct, LoginResult};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();

    let username = use_state(String::new);
    let password = use_state(String::new);
    let login_pressed = use_state(|| false);
    let show_invalid_credentials = use_state(|| false);

    let onclick = {
        let username = (*username).clone();
        let password = (*password).clone();
        let navigator = navigator.clone();
        let show_invalid_credentials = show_invalid_credentials.clone();
        Callback::from(move |_: MouseEvent| {
            // Validate credentials before calling backend
            let (user_errs, pass_errs) = validation::validate_credentials(&username, &password);

            // If there are validation errors, show "Credenciais inválidas" and don't proceed
            if !user_errs.is_empty() || !pass_errs.is_empty() {
                show_invalid_credentials.set(true);
                return;
            }

            // Clear the error message if validation passes
            show_invalid_credentials.set(false);

            let username = username.clone();
            let password = password.clone();
            let navigator = navigator.clone();
            let login_pressed = login_pressed.clone();
            let show_invalid_credentials = show_invalid_credentials.clone();
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
                        show_invalid_credentials.set(true);
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
        let show_invalid_credentials = show_invalid_credentials.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value.clone());
            
            // Clear error message when user starts typing
            show_invalid_credentials.set(false);
        })
    };

    let on_password_input_change = {
        let password = password.clone();
        let show_invalid_credentials = show_invalid_credentials.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            password.set(value.clone());
            
            // Clear error message when user starts typing
            show_invalid_credentials.set(false);
        })
    };

    let on_password_keydown = {
        let onclick = onclick.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                // Trigger the login by creating a dummy MouseEvent
                let dummy_event = web_sys::MouseEvent::new("click").unwrap();
                onclick.emit(dummy_event);
            }
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
                                        oninput={on_password_input_change}
                                        onkeydown={on_password_keydown}
                                        class="login-input-password" type="password" />
                                    {
                                        if *show_invalid_credentials {
                                            html! {
                                                <div class="error-messages">
                                                    <div>{ "- Credenciais inválidas" }</div>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <button {onclick} class="login-button" type="button"> {"Entrar"} </button>
                                    <a class="login-register-link" onclick={onclick_register}> {"Ainda não possui uma conta? Clique aqui."} </a>
                                </div>
                            </form>
                        </div>
                        <div class="left-login-container">
                            <img src="coca_login.png" alt="login image" class="login-image" />
                        </div>
                    </div>
                </div>
        </div>
    }
}
