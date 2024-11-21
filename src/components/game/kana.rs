use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yewtil::NeqAssign;

use super::{GameMessage, GameScreen, KanaTranslationList};


pub enum KanaLineMessage {
    ResampleContent
}

#[derive(Clone, Properties)]
pub struct KanaLineProperties {
    pub hidden: bool,
    pub length: usize,
    pub text: String,
    pub translations: KanaTranslationList,
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
    link: ComponentLink<Self>,
    props: KanaLineProperties,
    content_generator: ContentGenerator,
    contents: Vec<Content>,
    index: usize,
    onshift: Callback<bool>,
    show_transition: bool,
}

impl Component for KanaLine {
    type Message = KanaLineMessage;
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
            link,
            props,
            content_generator: ContentGenerator::default(),
            contents,
            index: 0,
            onshift,
            show_transition: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::ResampleContent => self.resample()
        }
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
            format!("transform: translate3d(-{}em,0,0);", self.get_margin()),
            format!("width: {}em;", self.get_width()),
        ];
        if self.show_transition {
            styles.push(String::from("transition: transform 250ms ease-in-out"))
        }
        return html!{
            <div class="kana-line">
                <div class="kana-line-container" >
                    <div
                        class="kana-line-scroll-container"
                        ontransitionend=self.link.callback(|_| Self::Message::ResampleContent)
                        style=styles.join(" ")
                    >
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

        if let Some(current) = self.contents.get_mut(self.index) {
            current.check(self.props.text.to_lowercase());
            if let ContentState::Answered(is_correct) = current.state {
                self.onshift.emit(is_correct);
                self.index += 1;
                self.show_transition = true;
                return true
            }
        }
        false
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
            self.show_transition = false;
            return true
        }
        false
    }

    fn get_margin(&self) -> f32 {
        if self.contents.is_empty() {
            return 0f32
        }

        let mut margin = self.contents
            .iter()
            .take(self.index)
            .fold(0, |s, c| s + c.size)
            as f32;
        if let Some(current) = self.contents.get(self.index) {
            margin += current.size as f32 / 2.0
        }
        return margin
    }

    fn get_width(&self) -> usize {
        self.contents.iter().fold(0, |s, c| s + c.size)
    }
}

#[derive(Default)]
struct ContentGenerator {
    translations: KanaTranslationList,
}

impl ContentGenerator {
    fn random(&self) -> Option<Content> {
        if self.translations.is_empty() {
            return None
        }
        let translation = unsafe {
            let index = rand::random::<usize>() % self.translations.len();
            <&crate::KanaTranslation>::clone(self.translations.get_unchecked(index))
        };

        Some(Content::new(translation))
    }

    fn set_translations(&mut self, translations: KanaTranslationList) {
        self.translations = translations;
    }
}


enum ContentState {
    Unanswered,
    Answered(bool),
}

struct Content {
    translation: &'static crate::KanaTranslation,
    size: usize,
    state: ContentState,
}

impl Content {
    fn new(translation: &'static crate::KanaTranslation) -> Self {
        let size = translation.kana.chars().count();
        Self {
            translation,
            size,
            state: ContentState::Unanswered,
        }
    }

    fn check(&mut self, text: String) {
        if text.is_empty() {
            return
        }
        if "あいうえおんアイウエオ".contains(&self.translation.kana)
            || (text.len() == 1 && "aeiuo".contains(&text))
            || text.len() >= self.translation.romanji.len()
        {
            let mut is_correct = self.translation.romanji == text;
            if let Some(alt_romanji) = self.translation.alt_romanji {
                is_correct |= alt_romanji == text;
            }
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
                    <span class="small-above">
                        { <&'static str>::clone(&self.translation.romanji) }
                    </span>
                };
            }
        }

        return html! {
            <span class=class style=style>
                { small_above }
                { <&'static str>::clone(&self.translation.kana) }
            </span>
        }
    }
}
