//! The main component of the application.
use std::rc::Rc;

use yew::prelude::*;
use yew::utils::window;

use crate::translations::create_translations;
use super::{
    game::GameScreen,
    kana::KanaSelector,
    hide_state::HideState,
    icons::github,
};


pub enum AppMessage {
    AnimationStart(Animation, Screen),
    AnimationEnd(Animation, Screen),
    ToggleGameMode,
    SetSymbols(Vec<String>)
}

pub struct App {
    link: ComponentLink<Self>,
    game_hide_state: HideState,
    kana_hide_state: HideState,
    symbols: Vec<String>

}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            game_hide_state: HideState::new_hidden(),
            kana_hide_state: HideState::new_visible(),
            symbols: Vec::new()
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            AppMessage::AnimationStart(_animation, _child) => {
                false
            },
            AppMessage::AnimationEnd(animation, child) => {
                // At the end for a FadeOut animation we toggle the display attribute
                //   of the game and kana screens.
                match (animation, child) {
                    (Animation::FadeOut, Screen::KanaSelector) => {
                        self.game_hide_state.toggle_display_none();
                        self.kana_hide_state.toggle_display_none();
                        true
                    },
                    (Animation::FadeOut, Screen::Game) => {
                        self.game_hide_state.toggle_display_none();
                        self.kana_hide_state.toggle_display_none();
                        true
                    },
                    _ => false
                }
            },
            AppMessage::ToggleGameMode => {
                window().scroll_to_with_x_and_y(0.0, 0.0);
                self.game_hide_state.toggle();
                self.kana_hide_state.toggle();
                true
            },
            AppMessage::SetSymbols(symbols) => {
                self.symbols = symbols;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| AppMessage::ToggleGameMode);
        let mut button_content = "Start";
        let mut translations = Vec::new();
        if self.is_play_mode() {
            button_content = "Back";
            translations = create_translations(&self.symbols);
        }

        return html! {
            <>
            <div class="github-link">
                <a href="https://github.com/bbonenfant/kana-kilo">{ github() }</a>
            </div>
            <div class="main-container" centered=(!self.game_hide_state.is_hidden()).to_string()>
                <h2 class="title">{ "Kana Kilo" }</h2>
                <div class="play-configurations">
                    <button href="#top" onclick=onclick>{ button_content }</button>
                </div>
                <GameScreen
                    hide_state=self.game_hide_state.clone()
                    translations=Rc::new(translations)
                    screen_type=Screen::Game/>
                <KanaSelector
                    hide_state=self.kana_hide_state.clone()
                    screen_type=Screen::KanaSelector/>
            </div>
            </>
        }
    }
}

impl App {
    fn is_play_mode(&self) -> bool {
        !self.game_hide_state.is_hidden()
    }
}


/// Enumeration of all the CSS animations supported.
pub enum Animation {
    Unknown,
    FadeIn,
    FadeOut,
}

impl From<String> for Animation {

    /// Conversion from the output of AnimationElement.animation_name() method.
    fn from(str: String) -> Self {
        if str == "fade-in" {
            Self::FadeIn
        } else if str == "fade-out" {
            Self::FadeOut
        } else {
            Self::Unknown
        }
    }
}


/// Enumeration of all "Screens" of the app.
#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Game,
    KanaSelector,
}

impl Screen {

    /// Helper function for creating the Callback for onanimationstart.
    pub fn onanimationstart(&self, link: &ComponentLink<App>) -> Callback<AnimationEvent> {
        let screen = *self;
        link.callback(move |event: AnimationEvent|
            AppMessage::AnimationStart(
                event.animation_name().into(),
                screen
            )
        )
    }

    /// Helper function for creating the Callback for onanimationend.
    pub fn onanimationend(&self, link: &ComponentLink<App>) -> Callback<AnimationEvent> {
        let screen = *self;
        link.callback(move |event: AnimationEvent|
            AppMessage::AnimationEnd(
                event.animation_name().into(),
                screen
            )
        )
    }
}