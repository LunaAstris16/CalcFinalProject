use yew::prelude::*;
use gloo::console::log;

mod components;

use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;
use components::organisms::custom_drawling::CustomDrawling;
use components::atoms::text::{Text};

#[function_component(App)]
pub fn app() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));

    let description = "Our project is on the nature of gravity assists";

    html!{
        <>
            <MainTitle title="Gravity Assist Modeling" color={Color::Normal} on_load={main_title_load}/>
            <CustomForm />
            <CustomDrawling />
            <Text text={description}/>
        </>
    }
}