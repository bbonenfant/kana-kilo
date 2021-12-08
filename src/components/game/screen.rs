use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
use yewtil::NeqAssign;

use super::super::{
    app::{App, Screen},
    hide_state::HideState
};
use super::{
    Clock,
    KanaLine,
    KanaTranslationList,
    RomanjiLine,
    Score,
};


pub enum GameMessage {
    TextInput(String),
    KanaShift(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProperties {
    pub hide_state: HideState,
    #[prop_or_default]
    pub translations: KanaTranslationList,
    pub screen_type: Screen,
}

pub struct GameScreen {
    props: GameProperties,
    active: bool,
    input_ref: NodeRef,
    score: Score,
    text: String,
    onanimationstart: Callback<AnimationEvent>,
    onanimationend: Callback<AnimationEvent>,
}

impl Component for GameScreen {
    type Message = GameMessage;
    type Properties = GameProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let app = link.get_parent().unwrap().clone().downcast::<App>();
        let onanimationstart = props.screen_type.onanimationstart(&app);
        let onanimationend = props.screen_type.onanimationend(&app);
        Self {
            props,
            active: false,
            input_ref: NodeRef::default(),
            score: Score::default(),
            text: String::default(),
            onanimationstart,
            onanimationend,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::TextInput(text) => {
                self.active = true;
                self.text = text;
                true
            },
            Self::Message::KanaShift(is_correct) => {
                self.score.tally(is_correct);
                self.text = String::default();
                if let Some(input) = self.html_input_element() {
                    input.set_value(&self.text);
                    input.focus().ok();
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = self.props.neq_assign(props);
        if !self.props.hide_state.is_hidden() {
            if let Some(input) = self.html_input_element() {
                input.focus().ok();
            }
        } else {
            self.active = false;
            self.score.reset();
        }
        should_render
    }

    fn view(&self) -> Html {
        return html! {
            <div
                class="game-screen-container"
                state=self.props.hide_state.as_string()
                style=self.props.hide_state.display_none()
                onanimationend=self.onanimationend.clone()
                onanimationstart=self.onanimationstart.clone()
            >
                <KanaLine
                    hidden=self.props.hide_state.is_hidden()
                    length=100
                    text=self.text.clone()
                    translations=self.props.translations.clone()
                />
                <RomanjiLine
                    hidden=self.props.hide_state.is_hidden()
                    input_ref=self.input_ref.clone()
                />
                <div class="metrics-container">
                    <Clock active=self.active/>
                    { self.score.render() }
                </div>
            </div>
        }
    }
}

impl GameScreen {
    //noinspection RsWrongGenericArgumentsNumber
    fn html_input_element(&self) -> Option<HtmlInputElement> {
        self.input_ref.cast::<HtmlInputElement>()
    }
}
