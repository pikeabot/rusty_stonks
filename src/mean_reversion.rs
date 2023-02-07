#[path = "postgres_utils.rs"] mod postgres_utils;
#[path = "indicators.rs"] mod indicators;
use plotters::prelude::*;

pub fn mean_reversion() -> Result<(), Box<dyn std::error::Error>> {
    let data = postgres_utils::get_stock_data("spy");
    let rows = postgres_utils::convert_to_stock_data_struct(data);
    let rsi = indicators::rsi(&rows);

    let closing_prices = postgres_utils::get_stock_closes(&rows);
    draw_mean_reversion_chart(closing_prices, rsi).expect("TODO: panic message");
    Ok(())
}

fn draw_mean_reversion_chart(closing_prices: Vec<f32>, rsi: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    // Draw chart
    let root = BitMapBackend::new("mean_reversion.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Mean Reversion", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..1000f32, 0f32..500f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;
    let mut t_rsi = 14.0;
    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        rsi.into_iter().map(|x | {t_rsi+=1.0; (t_rsi, x) } ),
        &RED,
    ))?;

    let mut t_close = 0.0;
    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        closing_prices.into_iter().map(|x | {t_close+=1.0; (t_close, x) } ),
        &BLUE,
    ))?;
    root.present()?;
    Ok(())
}