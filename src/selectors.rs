use scraper::Selector;

pub struct TournamentSelector(Selector);
pub struct TournamentPageSelector(Selector);
pub struct PairSelector(Selector);
pub struct CodeSelector(Selector);
pub struct PlayerSelector(Selector);
pub struct TournamentDateSelector(Selector);
pub struct CodeNameCircoloSelector(Selector);

impl TournamentSelector {
    pub fn new() -> Self {
        Self(Selector::parse("tr.ALTbase20").unwrap())
    }
}
impl TournamentPageSelector {
    pub fn new() -> Self {
        Self(Selector::parse("a").unwrap())
    }
}
impl PairSelector {
    pub fn new() -> Self {
        Self(Selector::parse("tr.BGCLibere.ALTbase25,tr.BGCTDLibere.ALTbase25").unwrap())
    }
}
//impl CodeSelector {
//    pub fn new() -> Self {
//        Self(Selector::parse("td.COLceleste").unwrap())
//    }
//}
//impl PlayerSelector {
//    pub fn new() -> Self {
//        Self(Selector::parse("td.Capitalize.POSbase0").unwrap())
//    }
//}
impl TournamentDateSelector {
    pub fn new() -> Self {
        Self(Selector::parse("b").unwrap())
    }
}
impl CodeNameCircoloSelector {
    pub fn new() -> Self {
        Self(
            Selector::parse("td.Capitalize.POSbase0, td.COLceleste, td.Capitalize.POSbase0>span")
                .unwrap(),
        )
    }
}

macro_rules! implement_deref {
    ($t:ty) => {
        impl std::ops::Deref for $t {
            type Target = scraper::Selector;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

implement_deref!(TournamentSelector);
implement_deref!(TournamentPageSelector);
implement_deref!(PairSelector);
implement_deref!(CodeSelector);
implement_deref!(PlayerSelector);
implement_deref!(TournamentDateSelector);
implement_deref!(CodeNameCircoloSelector);
