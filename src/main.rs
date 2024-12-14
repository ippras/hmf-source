//! HMF (Human Milk Fat)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use hmf::App;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    std::env::set_var("POLARS_FMT_MAX_COLS", "256");
    // std::env::set_var("POLARS_FMT_MAX_ROWS", "32");
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "256");
    std::env::set_var("POLARS_FMT_STR_LEN", "256");

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    eframe::run_native(
        "HMF",
        Default::default(),
        Box::new(|context| Ok(Box::new(App::new(context)))),
    )
}

// When compiling to web using trunk
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = Default::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use polars::prelude::*;
    use ron::{extensions::Extensions, ser::PrettyConfig};
    use std::{fs::write, iter::empty, path::Path};

    #[test]
    fn test() -> Result<()> {
        // let foo = StructChunked::(
        //     "foo",
        //     &[
        //         Series::new("f1", ["a", "c", "e"]),
        //         Series::new("f2", ["b", "d", "f"]),
        //     ],
        // )
        // .unwrap();
        let data_frame = df! {
            "FattyAcid" => df! {
                "Carbons" => &[
                    10u8,
                    12,
                    14,
                    15,
                    15,
                    16,
                    16,
                    16,
                    17,
                    17,
                    18,
                    18,
                    18,
                    18,
                    18,
                    20,
                    20,
                    21,
                    20,
                    20,
                    20,
                    20,
                    20,
                    22,
                    22,
                    22,
                    23,
                    22,
                    22,
                    22,
                    22,
                    24,
                    24,
                ],
                "Unsaturated" => &[
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([None::<u8>]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([9u8]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([None::<u8>, None::<u8>]),
                        "Isomerism"    => Series::from_iter([1i8, 1i8]),
                        "Unsaturation" => Series::from_iter([1u8, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([None::<u8>]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([9u8]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([9, 12u8]),
                        "Isomerism"    => Series::from_iter([1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([6, 9, 12u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([9, 12, 15u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([11u8]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([11, 14u8]),
                        "Isomerism"    => Series::from_iter([1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([8, 11, 14u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([5, 8, 11, 14u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([11, 14, 17u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([5, 8, 11, 14, 17u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([13u8]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([13, 16u8]),
                        "Isomerism"    => Series::from_iter([1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([7, 10, 13, 16u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([7, 10, 13, 16, 19u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([4, 7, 10, 13, 16u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([4, 7, 10, 13, 16, 19u8]),
                        "Isomerism"    => Series::from_iter([1, 1, 1, 1, 1, 1i8]),
                        "Unsaturation" => Series::from_iter([1, 1, 1, 1, 1, 1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter(empty::<u8>()),
                        "Isomerism"    => Series::from_iter(empty::<i8>()),
                        "Unsaturation" => Series::from_iter(empty::<u8>()),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index"        => Series::from_iter([None::<u8>]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            "Median13" => [
                Some(0.91f64),
                Some(3.61f64),
                Some(3.50f64),
                Some(0.08f64),
                Some(0.00f64),
                Some(20.22f64),
                Some(2.44f64),
                Some(0.00f64),
                Some(0.19f64),
                Some(0.11f64),
                Some(5.29f64),
                Some(36.96f64),
                Some(20.85f64),
                Some(0.07f64),
                Some(0.83f64),
                Some(0.21f64),
                Some(0.53f64),
                None,
                Some(0.48f64),
                Some(0.38f64),
                Some(0.54f64),
                Some(0.00f64),
                Some(0.17f64),
                Some(0.00f64),
                Some(0.10f64),
                Some(0.07f64),
                Some(0.10f64),
                Some(0.14f64),
                Some(0.12f64),
                Some(0.23f64),
                Some(0.44f64),
                Some(0.10f64),
                Some(0.00f64),
            ],
            "InterquartileRange13" => [
                Some(0.38),
                Some(2.16),
                Some(2.32),
                Some(0.11),
                Some(0.06),
                Some(3.29),
                Some(1.44),
                Some(0.08),
                Some(0.07),
                Some(0.07),
                Some(1.55),
                Some(3.31),
                Some(4.60),
                Some(0.18),
                Some(0.67),
                Some(0.17),
                Some(0.10),
                None,
                Some(0.18),
                Some(0.16),
                Some(0.19),
                Some(0.05),
                Some(0.19),
                Some(0.04),
                Some(0.18),
                Some(0.13),
                Some(0.23),
                Some(0.12),
                Some(0.17),
                Some(0.21),
                Some(0.58),
                Some(0.46),
                Some(0.19),
            ],
            "ReferenceRangeMin13" => [
                Some(0.52),
                Some(1.23),
                Some(1.20),
                Some(0.00),
                Some(0.00),
                Some(17.02),
                Some(1.12),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(3.89),
                Some(28.50),
                Some(16.58),
                Some(0.00),
                Some(0.46),
                Some(0.12),
                Some(0.38),
                None,
                Some(0.05),
                Some(0.18),
                Some(0.37),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
            ],
            "ReferenceRangeMax13" => [
                Some(1.64),
                Some(6.41),
                Some(5.29),
                Some(0.18),
                Some(0.14),
                Some(24.39),
                Some(3.47),
                Some(0.21),
                Some(0.28),
                Some(0.19),
                Some(7.37),
                Some(42.37),
                Some(27.29),
                Some(0.20),
                Some(2.04),
                Some(0.53),
                Some(0.95),
                None,
                Some(1.89),
                Some(0.63),
                Some(1.77),
                Some(0.85),
                Some(0.87),
                Some(0.35),
                Some(1.80),
                Some(1.14),
                Some(1.39),
                Some(2.74),
                Some(0.51),
                Some(1.15),
                Some(3.69),
                Some(1.68),
                Some(0.41),
            ],
            "Median2" => [
                Some(0.58),
                Some(4.32),
                Some(5.55),
                Some(0.17),
                None,
                Some(49.06),
                Some(3.22),
                None,
                Some(0.28),
                Some(0.06),
                Some(1.60),
                Some(14.91),
                Some(12.95),
                Some(0.00),
                Some(0.58),
                Some(0.24),
                Some(0.40),
                Some(0.16),
                Some(0.25),
                Some(0.26),
                Some(0.54),
                None,
                Some(0.00),
                None,
                Some(0.00),
                Some(0.32),
                Some(0.00),
                Some(0.32),
                Some(0.27),
                Some(0.31),
                Some(1.06),
                Some(0.36),
                Some(0.00),
            ],
            "InterquartileRange2" => [
                Some(0.29),
                Some(2.28),
                Some(1.65),
                Some(0.14),
                None,
                Some(5.49),
                Some(1.18),
                None,
                Some(0.14),
                Some(0.19),
                Some(0.52),
                Some(1.97),
                Some(2.83),
                Some(0.16),
                Some(0.27),
                Some(0.15),
                Some(0.10),
                Some(0.28),
                Some(0.21),
                Some(0.08),
                Some(0.23),
                None,
                Some(0.24),
                None,
                Some(0.16),
                Some(0.66),
                Some(0.29),
                Some(0.25),
                Some(0.38),
                Some(0.46),
                Some(0.71),
                Some(0.73),
                Some(0.33),
            ],
            "ReferenceRangeMin2" => [
                Some(0.00),
                Some(1.42),
                Some(2.06),
                Some(0.00),
                None,
                Some(41.79),
                Some(1.23),
                None,
                Some(0.00),
                Some(0.00),
                Some(0.55),
                Some(11.32),
                Some(9.66),
                Some(0.00),
                Some(0.28),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.12),
                Some(0.19),
                None,
                Some(0.00),
                None,
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
                Some(0.00),
            ],
            "ReferenceRangeMax2" => [
                Some(0.99),
                Some(7.48),
                Some(10.12),
                Some(1.10),
                None,
                Some(58.84),
                Some(5.23),
                None,
                Some(3.84),
                Some(0.22),
                Some(2.68),
                Some(21.35),
                Some(17.76),
                Some(0.31),
                Some(1.78),
                Some(0.51),
                Some(1.56),
                Some(0.64),
                Some(0.74),
                Some(0.50),
                Some(0.81),
                None,
                Some(0.38),
                None,
                Some(0.28),
                Some(2.66),
                Some(0.34),
                Some(0.58),
                Some(0.67),
                Some(0.84),
                Some(2.44),
                Some(1.73),
                Some(1.37),
            ]

        }?;
        let data_frame = data_frame
            .lazy()
            .select([
                col("FattyAcid"),
                as_struct(vec![
                    col("Median13").alias("Median"),
                    col("InterquartileRange13").alias("InterquartileRange"),
                    as_struct(vec![
                        col("ReferenceRangeMin13").alias("Min"),
                        col("ReferenceRangeMax13").alias("Max"),
                    ])
                    .alias("ReferenceRange"),
                ])
                .alias("StereospecificNumber123"),
                as_struct(vec![
                    col("Median2").alias("Median"),
                    col("InterquartileRange2").alias("InterquartileRange"),
                    as_struct(vec![
                        col("ReferenceRangeMin2").alias("Min"),
                        col("ReferenceRangeMax2").alias("Max"),
                    ])
                    .alias("ReferenceRange"),
                ])
                .alias("StereospecificNumber2"),
            ])
            .collect()?;
        let contents = ron::ser::to_string_pretty(
            &data_frame,
            PrettyConfig::default().extensions(Extensions::IMPLICIT_SOME),
        )?;
        write("df.ron", contents)?;
        Ok(())
    }
}
