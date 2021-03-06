pub mod screen;
mod clock;
mod kana;
mod romanji;
mod score;

use clock::Clock;
use kana::KanaLine;
use romanji::RomanjiLine;
use score::Score;
use screen::GameMessage;

pub use screen::GameScreen;

pub type KanaTranslationList = std::rc::Rc<Vec<&'static crate::KanaTranslation>>;
