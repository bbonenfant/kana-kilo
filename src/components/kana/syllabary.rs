use std::collections::BTreeMap;

use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::utils::some_if;
use super::{
    chart::KanaChart,
    enums::{ChartVersion, SyllabaryVersion},
    screen::{KanaSelector, KanaMessage},
};


pub struct KanaSyllabary {
    title: String,
    callback: Callback<MouseEvent>,
    pub charts: BTreeMap<ChartVersion, KanaChart>,
}

impl KanaSyllabary {
    pub fn new(version: SyllabaryVersion, link: &ComponentLink<KanaSelector>) -> Self {
        let callback = link.callback(
            move |_| KanaMessage::SyllabaryToggle(version)
        );

        let mut charts = BTreeMap::new();
        for chart in ChartVersion::iter() {
            charts.insert(
                chart, KanaChart::new(chart,version, link)
            );
        }
        Self {
            title: version.to_string(),
            callback,
            charts,
        }
    }

    pub fn get_selected_kana(&self) -> Vec<String> {
        self.charts.values().flat_map(KanaChart::get_selected_kana).collect()
    }

    pub fn is_active(&self) -> bool {
        self.charts.values().any(KanaChart::is_active)
    }

    pub fn toggle(&mut self) {
        let active = !self.is_active();
        for chart in self.charts.values_mut() {
            chart.set_active(active);
        }
    }

    pub fn render(&self) -> Html {
        return html! {
            <div class="kana-syllabary">
                <button
                    active=some_if!(self.is_active(), "true")
                    class="syllabary-title"
                    onclick=self.callback.clone()
                >{ &self.title }</button>
                <div class="kana-chart-collection">
                    { for self.charts.values().map(KanaChart::render) }
                </div>
            </div>
        }
    }
}
