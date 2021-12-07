use strum::{Display, EnumIter};


#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Display, EnumIter)]
pub enum ChartVersion {
    Basic,
    Dakuon,
    Combo,
}

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Display, EnumIter)]
pub enum SyllabaryVersion {
    Hirigana,
    Katakana,
}
