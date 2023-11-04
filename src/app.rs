use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use embedded_graphics::pixelcolor::Rgb565;
use yew::prelude::*;
use web_sys::{Element, console};
use crate::view::{Breadboard, Row, HitTestResult};
use std::collections::HashSet;

#[derive(Default, Properties, PartialEq)]
pub struct Props;

pub struct App {
    node_ref: NodeRef,
    view: Breadboard<'static>,
}

impl Component for App {
    type Message = (i32, i32);
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            view: Breadboard::new((20, 20, 20, 20), 10, (10, 10))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(|e: MouseEvent| (e.offset_x(), e.offset_y()));
        html! {
            <div onmousedown={onmousedown} ref={self.node_ref.clone()}></div>
        }
    }

   fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
       match self.view.hit_test(msg.0, msg.1) {
           HitTestResult::HitColumnLabel((region, column_label)) => {
               self.view.select_column_label(region, column_label);
               true
           },
           _ => false
       }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let output_settings = OutputSettingsBuilder::new()
                .scale(1)
                .pixel_spacing(0)
                .build();

            let display = WebSimulatorDisplay::new(
                (480, 320),
                &output_settings,
                self.node_ref.cast::<Element>().as_ref()
            );

            self.view.init(display);
        }
        self.view.draw();
    }
}

// if self.start.is_none() {
//     console::log_1(&"collecting".into());
//     self.start = Some(msg);
// } else {
//     console::log_1(&"togglging".into());
//     self.connections.toggle((
//       (self.start.unwrap().0.div_ceil(21)).try_into().unwrap(),
//       (msg.0.div_ceil(21)).try_into().unwrap()
//     ));

//     crate::breadboard::draw(
//         &mut self.connections,
//         &mut self.connection_area,
//         &[
//             Rgb565::new(255, 0, 0),
//             Rgb565::new(0, 255, 0),
//             Rgb565::new(0, 0, 255),
//         ]
//     );

//     self.connection_area.draw(self.display.as_mut().unwrap());
//     self.display.as_mut().unwrap().flush().expect("Couldn't update");
//     self.start = None;
// }

