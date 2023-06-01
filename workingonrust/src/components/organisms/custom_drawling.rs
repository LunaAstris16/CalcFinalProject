use yew::prelude::*;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::custom_form::retrive_data_username;
use gloo::console::log;


#[function_component(CustomDrawling)]
pub fn custom_drawling() -> Html {

    let button_clicked = Callback::from(move |_| {
        let username = retrive_data_username();
        log!(&*username);
    });
    


    html!{
        <CustomButton label="Input Data" onclick={button_clicked}/>
    }
}