use crate::special::polars::series::{
    SeriesExt as _, fatty_acids::FattyAcidSeries, identifier::Identifier,
};
use polars::prelude::*;

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> FattyAcidSeries;

    fn identifier(&self) -> Identifier;
}

impl ColumnExt for Column {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self.as_materialized_series().fatty_acid()
    }

    fn identifier(&self) -> Identifier {
        self.as_materialized_series().identifier()
    }
}
