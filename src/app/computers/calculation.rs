use crate::{app::panes::calculation::control::Settings, special::mature_milk::MATURE_MILK};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::hash::{Hash, Hasher};

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.data_frame.clone().lazy();
        let other = MATURE_MILK.clone().lazy().select([
            col("FattyAcid").struct_().field_by_names(["*"]),
            col("StereospecificNumber123"),
            col("StereospecificNumber2"),
        ]);
        if !key.data_frame.is_empty() {
            lazy_frame = lazy_frame
                .unnest(["FattyAcid"])
                .join(
                    other,
                    &[col("Carbons"), col("Unsaturated")],
                    &[col("Carbons"), col("Unsaturated")],
                    JoinArgs::new(JoinType::Left),
                )
                .with_columns([
                    as_struct(vec![col("Carbons"), col("Unsaturated")]).alias("FattyAcid")
                ]);
            // SN123 A B D
            lazy_frame = lazy_frame.with_columns([
                col("StereospecificNumber123")
                    .struct_()
                    .field_by_name("ReferenceRange")
                    .alias("A"),
                col("TAG").alias("B"),
                col("StereospecificNumber123")
                    .struct_()
                    .field_by_name("Median")
                    .alias("D"),
            ]);
            lazy_frame = calculate(lazy_frame, key.settings);
            lazy_frame = lazy_frame.with_columns([as_struct(vec![
                col("A"),
                col("B"),
                col("C"),
                col("D"),
                col("E"),
                col("F"),
            ])
            .alias("SN123")]);
            lazy_frame =
                lazy_frame.drop([col("A"), col("B"), col("C"), col("D"), col("E"), col("F")]);
            // SN2 A B D
            lazy_frame = lazy_frame.with_columns([
                col("StereospecificNumber2")
                    .struct_()
                    .field_by_name("ReferenceRange")
                    .alias("A"),
                col("MAG").alias("B"),
                col("StereospecificNumber2")
                    .struct_()
                    .field_by_name("Median")
                    .alias("D"),
            ]);
            lazy_frame = calculate(lazy_frame, key.settings);
            lazy_frame = lazy_frame.with_columns([as_struct(vec![
                col("A"),
                col("B"),
                col("C"),
                col("D"),
                col("E"),
                col("F"),
            ])
            .alias("SN2")]);
            lazy_frame =
                lazy_frame.drop([col("A"), col("B"), col("C"), col("D"), col("E"), col("F")]);
            lazy_frame = lazy_frame.with_columns([(col("SN123").struct_().field_by_name("E")
                + col("SN2").struct_().field_by_name("E"))
            .alias("F")]);
            lazy_frame = lazy_frame.select([col("SN123"), col("SN2"), col("F")]);
        }
        // println!("lazy_frame1: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = lazy_frame.with_row_index("Index", None);
        lazy_frame.collect()
    }
}

fn calculate(mut lazy_frame: LazyFrame, settings: &Settings) -> LazyFrame {
    // A
    lazy_frame =
        lazy_frame.with_columns([when(col("B").lt(col("A").struct_().field_by_name("Min")))
            .then(col("A").struct_().field_by_name("Min"))
            .when(col("B").gt(col("A").struct_().field_by_name("Max")))
            .then(col("A").struct_().field_by_name("Max"))
            .otherwise(col("B"))
            .alias("A")]);
    // C
    lazy_frame = lazy_frame.with_columns([((col("B") - col("A")).abs() / col("A"))
        .fill_nan(lit(0))
        .alias("C")]);
    if settings.round > 0 {
        lazy_frame = lazy_frame.with_column(col("C").round(settings.round));
    }
    lazy_frame = lazy_frame.with_columns([(col("C") * col("D") / col("D").sum()).alias("1")]);
    // E
    lazy_frame =
        lazy_frame.with_columns([(lit(50) * col("C") * col("D") / col("D").sum()).alias("E")]);
    // F
    lazy_frame = lazy_frame.with_columns([(lit(50) - col("E").sum()).alias("F")]);
    lazy_frame
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
        // self.settings.from.hash(state);
        // self.settings.normalize.hash(state);
        // self.settings.unsigned.hash(state);
        // self.settings.christie.hash(state);
        // self.settings.ddof.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
