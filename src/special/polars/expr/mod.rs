use self::fatty_acid::FattyAcidExpr;
use polars::prelude::*;

/// Extension methods for [`Expr`]
pub trait ExprExt {
    fn fatty_acid(self) -> FattyAcidExpr;
}

impl ExprExt for Expr {
    fn fatty_acid(self) -> FattyAcidExpr {
        FattyAcidExpr(self)
    }
}

pub mod fatty_acid;
