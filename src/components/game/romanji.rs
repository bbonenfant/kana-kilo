use yew::prelude::*;
use yewtil::NeqAssign;

use super::{GameMessage, GameScreen};

#[derive(Clone, PartialEq, Properties)]
pub struct RomanjiLineProperties {
    pub hidden: bool,
    pub input_ref: NodeRef
}

pub struct RomanjiLine {
    props: RomanjiLineProperties,
    oninput: Callback<InputData>,
}

impl Component for RomanjiLine {
    type Message = ();
    type Properties = RomanjiLineProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let oninput = {
            let parent = link
                .get_parent().expect("RomanjiLine component does not have parent")
                .clone().downcast::<GameScreen>();
            parent.callback(|e: InputData| GameMessage::TextInput(e.value))
        };
        Self { props, oninput }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        return html!{
            <div class="romanji-line">
                <input
                    autocapitalize="off"
                    autocomplete="off"
                    autocorrect="off"
                    spellcheck="false"
                    ref=self.props.input_ref.clone()
                    type="visiblePassword"
                    oninput=self.oninput.clone()
                />
            </div>
        }
    }
}