use std::rc::Rc;

use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;

use super::{GameMessage, GameScreen, Translation};


#[derive(Clone, Properties, Debug)]
pub struct KanaLineProperties {
    pub hidden: bool,
    pub length: usize,
    pub text: String,
    pub translations: Rc<Vec<Translation>>,
}

impl NeqAssign<KanaLineProperties> for KanaLineProperties {
    fn neq_assign(&mut self, new: KanaLineProperties) -> ShouldRender {
        let should_render =
            self.hidden != new.hidden
            || self.length != new.length
            || self.translations != new.translations;
        *self = new;
        should_render
    }
}

pub struct KanaLine {
    props: KanaLineProperties,
    content_generator: ContentGenerator,
    contents: Vec<Content>,
    index: usize,
    onshift: Callback<bool>,
    shifted: bool,
}

impl Component for KanaLine {
    type Message = ();
    type Properties = KanaLineProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let contents = Vec::with_capacity(props.length);
        let onshift = {
            let parent = link
                .get_parent().expect("KanaLine component does not have parent")
                .clone().downcast::<GameScreen>();
            parent.callback(GameMessage::KanaShift)
        };
        Self {
            props,
            content_generator: ContentGenerator::default(),
            contents,
            index: 0,
            onshift,
            shifted: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = self.props.neq_assign(props);

        // If the properties have changed, then we should reinitialize the KanaLine contents.
        if should_render {
            self.content_generator.set_translations(self.props.translations.clone());
            self.contents.clear();
            if !self.props.hidden {
                for _ in 0..self.props.length {
                    match self.content_generator.random() {
                        Some(content) => self.contents.push(content),
                        None => {
                            ConsoleService::warn("No Content Generated!");
                            break
                        }
                    };
                }
            }
            self.index = 0;
        }

        should_render | self.check()
    }

    fn view(&self) -> Html {
        let mut styles = vec![
            format!("margin-left: -{}em;", self.get_margin()),
            format!("width: {}em;", self.get_width()),
        ];
        if self.shifted {
            styles.push(String::from("transition: margin-left 200ms linear"))
        }
        return html!{
            <div class="kana-line">
                <div class="kana-line-container" >
                    <div class="kana-line-scroll-container" style=styles.join(" ")>
                        { for self.contents.iter().map(|c| c.render() )}
                    </div>
                </div>
            </div>
        }
    }
}

impl KanaLine {

    fn check(&mut self) -> ShouldRender {
        if self.contents.is_empty() || self.props.text.is_empty() {
            return false
        }

        let current = self.contents.get_mut(self.index)
            .expect("could not locate current content of KanaLine");
        current.check(self.props.text.to_lowercase());
        match current.state {
            ContentState::Unanswered => {
                self.shifted = false;
                self.resample()
            },
            ContentState::Answered(is_correct) => {
                self.onshift.emit(is_correct);
                self.index += 1;
                self.shifted = true;
                true
            },
        }
    }

    fn resample(&mut self) -> ShouldRender {
        if !self.contents.is_empty() && self.index >= (self.contents.len() / 2) {
            for _ in 0..(self.contents.len() / 3) {
                self.contents.remove(0);
                let new = self.content_generator.random()
                    .expect("Could not generate new content during resample. ");
                self.contents.push(new);
                self.index -= 1;
            }
            return true
        }
        false
    }

    fn get_margin(&self) -> f32 {
        if self.contents.is_empty() {
            return 0f32
        }

        let margin = self.contents[0..self.index]
            .iter()
            .fold(0, |s, c| s + c.size)
            as f32;
        let current = self.contents.get(self.index)
            .expect("Could not get current content");
        margin + (current.size as f32 / 2.0)
    }

    fn get_width(&self) -> usize {
        self.contents.iter().fold(0, |s, c| s + c.size)
    }
}

#[derive(Default)]
struct ContentGenerator {
    translations: Rc<Vec<Translation>>,
}

impl ContentGenerator {
    fn random(&self) -> Option<Content> {
        if self.translations.is_empty() {
            return None
        }
        let index = rand::random::<usize>() % self.translations.len();
        let (kana, romanji) = self.translations[index].clone();
        Some(Content::new(kana, romanji))
    }

    fn set_translations(&mut self, translations: Rc<Vec<Translation>>) {
        self.translations = translations;
    }
}


enum ContentState {
    Unanswered,
    Answered(bool),
}

struct Content {
    kana: String,
    romanji: Vec<String>,
    size: usize,
    state: ContentState,
}

impl Content {
    fn new(kana: String, romanji: Vec<String>) -> Self {
        let size = kana.chars().count();
        Self {
            kana,
            romanji,
            size,
            state: ContentState::Unanswered,
        }
    }

    fn check(&mut self, text: String) {
        if text.is_empty() {
            return
        }
        if "あいうえおんアイウエオ".contains(&self.kana) || text.len() == self.romanji[0].len() {
            let is_correct = self.romanji.contains(&text);
            self.state = ContentState::Answered(is_correct);
        }
    }

    fn render(&self) -> Html {
        let style = format!("width: {}em", self.size);
        let mut class = "white";
        let mut small_above = VNode::default();

        if let ContentState::Answered(is_correct) = self.state {
            if is_correct {
                class = "green";
            } else {
                class = "red";
                small_above = html!{
                    <span class="small-above">{ self.romanji[0].clone() }</span>
                };
            }
        }

        return html! {
            <span class=class style=style>
                { small_above }
                { self.kana.clone() }
            </span>
        }
    }
}
