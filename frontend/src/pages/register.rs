use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    services::auth::{self, AuthStruct, RegisterResult},
    utils::{routes::Route, validation},
};

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();

    let username = use_state(String::new);
    let password = use_state(String::new);
    let button_pressed = use_state(|| false);
    let username_errors = use_state(|| Vec::<String>::new());
    let password_errors = use_state(|| Vec::<String>::new());

    let on_username_input_change = {
        let username = username.clone();
        let username_errors = username_errors.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value.clone());
            
            // Only validate if not empty
            if !value.is_empty() {
                let errors = validation::validate_username(&value);
                username_errors.set(errors);
            } else {
                username_errors.set(Vec::new());
            }
        })
    };

    let on_password_input_change = {
        let password = password.clone();
        let password_errors = password_errors.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            password.set(value.clone());
            
            // Only validate if not empty
            if !value.is_empty() {
                let errors = validation::validate_password(&value);
                password_errors.set(errors);
            } else {
                password_errors.set(Vec::new());
            }
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
        let username_errors = username_errors.clone();
        let password_errors = password_errors.clone();

        Callback::from(move |_: MouseEvent| {
            // Validate credentials
            let (user_errs, pass_errs) = validation::validate_credentials(&username, &password);
            username_errors.set(user_errs.clone());
            password_errors.set(pass_errs.clone());

            // If there are validation errors, don't proceed
            if !user_errs.is_empty() || !pass_errs.is_empty() {
                return;
            }

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

    let on_password_keydown = {
        let onclick = on_click_register.clone();
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
                                    oninput={on_username_input_change} class="register-input" type="text" required=true minlength="6" />
                                {
                                    if !(*username_errors).is_empty() {
                                        html! {
                                            <div class="error-messages">
                                                { for (*username_errors).iter().map(|error| html! {
                                                    <div>{ format!("- {}", error) }</div>
                                                })}
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <label class="register-form-label"> {"Insira sua senha"} </label>
                                <input value={(*password).clone()}
                                    oninput={on_password_input_change} onkeydown={on_password_keydown} class="register-input-password" type="password" required=true minlength="8" />
                                {
                                    if !(*password_errors).is_empty() {
                                        html! {
                                            <div class="error-messages">
                                                { for (*password_errors).iter().map(|error| html! {
                                                    <div>{ format!("- {}", error) }</div>
                                                })}
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
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
