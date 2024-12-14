use self::{index::Notation, isomerism::Elision};
use crate::r#const::relative_atomic_mass::{C, H, O};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::{self, Formatter},
};

pub const ID: Options = Options {
    separators: Separators {
        c: "c",
        u: "u",
        i: ["", ""],
    },
    notation: Notation::Prefix,
    elision: Elision::Explicit,
};

pub const COMMON: Options = Options {
    separators: Separators {
        c: "",
        u: ":",
        i: ["Δ", ","],
    },
    notation: Notation::Suffix,
    elision: Elision::Implicit,
};

pub macro fatty_acid($c:expr $(; $($i:expr),*)*) {{
    assert!($c > 0);
    #[allow(unused_mut)]
    let mut fatty_acid = FattyAcid::new($c);
    let mut _count = 0;
    $(
        _count += 1;
        $(
            assert!($i != 0);
            assert!($i < $c);
            let r#i8 = ($i as i8);
            let unsaturated = Unsaturated {
                unsaturation: Unsaturation::try_from(_count).ok(),
                index: (r#i8 != 0).then_some(r#i8.abs() as _) ,
                isomerism: Isomerism::try_from(r#i8).ok(),
            };
            fatty_acid.unsaturated.push(unsaturated);
        )*
    )*
    fatty_acid
}}

/// Fatty acid
pub trait FattyAcidExt {
    /// Carbon
    fn c(&self) -> u8 {
        self.b() + 1
    }

    /// Hydrogen
    ///
    /// `H = 2C - 2U`
    fn h(&self) -> u8 {
        2 * self.c() - 2 * self.u()
    }

    /// Fatty acid ECN (Equivalent carbon number)
    ///
    /// `ECN = C - 2U`
    fn ecn(&self) -> u8 {
        self.c() - 2 * self.u()
    }

    /// Mass
    fn mass(&self) -> f64 {
        self.c() as f64 * C + self.h() as f64 * H + 2. * O
    }

    /// Saturated
    fn s(&self) -> bool {
        self.u() == 0
    }

    /// Bounds
    fn b(&self) -> u8;

    /// Unsaturated bounds
    fn u(&self) -> u8;
}

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FattyAcid {
    pub carbons: u8,
    pub unsaturated: Vec<Unsaturated>,
}

impl FattyAcid {
    pub const fn new(carbons: u8) -> Self {
        Self {
            carbons,
            unsaturated: Vec::new(),
        }
    }

    fn sort(&mut self) {
        self.unsaturated
            .sort_by_cached_key(|bound| (bound.unsaturation, bound.isomerism, bound.index));
    }
}

impl FattyAcidExt for &FattyAcid {
    fn b(&self) -> u8 {
        self.carbons.saturating_sub(1)
    }

    fn u(&self) -> u8 {
        self.unsaturated.iter().fold(0, |sum, bound| {
            match bound.unsaturation.unwrap_or_default() {
                Unsaturation::One => sum + 1,
                Unsaturation::Two => sum + 2,
            }
        })
    }
}

/// Unsaturated
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Unsaturated {
    pub index: Option<u8>,
    pub isomerism: Option<Isomerism>,
    pub unsaturation: Option<Unsaturation>,
}

/// Isomerism
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Isomerism {
    Cis = 1,
    Trans = -1,
}

impl From<Isomerism> for i8 {
    fn from(value: Isomerism) -> Self {
        match value {
            Isomerism::Cis => 1,
            Isomerism::Trans => -1,
        }
    }
}

impl TryFrom<i8> for Isomerism {
    type Error = i8;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value.is_positive() {
            Ok(Self::Cis)
        } else if value.is_negative() {
            Ok(Self::Trans)
        } else {
            Err(value)
        }
    }
}

/// Unsaturation
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Unsaturation {
    #[default]
    One = 1,
    Two = 2,
}

impl TryFrom<u8> for Unsaturation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(value),
        }
    }
}

/// Display with options
pub trait DisplayWithOptions {
    fn display(&self, options: Options) -> Display<&Self>
    where
        Self: Sized;
}

impl<T: Borrow<FattyAcid>> DisplayWithOptions for T {
    fn display(&self, options: Options) -> Display<&T> {
        Display::new(self, options)
    }
}

/// Fatty acid display
#[derive(Clone, Debug)]
pub struct Display<T> {
    fatty_acid: T,
    options: Options,
}

impl<T> Display<T> {
    pub fn new(fatty_acid: T, options: Options) -> Self {
        Self {
            fatty_acid,
            options,
        }
    }
}

// impl<T: Borrow<FattyAcid>> fmt::Display for Display<T> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let fatty_acid = self.fatty_acid.borrow();
//         f.write_str(self.options.separators.c)?;
//         fmt::Display::fmt(&fatty_acid.carbons, f)?;
//         let point = fatty_acid
//             .unsaturated
//             .partition_point(|unsaturated| unsaturated.unsaturation == Some(Unsaturation::One));
//         let doubles = &fatty_acid.unsaturated.as_slice()[..point];
//         let triples = &fatty_acid.unsaturated.as_slice()[point..];
//         println!("fatty_acid.unsaturated {:?}", fatty_acid.unsaturated,);
//         // unsaturated
//         f.write_str(self.options.separators.u)?;
//         fmt::Display::fmt(&doubles.len(), f)?;
//         if !triples.is_empty() {
//             f.write_str(self.options.separators.u)?;
//             fmt::Display::fmt(&triples.len(), f)?;
//         }
//         if f.alternate() {
//             let mut iter = doubles.into_iter().chain(triples);
//             // if let Some(unsaturated) = iter.next() {
//             //     f.write_str(self.options.separators.i[0])?;
//             //     fmt::Display::fmt(
//             //         &index::Display::new(
//             //             unsaturated.index + 1,
//             //             isomerism::Display::new(unsaturated.isomerism, self.options.elision),
//             //             self.options.notation,
//             //         ),
//             //         f,
//             //     )?;
//             //     for unsaturated in iter {
//             //         f.write_str(self.options.separators.i[1])?;
//             //         fmt::Display::fmt(
//             //             &index::Display::new(
//             //                 unsaturated.index + 1,
//             //                 isomerism::Display::new(unsaturated.isomerism, self.options.elision),
//             //                 self.options.notation,
//             //             ),
//             //             f,
//             //         )?;
//             //     }
//             // }
//         }
//         Ok(())
//     }
// }
impl<T: Borrow<FattyAcid>> fmt::Display for Display<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let fatty_acid = self.fatty_acid.borrow();
        f.write_str(self.options.separators.c)?;
        fmt::Display::fmt(&fatty_acid.carbons, f)?;
        f.write_str(self.options.separators.u)?;
        fmt::Display::fmt(&fatty_acid.unsaturated.len(), f)?;
        if f.alternate() {
            let mut iter = fatty_acid.unsaturated.iter();
            if let Some(unsaturated) = iter.next() {
                if let Some(index) = unsaturated.index {
                    f.write_str(self.options.separators.i[0])?;
                    fmt::Display::fmt(&index, f)?;
                }
                // fmt::Display::fmt(
                //     &index::Display::new(
                //         unsaturated.index.map(|index| index + 1),
                //         isomerism::Display::new(unsaturated.isomerism, self.options.elision),
                //         self.options.notation,
                //     ),
                //     f,
                // )?;
                for unsaturated in iter {
                    f.write_str(self.options.separators.i[1])?;
                    if let Some(index) = unsaturated.index {
                        fmt::Display::fmt(&index, f)?;
                    }
                    // fmt::Display::fmt(
                    //     &index::Display::new(
                    //         unsaturated.index + 1,
                    //         isomerism::Display::new(unsaturated.isomerism, self.options.elision),
                    //         self.options.notation,
                    //     ),
                    //     f,
                    // )?;
                }
            }
        }
        Ok(())
    }
}

/// Display options
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Options {
    pub separators: Separators,
    pub notation: Notation,
    pub elision: Elision,
}

/// Separators
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Separators {
    pub c: &'static str,
    pub u: &'static str,
    pub i: [&'static str; 2],
}

mod index {
    use super::isomerism;
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Formatter};

    /// Index display
    pub(super) struct Display {
        index: usize,
        isomerism: isomerism::Display,
        notation: Notation,
    }

    impl Display {
        pub(super) fn new(index: usize, isomerism: isomerism::Display, notation: Notation) -> Self {
            Self {
                index,
                isomerism,
                notation,
            }
        }
    }

    impl fmt::Display for Display {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.notation {
                Notation::Prefix => {
                    fmt::Display::fmt(&self.isomerism, f)?;
                    fmt::Display::fmt(&self.index, f)
                }
                Notation::Suffix => {
                    fmt::Display::fmt(&self.index, f)?;
                    fmt::Display::fmt(&self.isomerism, f)
                }
            }
        }
    }

    /// Isomerism notation
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Notation {
        Prefix,
        Suffix,
    }
}

// C:D:TΔI,I,I
mod isomerism {
    use super::Isomerism;
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Formatter, Write};

    /// Display isomerism
    pub(super) struct Display {
        pub(super) isomerism: Isomerism,
        pub(super) elision: Elision,
    }

    impl Display {
        pub(super) fn new(isomerism: Isomerism, elision: Elision) -> Self {
            Self { isomerism, elision }
        }
    }

    impl fmt::Display for Display {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.isomerism {
                Isomerism::Cis => {
                    if self.elision == Elision::Explicit {
                        f.write_char('c')?;
                    }
                }
                Isomerism::Trans => {
                    f.write_char('t')?;
                }
            }
            Ok(())
        }
    }

    /// Isomerism elision
    #[derive(
        Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize,
    )]
    pub enum Elision {
        Explicit,
        #[default]
        Implicit,
    }
}

#[test]
fn test() {
    // let fatty_acid = fatty_acid!(18;9).display(COMMON);
    // assert_eq!(fatty_acid.to_string(), "18:1");
    // assert_eq!(format!("{fatty_acid:02}"), "18:01");
    // assert_eq!(format!("{fatty_acid:#}"), "18:1Δ9");
    // assert_eq!(format!("{fatty_acid:#02}"), "18:01Δ09");
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[test]
//     // fn isomerism() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15t",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;-9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;-12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15t",
//     //     );
//     //     // 1:2
//     // }

//     // #[test]
//     // fn order() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     // 1:2
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     // }

//     // #[test]
//     // fn macros() {
//     //     // 0
//     //     assert_eq!(fatty_acid!(18), new(vec![0; 17]));
//     //     // 1
//     //     assert_eq!(
//     //         fatty_acid!(18;9),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 2
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     // }

//     mod errors {
//         use super::*;

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 > 0")]
//         fn zero_carbons() {
//             fatty_acid!(0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 != 0")]
//         fn zero_index() {
//             fatty_acid!(18;0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 18 < 18")]
//         fn equal_carbons() {
//             fatty_acid!(18;18);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 19 < 18")]
//         fn greater_carbons() {
//             fatty_acid!(18;19);
//         }
//     }

//     #[test]
//     fn common() {
//         let fatty_acid = fatty_acid!(18).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:0");
//         assert_eq!(format!("{fatty_acid:02}"), "18:00");
//         assert_eq!(format!("{fatty_acid:#}"), "18:0");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:00");
//         let fatty_acid = &fatty_acid!(18;9).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:1");
//         assert_eq!(format!("{fatty_acid:02}"), "18:01");
//         assert_eq!(format!("{fatty_acid:#}"), "18:1Δ9");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:01Δ09");
//         let fatty_acid = fatty_acid!(18;9,12).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:2");
//         assert_eq!(format!("{fatty_acid:02}"), "18:02");
//         assert_eq!(format!("{fatty_acid:#}"), "18:2Δ9,12");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:02Δ09,12");
//         // Triple
//         let fatty_acid = fatty_acid!(18;9;12).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:1:1");
//         assert_eq!(format!("{fatty_acid:02}"), "18:01:01");
//         assert_eq!(format!("{fatty_acid:#}"), "18:1:1Δ9,12");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:01:01Δ09,12");
//         // Isomerism
//         let fatty_acid = fatty_acid!(18;-9,-12,-15).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:3");
//         assert_eq!(format!("{fatty_acid:02}"), "18:03");
//         assert_eq!(format!("{fatty_acid:#}"), "18:3Δ9t,12t,15t");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:03Δ09t,12t,15t");
//     }

//     #[test]
//     fn id() {
//         let fatty_acid = fatty_acid!(18).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u0");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u00");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u0");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u00");
//         let fatty_acid = fatty_acid!(18;9).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u1");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u01");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u1c9");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u01c09");
//         let fatty_acid = fatty_acid!(18;9,12).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u2");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u02");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u2c9c12");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u02c09c12");
//         // Triple
//         let fatty_acid = fatty_acid!(18;9;12).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u1u1");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u01u01");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u1u1c9c12");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u01u01c09c12");
//         // Isomerism
//         let fatty_acid = fatty_acid!(18;-9,-12,-15).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u3");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u03");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u3t9t12t15");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u03t09t12t15");
//     }
// }
