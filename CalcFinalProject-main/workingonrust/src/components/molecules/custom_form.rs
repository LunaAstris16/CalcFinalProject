
use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use std::boxed::Box;
use crate::components::atoms::text::Text;

static mut PLANET: &str = "";
static mut VELOCITY: &str = "";
static mut ANGLE: &str = "";

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

pub fn recive_velocity(variable: String) -> String{
    let var = variable;
    log!(var.clone());
    let basevelocity = "10".to_string();
    if var.clone() == "pull"{
        if unsafe { VELOCITY.clone() } != ""{
            return unsafe { VELOCITY.to_string() };
        }
        else {
            return basevelocity;
        }
    }
    else {
        log!("pushed");
        let temp: &str = Box::leak(var.into_boxed_str());
        unsafe { VELOCITY = temp};
        return unsafe { VELOCITY.clone().to_string() };
    }

}

pub fn recive_angle(variable: String) -> String{
    let var = variable;
    log!(var.clone());
    let baseangle = "100".to_string();
    if var.clone() == "pull"{
        if unsafe { ANGLE.clone() } != ""{
            return unsafe { ANGLE.to_string() };
        }
        else {
            return baseangle;
        }
    }
    else {
        log!("pushed");
        let temp: &str = Box::leak(var.into_boxed_str());
        unsafe { ANGLE = temp};
        return unsafe { ANGLE.clone().to_string() };
    }

}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {

    let username_state = use_state(||"no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username: String|{
        cloned_username_state.set(username.clone());
        let _planet_recived = recive_planet(username.clone());
    });

    let velocity_state = use_state(||"No velocity Set".to_owned());
    let cloned_velocity_state = velocity_state.clone();
    let velocity_changed = Callback::from(move |velocity: String|{
        cloned_velocity_state.set(velocity.clone());
        let _velocity_recived = recive_velocity(velocity.clone());
    });

    let angle_state = use_state(||"No angle Set".to_owned());
    let cloned_angle_state = angle_state.clone();
    let angle_changed = Callback::from(move |angle: String|{
        cloned_angle_state.set(angle.clone());
        let _angle_recived = recive_angle(angle.clone());
    });

    html!{
        <form>
            <Text text={"Planet: "}/><TextInput name="username" handle_onchange={username_changed}/><br/>
            <Text text={"Velocity: "}/><TextInput name="velocity" handle_onchange={velocity_changed}/><br/>
            <Text text={"Angle: "}/><TextInput name="angle" handle_onchange={angle_changed}/>
        </form>
    }
}