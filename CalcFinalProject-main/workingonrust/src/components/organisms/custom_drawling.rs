use gloo::console::log;
use yew::prelude::*;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::molecules::custom_form::recive_planet;
use crate::components::molecules::custom_form::recive_velocity;
use crate::components::molecules::custom_form::recive_angle;
use yew_canvas::{Canvas, WithRander};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, PartialEq)]
struct Rander {
    //use this struct send props to canvas
    sakara: usize,
}

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let interface: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        interface.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        interface.set_fill_style(&JsValue::from_str("#fe5c5a"));
        interface.set_font("100px sans-serif");
        interface.set_text_baseline("top");

        let sakara = (vec!['🐟'; self.sakara]).into_iter().collect::<String>();
        let text = &format!("Planet");

        let text_metrics = interface.measure_text(text).unwrap();
        let (actual_bounding_box_descent, font_bounding_box_descent, width) = (
            text_metrics.actual_bounding_box_descent(),
            text_metrics.font_bounding_box_descent(),
            text_metrics.width(),
        );

        let text_pos = (100.0, 100.0);

        interface.fill_text(text, text_pos.0, text_pos.1).unwrap();
        interface.set_stroke_style(&JsValue::from_str("red"));
        interface.stroke_rect(text_pos.0, text_pos.1, width, actual_bounding_box_descent);

        interface.set_stroke_style(&JsValue::from_str("green"));
        interface.stroke_rect(text_pos.0, text_pos.1, width, font_bounding_box_descent);

        interface.stroke_circle();
    }
}

#[function_component(CustomDrawling)]
pub fn custom_drawling() -> Html {

    let button_clicked = Callback::from(move |_| {
        log!(recive_planet("pull".to_string()));
        log!(recive_velocity("pull".to_string()));
        log!(recive_angle("pull".to_string()));

    });

    let sakara_state = use_state(|| 0);

    let onclick = {
        let sakara_state = sakara_state.clone();
        Callback::from(move |_| sakara_state.set(*sakara_state + 1))
    };

    html!(
            <>
                <CustomButton label="Input Data" onclick={button_clicked}/>
                <button {onclick}>{"+🐟"}</button>
                <Canvas<CanvasRenderingContext2d, Rander>
                    //Just use style, canvas can suit automaticly.
                    style="
                        width: 100vw;
                        height: calc(100vh - 32px);
                    "
                    //send props when create a Rander
                    rander={Box::new(Rander{sakara: *sakara_state})}
                >
                    {"The browser is not supported."}
                </Canvas<CanvasRenderingContext2d, Rander>>
            </>
        )
}//