pub mod components;
mod syllabograms;
pub mod translations;
mod utils;

pub use components::App;
use syllabograms::Syllabograms;

pub trait SyllabogramSegmentation {
    fn syllabograms(&self) -> Syllabograms;
}

impl SyllabogramSegmentation for str {

    #[inline]
    fn syllabograms(&self) -> Syllabograms {
        syllabograms::new_syllabograms(self)
    }
}
