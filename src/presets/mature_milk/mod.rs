use polars::prelude::*;
use std::sync::LazyLock;

pub(crate) static MATURE_MILK: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("MatureMilk.ron")).expect("deserialize MatureMilk.ron")
});
