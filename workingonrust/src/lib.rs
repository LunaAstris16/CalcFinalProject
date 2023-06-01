use yew::prelude::*;
use gloo::console::log;

mod components;

use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;
use components::organisms::custom_drawling::CustomDrawling;

#[function_component(App)]
pub fn app() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));

    html! {
        <>
            <MainTitle title="Gravity Assist Modeling" color={Color::Normal} on_load={main_title_load}/>
            <CustomForm />
            <CustomDrawling />
        </>
    }
}