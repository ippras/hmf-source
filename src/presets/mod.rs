use polars::prelude::*;
use std::sync::LazyLock;

pub(crate) static HMF1: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-1.hmf.ron")).expect("deserialize HMF-1.hmf.ron")
});
pub(crate) static HMF2: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-2.hmf.ron")).expect("deserialize HMF-2.hmf.ron")
});
pub(crate) static HMF3: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-3.hmf.ron")).expect("deserialize HMF-3.hmf.ron")
});
