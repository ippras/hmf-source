use self::{fatty_acids::FattyAcidSeries, identifier::Identifier};
use polars::prelude::*;

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn identifier(&self) -> Identifier;

    fn fatty_acid(&self) -> FattyAcidSeries;
}

impl SeriesExt for Series {
    fn identifier(&self) -> Identifier {
        Identifier::new(self).expect(r#"Expected "ID" series"#)
    }

    fn fatty_acid(&self) -> FattyAcidSeries {
        FattyAcidSeries::new(self).expect(r#"Expected "FattyAcid" series"#)
    }
}

pub mod fatty_acids;
pub mod identifier;
