use std::time::Duration;

use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
use yewtil::NeqAssign;


pub enum ClockMessage {
    Tick
}

#[derive(Clone, PartialEq, Properties)]
pub struct ClockProperties {
    #[prop_or_default]
    pub active: bool,
}

pub struct Clock {
    active: bool,
    seconds: usize,
    _handler: IntervalTask,
}

impl Component for Clock {
    type Message = ClockMessage;
    type Properties = ClockProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _handler = IntervalService::spawn(
            Duration::from_secs(1),
            link.callback(|_| Self::Message::Tick)
        );
        Self { active: props.active, seconds: 0, _handler }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Tick => {
                self.seconds += 1;
                self.active
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = self.active.neq_assign(props.active);
        if should_render {
            self.seconds = 0;
        }
        should_render
    }

    fn view(&self) -> Html {
        let time = {
            let minutes = self.seconds / 60;
            let seconds = self.seconds % 60;
            format!("{:02}:{:02}", minutes, seconds)
        };
        return html!{
            <div class="clock">
                { crate::components::icons::clock_solid() }
                {time}
            </div>
        }
    }
}
