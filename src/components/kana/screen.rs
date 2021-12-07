use std::collections::BTreeMap;

use strum::IntoEnumIterator;
use yew::prelude::*;
use yewtil::NeqAssign;

use super::super::{
    app::{App, AppMessage, Screen},
    hide_state::HideState,
};
use super::{
    enums::{ChartVersion, SyllabaryVersion},
    syllabary::KanaSyllabary,
};


#[allow(clippy::enum_variant_names)]
pub enum KanaMessage {
    SyllabaryToggle(SyllabaryVersion),
    ChartToggle(ChartVersion, SyllabaryVersion),
    RowToggle(usize, ChartVersion, SyllabaryVersion),
    ButtonToggle(usize, usize, ChartVersion, SyllabaryVersion),
}

#[derive(Clone, PartialEq, Properties)]
pub struct KanaSelectorProperties {
    pub hide_state: HideState,
    pub screen_type: Screen
}

pub struct KanaSelector {
    props: KanaSelectorProperties,
    onanimationstart: Callback<AnimationEvent>,
    onanimationend: Callback<AnimationEvent>,
    refresh: Callback<Vec<String>>,
    syllabaries: BTreeMap<SyllabaryVersion, KanaSyllabary>,
}

impl KanaSelector {

    /// Iterates through all of the KanaSelector elements and returns all
    /// kana symbols that are currently selected.
    fn get_selected_kana(&self) -> Vec<String> {
        self.syllabaries.values().flat_map(KanaSyllabary::get_selected_kana).collect()
    }
}

impl Component for KanaSelector {
    type Message = KanaMessage;
    type Properties = KanaSelectorProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let app = link.get_parent().unwrap().clone().downcast::<App>();
        let onanimationstart = props.screen_type.onanimationstart(&app);
        let onanimationend = props.screen_type.onanimationend(&app);
        let refresh = app.callback(AppMessage::SetSymbols);

        let mut syllabaries: BTreeMap<SyllabaryVersion, KanaSyllabary> = BTreeMap::new();
        for syllabary in SyllabaryVersion::iter() {
            syllabaries.insert(
                syllabary, KanaSyllabary::new(syllabary, &link)
            );
        }

        { // Set the entire Basic Hirigana chart to active.
            let hiragana = syllabaries.get_mut(&SyllabaryVersion::Hirigana).unwrap();
            hiragana.charts.get_mut(&ChartVersion::Basic).unwrap().toggle();
            refresh.emit(hiragana.get_selected_kana());
        }
        Self { props, onanimationstart, onanimationend, refresh, syllabaries }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::SyllabaryToggle(syllabary) => {
                self.syllabaries.get_mut(&syllabary).unwrap()
                    .toggle();
            },
            Self::Message::ChartToggle(chart, syllabary) => {
                self.syllabaries.get_mut(&syllabary).unwrap()
                    .charts.get_mut(&chart).unwrap()
                    .toggle();
            },
            Self::Message::RowToggle(row, chart, syllabary) => {
                self.syllabaries.get_mut(&syllabary).unwrap()
                    .charts.get_mut(&chart).unwrap()
                    .rows[row]
                    .toggle();
            },
            Self::Message::ButtonToggle(button, row, chart, syllabary) => {
                self.syllabaries.get_mut(&syllabary).unwrap()
                    .charts.get_mut(&chart).unwrap()
                    .rows[row]
                    .buttons[button]
                    .toggle();
            },
        }
        self.refresh.emit(self.get_selected_kana());
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        return html! {
            <div
                class="kana-selection-container"
                state=self.props.hide_state.as_string()
                style=self.props.hide_state.display_none()
                onanimationend=self.onanimationend.clone()
                onanimationstart=self.onanimationstart.clone()
            >
                { for self.syllabaries.values().map(|syllabary| syllabary.render()) }
            </div>
        }
    }
}
