use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: AttrValue,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header class="app-header">
            <h1>{ &props.title }</h1>
        </header>
    }
}
