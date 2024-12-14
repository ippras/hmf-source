use crate::r#const::relative_atomic_mass::{C, H, O};
use polars::prelude::*;

/// Fatty acid
pub trait FattyAcidExt {
    /// Carbons
    fn carbons(&self) -> Expr;

    /// Unsaturation
    fn unsaturation(&self) -> Expr;

    /// Bounds
    fn bounds(&self) -> Expr {
        (self.carbons() - lit(1)).clip_min(lit(0))
    }

    /// ECN (Equivalent carbon number)
    ///
    /// `ECN = C - 2U`
    fn ecn(&self) -> Expr {
        self.carbons() - lit(2) * self.unsaturation()
    }

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    fn hydrogens(&self) -> Expr {
        lit(2) * self.carbons() - lit(2) * self.unsaturation()
    }

    /// Mass
    fn mass(&self) -> Expr {
        self.carbons() * lit(C) + self.hydrogens() * lit(H) + lit(2) * lit(O)
    }

    /// Saturated
    fn saturated(&self) -> Expr {
        self.unsaturation().eq(lit(0))
    }

    /// [`bounds`]
    fn b(&self) -> Expr {
        self.bounds()
    }

    /// [`carbons`]
    fn c(&self) -> Expr {
        self.carbons()
    }

    /// [`hydrogens`]
    fn h(&self) -> Expr {
        self.hydrogens()
    }

    /// [`saturated`]
    fn s(&self) -> Expr {
        self.saturated()
    }

    /// [`unsaturation`]
    fn u(&self) -> Expr {
        self.unsaturation()
    }
}

/// Fatty acids [`Expr`]
#[derive(Clone)]
pub struct FattyAcidExpr(pub(super) Expr);

impl FattyAcidExt for FattyAcidExpr {
    fn carbons(&self) -> Expr {
        self.0.clone().struct_().field_by_name("Carbons")
    }

    fn unsaturation(&self) -> Expr {
        self.0
            .clone()
            .struct_()
            .field_by_name("Unsaturated")
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .sum()
    }
    // /// Double bounds count
    // pub fn d(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Doubles")
    //         .list()
    //         .len()
    // }

    // /// Triple bounds count
    // pub fn t(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Triples")
    //         .list()
    //         .len()
    // }

    // pub fn r#type(self) -> Expr {
    //     ternary_expr(self.saturated(), lit("S"), lit("U"))
    // }

    // pub fn unsaturated(self) -> Expr {
    //     self.saturated().not()
    // }

    // pub fn unsaturation(self) -> Expr {
    //     self.d() + lit(2) * self.t()
    // }
}

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}
