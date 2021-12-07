use yew::prelude::*;

use crate::{SyllabogramSegmentation, utils::some_if};
use super::{
    button::KanaButton,
    enums::{ChartVersion, SyllabaryVersion},
    screen::{KanaSelector, KanaMessage},
};


pub struct KanaRow {
    callback: Callback<MouseEvent>,
    pub buttons: Vec<KanaButton>,
}

impl KanaRow {
    pub fn new(
        symbols: &str,
        row_index: usize,
        chart: ChartVersion,
        syllabary: SyllabaryVersion,
        link: &ComponentLink<KanaSelector>
    ) -> Self {
        Self {
            callback: link.callback(move |_|
                KanaMessage::RowToggle(
                    row_index,
                    chart,
                    syllabary,
                )
            ),
            buttons: symbols.syllabograms()
                .enumerate()
                .map(|(index, syllabogram)| {
                    KanaButton::new(
                        syllabogram,
                        index,
                        row_index,
                        chart,
                        syllabary,
                        link,
                    )
                })
                .collect(),
        }
    }

    pub fn get_selected_kana(&self) -> Vec<String> {
        self.buttons.iter()
            .filter(|&b| b.is_active())
            .map(|b| b.symbol.clone())
            .collect()
    }

    pub fn is_active(&self) -> bool {
        self.buttons.iter().any(KanaButton::is_active)
    }

    pub fn set_active(&mut self, active: bool) {
        self.buttons.iter_mut().for_each(|button: &mut KanaButton| button.set_active(active))
    }

    pub fn toggle(&mut self) {
        let active = !self.is_active();
        for button in self.buttons.iter_mut() {
            button.set_active(active);
        }
    }

    pub fn render(&self) -> Html {
        let is_active = self.is_active();
        let checkbox_icon = if is_active {
            crate::components::icons::square_check_solid()
        } else {
            crate::components::icons::square_solid()
        };

        return html! {
            <div class="kana-row">
                <button
                    active=some_if!(is_active, "true")
                    class="checkbox"
                    onclick=self.callback.clone()
                >{ checkbox_icon }</button>
                { for self.buttons.iter().map(KanaButton::render) }
            </div>
        }
    }
}
