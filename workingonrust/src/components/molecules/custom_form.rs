
use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use std::boxed::Box;

static mut PLANET: &str = "";

pub fn recive_planet(variable: String) -> String{
    let var = variable;
    log!(var.clone());
    let baseplanet = "Jupiter".to_string();
    if var.clone() == "pull"{
        if unsafe { PLANET.clone() } != ""{
            return unsafe { PLANET.to_string() };
        }
        else {
            return baseplanet;
        }
    }
    else {
        log!("pushed");
        let temp: &str = Box::leak(var.into_boxed_str());
        unsafe { PLANET = temp};
        return unsafe { PLANET.clone().to_string() };
    }

}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {

    let username_state = use_state(||"no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username: String|{
        cloned_username_state.set(username.clone());
        recive_planet(username.clone());
    });

    let velocity_changed = Callback::from(|username|{
        let velocity = username;
        log!(velocity)
    });

    let angle_changed = Callback::from(|username|{
        let angle = username;
        log!(angle)
    });

    html!{
        <form>
            <TextInput name="username" handle_onchange={username_changed}/>
            <TextInput name="velovity" handle_onchange={velocity_changed}/>
            <TextInput name="angle" handle_onchange={angle_changed}/>
        </form>
    }
}