use crate::{
    app::panes::calculation::settings::Settings, presets::_10_1021_jf903048p::MATURE_MILK_FAT,
};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use polars_ext::{ExprExt, column};
use std::{
    hash::{Hash, Hasher},
    iter::zip,
};

const MAX: f64 = 50.0;

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.data_frame.clone().lazy();
        let other = MATURE_MILK_FAT.data.clone().lazy().select([
            col("FattyAcid").hash(),
            col("FattyAcid"),
            col("StereospecificNumber123").alias("Target123"),
            col("StereospecificNumber2").alias("Target2"),
        ]);
        if !key.data_frame.is_empty() {
            lazy_frame = lazy_frame
                .select([
                    col("FattyAcid").hash(),
                    col("FattyAcid"),
                    col("StereospecificNumber123").alias("Source123"),
                    col("StereospecificNumber2").alias("Source2"),
                ])
                .join(
                    other,
                    &[col("Hash"), col("FattyAcid")],
                    &[col("Hash"), col("FattyAcid")],
                    JoinArgs::new(JoinType::Left),
                )
                .drop(["Hash"])
                .with_column(col("FattyAcid"))
                .with_column(if key.settings.relative {
                    lit(100) * col("Source2") / col("Source123") / lit(3)
                } else {
                    col("Source2")
                });
            let output_type = GetOutput::from_type(DataType::Struct(vec![
                Field::new(
                    "Data".into(),
                    DataType::Struct(vec![
                        Field::new("A".into(), DataType::Float64),
                        Field::new("B".into(), DataType::Float64),
                        Field::new("C".into(), DataType::Float64),
                        Field::new("D".into(), DataType::Float64),
                        Field::new("E".into(), DataType::Float64),
                        Field::new("F".into(), DataType::Float64),
                    ]),
                ),
                Field::new(
                    "Meta".into(),
                    DataType::Struct(vec![
                        Field::new("Min".into(), DataType::Float64),
                        Field::new("Max".into(), DataType::Float64),
                        Field::new("Sum".into(), DataType::Float64),
                    ]),
                ),
            ]));
            println!("calculate0: {}", lazy_frame.clone().collect().unwrap());
            lazy_frame =
                lazy_frame.with_columns([as_struct(vec![col("Source123"), col("Target123")])
                    .apply(column(abcdef(&key.settings)), output_type.clone())
                    .alias("StereospecificNumber123")]);
            println!("calculate1: {}", lazy_frame.clone().collect().unwrap());
            lazy_frame = lazy_frame.with_columns([as_struct(vec![col("Source2"), col("Target2")])
                .apply(column(abcdef(&key.settings)), output_type)
                .alias("StereospecificNumber2")]);
            lazy_frame = lazy_frame.with_columns([(col("StereospecificNumber123")
                .struct_()
                .field_by_name("Data")
                .struct_()
                .field_by_name("E")
                + col("StereospecificNumber2")
                    .struct_()
                    .field_by_name("Data")
                    .struct_()
                    .field_by_name("E"))
            .alias("F")]);
            println!("calculate !!!!: {}", lazy_frame.clone().collect().unwrap());
            lazy_frame = lazy_frame.select([
                col("StereospecificNumber123"),
                col("StereospecificNumber2"),
                col("F"),
            ]);
        }
        lazy_frame = lazy_frame.with_row_index("Index", None);
        lazy_frame.collect()
    }
}

/// Fields
/// * Value
/// * Median + Reference range
fn abcdef(settings: &Settings) -> impl Fn(&Series) -> PolarsResult<Series> + 'static {
    let settings = settings.clone();
    move |series| {
        let fields = series.struct_()?.fields_as_series();
        let b = &fields[0];
        let r#struct = match fields[1].name().as_str() {
            "Target123" => &fields[1],
            "Target2" if settings.relative => &fields[1].struct_()?.field_by_name("Relative")?,
            "Target2" => &fields[1].struct_()?.field_by_name("Absolute")?,
            _ => unreachable!(),
        };
        let d = r#struct.struct_()?.field_by_name("Median")?;
        let reference_range = r#struct.struct_()?.field_by_name("ReferenceRange")?;
        let min = reference_range.struct_()?.field_by_name("Min")?;
        let max = reference_range.struct_()?.field_by_name("Max")?;
        let sum = d.f64()?.sum();
        let mut builder = Builder::new(series.len());
        for (((b, d), min), max) in zip(b.f64()?, d.f64()?).zip(min.f64()?).zip(max.f64()?) {
            let Some((b, d, min, max, sum)) = (|| Some((b?, d?, min?, max?, sum?)))() else {
                builder.append_null();
                continue;
            };
            // A
            let a = if b < min {
                min
            } else if b > max {
                max
            } else {
                b
            };
            builder.a.append_value(a);
            builder.b.append_value(b);
            // C
            let mut c = if a != 0.0 { (b - a).abs() / a } else { 0.0 };
            if settings.round > 0 {
                let order = 10f64.powi(settings.round as _);
                c = (c * order).round() / order;
            }
            builder.c.append_value(c);
            builder.d.append_value(d);
            // E
            let e = MAX * c * d / sum;
            builder.e.append_value(e);
            // F
            builder.f -= e;
        }
        let data = builder.finish()?;
        let meta = StructChunked::from_series(
            "Meta".into(),
            series.len(),
            [
                min,
                max,
                Scalar::new(DataType::Float64, AnyValue::from(sum)).into_series("Sum".into()),
            ]
            .iter(),
        )?
        .into_series();
        Ok(
            StructChunked::from_series(PlSmallStr::EMPTY, series.len(), [data, meta].iter())?
                .into_series(),
        )
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// ABCDEF builder
struct Builder {
    capacity: usize,
    a: PrimitiveChunkedBuilder<Float64Type>,
    b: PrimitiveChunkedBuilder<Float64Type>,
    c: PrimitiveChunkedBuilder<Float64Type>,
    d: PrimitiveChunkedBuilder<Float64Type>,
    e: PrimitiveChunkedBuilder<Float64Type>,
    f: f64,
}

impl Builder {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            a: PrimitiveChunkedBuilder::<Float64Type>::new("A".into(), capacity),
            b: PrimitiveChunkedBuilder::<Float64Type>::new("B".into(), capacity),
            c: PrimitiveChunkedBuilder::<Float64Type>::new("C".into(), capacity),
            d: PrimitiveChunkedBuilder::<Float64Type>::new("D".into(), capacity),
            e: PrimitiveChunkedBuilder::<Float64Type>::new("E".into(), capacity),
            f: MAX,
        }
    }

    // fn append_value(
    //     &mut self,
    //     b: Option<f64>,
    //     d: Option<f64>,
    //     min: Option<f64>,
    //     max: Option<f64>,
    //     sum: Option<f64>,
    // ) {
    //     let Some((b, d, min, max, sum)) = (|| {
    //         let b = b?;
    //         let d = d?;
    //         let min = min?;
    //         let max = max?;
    //         let sum = sum?;
    //         Some((b, d, min, max, sum))
    //     })() else {
    //         self.append_null();
    //         return;
    //     };
    //     // A
    //     let a = if b < min {
    //         min
    //     } else if b > max {
    //         max
    //     } else {
    //         b
    //     };
    //     // C
    //     let mut c = (b - a).abs() / a;
    //     if settings.round > 0 {
    //         let d = settings.round as f64;
    //         c = (c * d).round() / d;
    //     }
    //     // E
    //     let e = 50.0 * c * d / sum;
    //     self.a.append_value(a);
    //     self.b.append_value(b);
    //     self.c.append_value(c);
    //     self.d.append_value(d);
    //     self.e.append_value(e);
    //     // F
    //     self.f -= e;
    // }

    fn append_null(&mut self) {
        self.a.append_null();
        self.b.append_null();
        self.c.append_null();
        self.d.append_null();
        self.e.append_null();
    }

    fn finish(self) -> PolarsResult<Series> {
        Ok(StructChunked::from_series(
            "Data".into(),
            self.capacity,
            [
                self.a.finish().into_series(),
                self.b.finish().into_series(),
                self.c.finish().into_series(),
                self.d.finish().into_series(),
                self.e.finish().into_series(),
                Scalar::new(DataType::Float64, AnyValue::Float64(self.f)).into_series("F".into()),
            ]
            .iter(),
        )?
        .into_series())
    }
    // fn finish(self) -> [Series; 6] {
    //     [
    //         self.a.finish().into_series(),
    //         self.b.finish().into_series(),
    //         self.c.finish().into_series(),
    //         self.d.finish().into_series(),
    //         self.e.finish().into_series(),
    //         Scalar::new(DataType::Float64, AnyValue::Float64(self.f)).into_series("F".into()),
    //     ]
    // }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for series in self.data_frame.iter() {
            for value in series.iter() {
                value.hash(state);
            }
        }
        self.settings.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
