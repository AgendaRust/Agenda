use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    services::auth::{self, AuthStruct, RegisterResult},
    utils::routes::Route,
};

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();

    let username = use_state(String::new);
    let password = use_state(String::new);
    let button_pressed = use_state(|| false);

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

    let on_click_login: Callback<MouseEvent> = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Login);
        })
    };

    let on_click_register: Callback<MouseEvent> = {
        let username: String = (*username).clone();
        let password: String = (*password).clone();
        let navigator = navigator.clone();
        let button_pressed = button_pressed.clone();

        Callback::from(move |_: MouseEvent| {
            web_sys::console::log_1(
                &format!("tentando registrar com {username} {password}").into(),
            );

            let register_info: AuthStruct = AuthStruct::new(username.clone(), password.clone());
            let navigator = navigator.clone();
            let button_pressed = button_pressed.clone();
            if (*button_pressed) == false {
                spawn_local(async move {
                    button_pressed.set(true);
                    match auth::register(&register_info).await {
                        RegisterResult::Success => {
                            // Handle successful registration
                            web_sys::console::log_1(&"Successful register".into());
                            navigator.push(&Route::Home);
                        }
                        RegisterResult::InvalidFields => {
                            web_sys::console::log_1(
                                &format!("Registration failed, invalid input").into(),
                            );
                        }
                        RegisterResult::NetworkError => {
                            web_sys::console::log_1(&format!("Backend off do line").into());
                        }
                    }
                    button_pressed.set(false);
                })
            }
        })
    };

    html! {
        <div class="register-page-wrapper">
            <div class="register-father-container">
                // Windows 98 Header Bar
                <div class="register-header">
                    <span class="register-header-title">{"Register"}</span>
                    <div class="register-header-controls">
                        <button class="register-control-button minimize" type="button"></button>
                        <button class="register-control-button maximize" type="button"></button>
                        <button class="register-control-button close" type="button"></button>
                    </div>
                </div>
                
                // Main Content Container
                <div class="register-content-container">
                    <div class="right-register-container">
                        <h1>{ "Registre-se!" } </h1>
                        <form class="register-form">
                            <div class="register-form-container">
                                <label class="register-form-label"> {"Insira seu nome de usuário"} </label>
                                <input value={(*username).clone()}
                                    oninput={on_username_input_change} class="register-input" type="text" required=true minlength="3" />
                                <label class="register-form-label"> {"Insira sua senha"} </label>
                                <input value={(*password).clone()}
                                    oninput={on_password_input_change} class="register-input-password" type="password" required=true minlength="6" />
                                <button disabled={(*button_pressed).clone()} onclick={on_click_register} class="register-button" type="button"> {"Cadastrar"} </button>
                                <a class="register-login-link" onclick={on_click_login}> {"Já possui uma conta? entre aqui."} </a>
                            </div>
                        </form>
                    </div>
                    <div class="left-register-container">
                        <img src="Tired.jpeg" alt="register image" class="register-image" />
                    </div>
                </div>
            </div>
        </div>
    }
}
