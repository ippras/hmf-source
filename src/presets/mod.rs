pub(crate) use self::mature_milk::MATURE_MILK;

use polars::prelude::*;
use std::sync::LazyLock;

// [DOI:10.1021/jf903048p](https://doi.org/10.1021/jf903048p)
pub(crate) static HMF_1: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-1.hmf.ron")).expect("deserialize HMF-1.hmf.ron")
});
pub(crate) static HMF_2: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-2.hmf.ron")).expect("deserialize HMF-2.hmf.ron")
});
pub(crate) static HMF_3: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-3.hmf.ron")).expect("deserialize HMF-3.hmf.ron")
});
pub(crate) static HMF_4: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("HMF-4.hmf.ron")).expect("deserialize HMF-4.hmf.ron")
});
// [DOI:10.1016/j.algal.2018.11.004](https://doi.org/10.1016/j.algal.2018.11.004)
pub(crate) static CV_15: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("CV-15.hmf.ron")).expect("deserialize CV-15.hmf.ron")
});
pub(crate) static CZ_30412: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("CZ-30412.hmf.ron")).expect("deserialize CZ-30412.hmf.ron")
});
pub(crate) static CP_9: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("CP-9.hmf.ron")).expect("deserialize CP-9.hmf.ron")
});
pub(crate) static CV_395: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("CV-395.hmf.ron")).expect("deserialize CV-395.hmf.ron")
});

pub(crate) static ISO_FJ: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("ISO-FJ.hmf.ron")).expect("deserialize ISO-FJ.hmf.ron")
});
// IPPRAS
pub(crate) static C70_CONTROL: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("C70-Control.hmf.ron")).expect("deserialize C70-Control.hmf.ron")
});
pub(crate) static C70_H2O2: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("C70-H2O2.hmf.ron")).expect("deserialize C70-H2O2.hmf.ron")
});
pub(crate) static C70_NACL: LazyLock<DataFrame> = LazyLock::new(|| {
    ron::de::from_str(include_str!("C70-NaCl.hmf.ron")).expect("deserialize C70-NaCl.hmf.ron")
});

mod mature_milk;
