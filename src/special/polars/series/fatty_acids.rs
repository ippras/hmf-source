use crate::special::fatty_acid::{FattyAcid, Unsaturated};
use polars::prelude::*;

/// Fatty acid series
#[derive(Clone, Debug)]
pub struct FattyAcidSeries {
    pub carbons: Series,
    pub unsaturated: Series,
}

impl FattyAcidSeries {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        let r#struct = series.struct_()?;
        let carbons = r#struct.field_by_name("Carbons")?;
        let unsaturated = r#struct.field_by_name("Unsaturated")?;
        Ok(Self {
            carbons,
            unsaturated,
        })
    }

    pub fn len(&self) -> usize {
        self.carbons.len()
    }

    pub fn get(&self, index: usize) -> PolarsResult<Option<FattyAcid>> {
        let Some(carbons) = self.carbons.u8()?.get(index) else {
            return Ok(None);
        };
        let mut unsaturated = Vec::new();
        if let Some(series) = self.unsaturated.list()?.get_as_series(index) {
            let unsaturated_series = UnsaturatedSeries::new(&series)?;
            for index in 0..unsaturated_series.len() {
                unsaturated.push(unsaturated_series.get(index)?);
            }
        };
        unsaturated.sort_by_cached_key(|unsaturated| {
            (
                unsaturated.unsaturation,
                unsaturated.index,
                unsaturated.isomerism,
            )
        });
        Ok(Some(FattyAcid {
            carbons,
            unsaturated,
        }))
    }

    pub fn unsaturated(&self, index: usize) -> PolarsResult<Option<UnsaturatedSeries>> {
        let Some(unsaturated) = self.unsaturated.list()?.get_as_series(index) else {
            return Ok(None);
        };
        Ok(Some(UnsaturatedSeries::new(&unsaturated)?))
    }
}

/// Unsaturated series
#[derive(Clone, Debug)]
pub struct UnsaturatedSeries {
    pub index: Series,
    pub isomerism: Series,
    pub unsaturation: Series,
}

impl UnsaturatedSeries {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        let r#struct = series.struct_()?;
        let index = r#struct.field_by_name("Index")?;
        let isomerism = r#struct.field_by_name("Isomerism")?;
        let unsaturation = r#struct.field_by_name("Unsaturation")?;
        Ok(Self {
            index,
            isomerism,
            unsaturation,
        })
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn get(&self, index: usize) -> PolarsResult<Unsaturated> {
        Ok(Unsaturated {
            index: self.index.u8()?.get(index),
            isomerism: self
                .isomerism
                .i8()?
                .get(index)
                .and_then(|isomerism| isomerism.try_into().ok()),
            unsaturation: self
                .unsaturation
                .u8()?
                .get(index)
                .and_then(|unsaturation| unsaturation.try_into().ok()),
        })
    }
}
