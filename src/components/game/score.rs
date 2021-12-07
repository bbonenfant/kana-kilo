use yew::{Html, html};

const RED: Color = Color { h: 6, s: 78, l: 57 };
const GREEN: Color = Color { h: 145, s: 70, l: 50 };

#[derive(Default)]
pub struct Score {
    correct: usize,
    total: usize,
}

impl Score {

    pub fn color(&self) -> String {
        if self.total == 0 {
            return "".into()
        }
        let percent = (self.correct as f32) / (self.total as f32);
        RED.interpolate_to(&GREEN, percent.powi(4)).as_string()
    }

    pub fn render(&self) -> Html {
        let style = format!("color: {};", self.color());
        return html!{
            <div class="score" style=style>
                { crate::components::icons::square_check_solid() }
                <div class="score-number"> {self.correct} </div>
                <div> {"/"} </div>
                <div class="score-number"> {self.total} </div>
            </div>
        }
    }

    pub fn reset(&mut self) {
        self.correct = 0;
        self.total = 0;
    }

    pub fn tally(&mut self, is_correct: bool) {
        self.total += 1;
        if is_correct {
            self.correct += 1;
        }
    }
}

struct Color {
    h: u8,
    s: u8,
    l: u8,
}

impl Color {
    fn as_string(&self) -> String {
        format!("hsl({},{}%,{}%)", self.h, self.s, self.l)
    }

    fn interpolate_to(&self, other: &Self, percent: f32) -> Color {
        Color {
            h: interpolate(&self.h, &other.h, &percent),
            s: interpolate(&self.s, &other.s, &percent),
            l: interpolate(&self.l, &other.l, &percent),
        }
    }
}

fn interpolate(from: &u8, to: &u8, percent: &f32) -> u8 {
    let from = *from as f32;
    let to = *to as f32;
    (from + (percent * (to - from))) as u8
}