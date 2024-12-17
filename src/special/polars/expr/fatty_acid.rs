use crate::r#const::relative_atomic_mass::{C, H, O};
use polars::prelude::*;

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
/// Fatty acid indices
pub trait Indices: FattyAcidExt
where
    Self: Sized,
{
    /// SFA
    fn sfa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().eq(0))
    }

    /// UFA
    fn ufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().neq(0))
    }

    /// MUFA
    fn mufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().eq(1))
    }

    /// PUFA
    fn pufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().gt(1))
    }

    /// PUFA n-3, n-6, ...
    fn pufan(&self, n: u8) -> Expr {
        self.unsaturated().list().eval(
            col("")
                .struct_()
                .field_by_name("Index")
                .eq(self.carbons() - lit(n)),
            true,
        )
    }

    /// IA (Index of atherogenicity)
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    fn ia(&self, expr: Expr) -> Expr {
        (self.c12u0(expr.clone()) + lit(4) * self.c14u0(expr.clone()) + self.c16u0(expr.clone()))
            / self.ufa(expr)
    }

    /// IT (Index of thrombogenicity)
    /// (C14:0 + C16:0 + C18:0)/((0.5*ΣMUFA) + (0.5*ΣPUFAN6) + (3*ΣPUFAN3) + (n-3/n-6)]
    fn it(&self, expr: Expr) -> Expr {
        (self.c14u0(expr.clone()) + self.c16u0(expr.clone()) + self.c18u0(expr.clone()))
            / (lit(0.5) * self.ufa(expr) + lit(0.5))
    }

    /// HH (Hypocholesterolemic/hypercholesterolemic ratio)
    /// (cis-C18:1 + ΣPUFA)/(C12:0 + C14:0 + C16:0)
    fn hh(&self, expr: Expr) -> Expr {
        (self.c18u1(expr.clone()) + self.pufa(expr.clone()))
            / (self.c12u0(expr.clone()) + self.c14u0(expr.clone()) + self.c16u0(expr))
    }

    /// HPI (Health-promoting index)
    /// ΣUFA / (C12:0 + 4 * C14:0 + C16:0)
    fn hpi(&self, expr: Expr) -> Expr {
        self.ufa(expr.clone())
            / (self.c12u0(expr.clone()) + lit(4) * self.c14u0(expr.clone()) + self.c16u0(expr))
    }

    /// UI (Unsaturation index)
    /// 1 * (% monoenoics) + 2 * (%dienoics) + 3 * (% trienoics) + 4 *(% tetraenoics) + 5 * (%pentaenoics) + 6 * (% hexaenoics)
    fn ui(&self, expr: Expr) -> Expr {
        lit("ХУЕТА")
    }

    /// FLQ (Fish lipid quality/flesh lipid quality)
    /// (EPA + DHA) / ΣFA
    fn flq(&self, expr: Expr) -> Expr {
        (self.epa(expr.clone()) + self.dha(expr.clone())) / expr.sum()
    }

    /// TFA (Trans fatty acid)
    ///
    fn tfa(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(3)))
    }
}

pub trait Single: FattyAcidExt {
    /// EPA (Eicosapentaenoic acid)
    /// C22:6 n-3
    fn epa(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(22).and(self.unsaturated().eq(6)))
    }

    /// DHA (Docosahexaenoic acid)
    /// C20:5 n-3
    fn dha(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(20).and(self.unsaturated().eq(5)))
    }

    /// LA (Linoleic acid)
    /// C18:2 n-6
    fn la(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(2)))
    }

    /// ALA (α-Linolenic acid)
    /// C18:3 n-3
    fn ala(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(3)))
    }

    fn c12u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(12).and(self.unsaturated().eq(0)))
    }

    fn c14u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(14).and(self.unsaturated().eq(0)))
    }

    fn c16u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(16).and(self.unsaturated().eq(0)))
    }

    fn c18u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(0)))
    }

    fn c18u1(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(1)))
    }
}

impl<T: FattyAcidExt> Single for T {}

/// Fatty acid
pub trait FattyAcidExt {
    /// Carbons
    fn carbons(&self) -> Expr;

    /// Unsaturation
    fn unsaturation(&self) -> Expr;

    /// Unsaturated
    fn unsaturated(&self) -> Expr;

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
        self.unsaturation().eq(0)
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

    fn unsaturated(&self) -> Expr {
        self.0
            .clone()
            .struct_()
            .field_by_name("Unsaturated")
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .len()
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
