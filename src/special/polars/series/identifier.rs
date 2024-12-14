use super::SeriesExt;
use crate::special::fatty_acid::FattyAcid;
use polars::prelude::*;

/// Identifier
#[derive(Clone, Debug)]
pub struct Identifier {
    labels: Series,
    fatty_acids: Series,
}

impl Identifier {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        let r#struct = series.struct_()?;
        let labels = r#struct.field_by_name("Label")?;
        let fatty_acids = r#struct.field_by_name("FA")?;
        Ok(Self {
            labels,
            fatty_acids,
        })
    }

    pub fn get(&self, index: usize) -> PolarsResult<Option<(&str, FattyAcid)>> {
        let Some(label) = self.labels.str()?.get(index) else {
            return Ok(None);
        };
        let Some(fatty_acid) = self.fatty_acids.fatty_acid().get(index)? else {
            return Ok(None);
        };
        Ok(Some((label, fatty_acid)))
    }
}
