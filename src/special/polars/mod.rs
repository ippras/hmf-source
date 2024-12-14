use self::series::{fatty_acids::FattyAcidSeries, identifier::Identifier, SeriesExt as _};
use crate::{
    r#const::relative_atomic_mass::{C, H, O},
    // utils::polars::ExprExt as _,
};
use column::fatty_acid::ColumnExt as _;
use polars::prelude::*;
use std::{borrow::Borrow, sync::LazyLock};

pub static FATTY_ACIDS_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::from_iter([
        Field::new("Label".into(), DataType::String),
        Field::new("Carbons".into(), DataType::UInt8),
        Field::new("Doubles".into(), DataType::List(Box::new(DataType::Int8))),
        Field::new("Triples".into(), DataType::List(Box::new(DataType::Int8))),
    ])
});

pub static DATA_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::from_iter([
        Field::new(
            "Experimental".into(),
            DataType::Struct(vec![
                Field::new("TAG".into(), DataType::Float64),
                Field::new("DAG1223".into(), DataType::Float64),
                Field::new("MAG2".into(), DataType::Float64),
            ]),
        ),
        Field::new(
            "Theoretical".into(),
            DataType::Struct(vec![
                Field::new("TAG".into(), DataType::Float64),
                Field::new("DAG1223".into(), DataType::Float64),
                Field::new("MAG2".into(), DataType::Float64),
                Field::new("DAG13".into(), DataType::Float64),
                Field::new("DAG13".into(), DataType::Float64),
            ]),
        ),
        Field::new(
            "Calculated".into(),
            DataType::Struct(vec![
                Field::new("TAG".into(), DataType::Float64),
                Field::new("DAG1223".into(), DataType::Float64),
                Field::new("MAG2".into(), DataType::Float64),
            ]),
        ),
        Field::new(
            "EnrichmentFactor".into(),
            DataType::Struct(vec![
                Field::new("MAG2".into(), DataType::Float64),
                Field::new("DAG13".into(), DataType::Float64),
            ]),
        ),
        Field::new(
            "SelectivityFactor".into(),
            DataType::Struct(vec![
                Field::new("MAG2".into(), DataType::Float64),
                Field::new("DAG13".into(), DataType::Float64),
            ]),
        ),
    ])
});

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> FattyAcidSeries;

    fn identifier(&self) -> Identifier;
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self["FattyAcid"].fatty_acid()
    }

    fn identifier(&self) -> Identifier {
        self["ID"].identifier()
    }
}

/// Extension methods for [`Schema`]
pub trait SchemaExt {
    fn names(&self) -> Vec<Expr>;
}

impl SchemaExt for Schema {
    fn names(&self) -> Vec<Expr> {
        self.iter_names_cloned().map(col).collect()
    }
}

pub mod column;
pub mod data_frame;
pub mod expr;
pub mod series;
