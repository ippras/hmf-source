use super::preset;
use metadata::MetaDataFrame;
use std::sync::LazyLock;

// pub(crate) static MATURE_MILK: LazyLock<DataFrame> = LazyLock::new(|| {
//     ron::de::from_str(include_str!("MatureMilk.ron")).expect("deserialize MatureMilk.ron")
// });

pub(crate) static MATURE_MILK: LazyLock<MetaDataFrame> =
    preset!("src/presets/10.1021/jf903048p/MatureMilk.ipc");
