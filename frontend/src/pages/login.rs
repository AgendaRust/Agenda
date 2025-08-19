use yew::prelude::*;

#[function_component(Login)]
pub fn login_page() -> Html {
    let onclick = {
        Callback::from(move |_: MouseEvent| {
            web_sys::console::log_1(&"Bonsonara".to_string().into());
        })
    };

    html! {
        <>
        <div class= "login-father-container">
            <div class= "right-login-container">
                <h1>{ "Bem vindo de volta!" } </h1>
                <form class= "login-form">
                    <div class= "login-form-container">
                        <label class= "login-form-label"> {"Insira seu nome de usuário"} </label>
                        <input class= "login-input" type= "text" />
                        <label class= "login-form-label"> {"Insira sua senha"} </label>
                        <input class= "login-input-password" type= "password" />
                        <button {onclick} class= "login-button" type="button"> {"Entrar"} </button>
                    </div>
                    </form>
            </div>
            <div class= "left-login-container">
                // <h1> {"Sua pag de login"} </h1>
                <img src = "login.avif" alt = "login image" class = "login-image" />
            </div>
        </div>

        </>
    }
}
