use yew::prelude::*;

use crate::utils::some_if;
use super::{
    row::KanaRow,
    enums::{ChartVersion, SyllabaryVersion},
    screen::{KanaSelector, KanaMessage},
};


pub struct KanaChart {
    title: String,
    callback: Callback<MouseEvent>,
    pub rows: Vec<KanaRow>,
}

impl KanaChart {
    pub fn new(
        version: ChartVersion,
        syllabary: SyllabaryVersion,
        link: &ComponentLink<KanaSelector>
    ) -> Self {
        let symbols =
            match syllabary {
                SyllabaryVersion::Hirigana => {
                    match version {
                        ChartVersion::Basic => {
                            vec![
                                "あいうえお",
                                "かきくけこ",
                                "さしすせそ",
                                "たちつてと",
                                "なにぬねの",
                                "はひふへほ",
                                "まみむめも",
                                "や ゆ よ",
                                "らりるれろ",
                                "わ   を",
                                "ん    ",
                            ]
                        },
                        ChartVersion::Dakuon => {
                            vec![
                                "がぎぐげご",
                                "ざじずぜぞ",
                                "だぢづでど",
                                "ばびぶべぼ",
                                "ぱぴぷぺぽ",
                            ]
                        },
                        ChartVersion::Combo => {
                            vec![
                                "きゃきゅきょ",
                                "しゃしゅしょ",
                                "ちゃちゅちょ",
                                "にゃにゅにょ",
                                "ひゃひゅひょ",
                                "みゃみゅみょ",
                                "りゃりゅりょ",
                                "ぎゃぎゅぎょ",
                                "じゃじゅじょ",
                                "びゃびゅびょ",
                                "ぴゃぴゅぴょ",
                            ]
                        },
                    }
                },
                SyllabaryVersion::Katakana => {
                    match version {
                        ChartVersion::Basic => {
                            vec![
                                "アイウエオ",
                                "カキクケコ",
                                "サシスセソ",
                                "タチツテト",
                                "ナニヌネノ",
                                "ハヒフヘホ",
                                "マミムメモ",
                                "ヤ ユ ヨ",
                                "ラリルレロ",
                                "ワ   ヲ",
                                "ン    ",
                            ]
                        },
                        ChartVersion::Dakuon => {
                            vec![
                                "ガギグゲゴ",
                                "ザジズゼゾ",
                                "ダヂヅデド",
                                "バビブベボ",
                                "パピプペポ",
                                "ヴ    ",
                            ]
                        },
                        ChartVersion::Combo => {
                            vec![
                                "キャキュキョ",
                                "ニャニュニョ",
                                "ヒャヒュヒョ",
                                "ミャミュミョ",
                                "リャリュリョ",
                                "ギャギュギョ",
                                "ビャビュビョ",
                                "ピャピュピョ",
                                "ウィウェウォ",
                                "シャシュシェショ",
                                "チャチュチェチョ",
                                "ファフィフェフォ",
                                "ジャジュジェジョ",
                                "ヴァヴィヴェヴォ",

                            ]
                        },
                    }
                },
            };
        Self {
            title: version.to_string(),
            callback: link.callback(move |_|
                KanaMessage::ChartToggle(version, syllabary)
            ),
            rows: symbols
                .iter()
                .enumerate()
                .map(|(index, &symbols)| {
                    KanaRow::new(symbols, index, version, syllabary, link)
                })
                .collect(),
        }
    }

    pub fn get_selected_kana(&self) -> Vec<String> {
        self.rows.iter().flat_map(KanaRow::get_selected_kana).collect()
    }

    pub fn is_active(&self) -> bool {
        self.rows.iter().any(KanaRow::is_active)
    }

    pub fn set_active(&mut self, active: bool) {
        self.rows.iter_mut().for_each(|row: &mut KanaRow| row.set_active(active))
    }

    pub fn toggle(&mut self) {
        let active = !self.is_active();
        for row in self.rows.iter_mut() {
            row.set_active(active);
        }
    }

    pub fn render(&self) -> Html {
        return html! {
            <div class="kana-chart">
                <button
                    active=some_if!(self.is_active(), "true")
                    class="chart-title"
                    onclick=self.callback.clone()
                >{ &self.title }</button>
                { for self.rows.iter().map(KanaRow::render) }
            </div>
        }
    }
}