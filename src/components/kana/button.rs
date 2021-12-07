use yew::prelude::*;

use crate::utils::some_if;
use super::{
    enums::{ChartVersion, SyllabaryVersion},
    screen::{KanaSelector, KanaMessage},
};


pub struct KanaButton {
    active: bool,
    void: bool,
    pub symbol: String,
    callback: Callback<MouseEvent>,
}

impl KanaButton {
    pub fn new(
        symbol: &str,
        button_index: usize,
        row_index: usize,
        chart: ChartVersion,
        syllabary: SyllabaryVersion,
        link: &ComponentLink<KanaSelector>
    ) -> Self {
        Self {
            active: false,
            void: symbol.is_ascii(),
            symbol: String::from(symbol),
            callback: link.callback(move |_|
                KanaMessage::ButtonToggle(
                    button_index,
                    row_index,
                    chart,
                    syllabary,
                )
            ),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn render(&self) -> Html {
        // Setting the content for a void button is a hack to ensure that the void
        //   buttons are the same size as the non-void buttons.
        let content: &str = if self.void { "ん" } else { &self.symbol };
        return html! {
            <button
              active=some_if!(self.active, "true")
              onclick=self.callback.clone()
              void=some_if!(self.void, "true")
            >{content}</button>
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active && !self.void
    }

    pub fn toggle(&mut self) {
        self.set_active(self.active ^ true)
    }
}
