use gloo::console::log;
use yew::prelude::*;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::custom_form::recive_planet;


#[function_component(CustomDrawling)]
pub fn custom_drawling() -> Html {

    let button_clicked = Callback::from(move |_| {
        log!(recive_planet("pull".to_string()));
    });

    html!{
        <CustomButton label="Input Data" onclick={button_clicked}/>
    }
}//