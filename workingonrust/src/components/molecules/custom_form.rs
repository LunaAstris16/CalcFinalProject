use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;

pub fn retrive_data_username() -> UseStateHandle<String>{
    let return_value = username_state.clone();
    return return_value;
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {

    let username_state = use_state(||"no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username|{
        cloned_username_state.set(username);
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