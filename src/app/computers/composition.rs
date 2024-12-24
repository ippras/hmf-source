use crate::{app::panes::calculation::control::Settings, presets::MATURE_MILK};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::fatty_acid::polars::{ExprExt as _, FindByName};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Composition computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Composition computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.data_frame.clone().lazy();
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = lazy_frame.with_columns([col("TAG")
            .filter(col("FattyAcid").fatty_acid().linoleic())
            .sum()]);
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = lazy_frame.with_row_index("Index", None);
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_frame.shape().hash(state);
        for series in self.data_frame.iter() {
            for value in series.iter() {
                value.hash(state);
            }
        }
        self.settings.hash(state);
    }
}

/// Composition value
type Value = DataFrame;
