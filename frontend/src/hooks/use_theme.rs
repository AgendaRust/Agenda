// use yew::prelude::*;

// #[hook]
// pub fn use_theme() -> (bool, Callback<()>) {
//     let dark_mode = use_state(|| StorageService::get_theme());

//     let toggle_theme = {
//         let dark_mode = dark_mode.clone();
//         Callback::from(move |_| {
//             let new_mode = !*dark_mode;
//             StorageService::set_theme(new_mode);
//             dark_mode.set(new_mode);
//         })
//     };

//     (*dark_mode, toggle_theme)
// }
