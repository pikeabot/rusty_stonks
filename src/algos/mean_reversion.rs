#[path = "postgres_utils.rs"] mod postgres_utils;
#[path = "indicators.rs"] mod indicators;
use chrono::NaiveDate;
use plotters::prelude::*;
use plotters::prelude::full_palette::ORANGE;
use postgres_utils::StockData;

pub fn mean_reversion() -> Result<(), Box<dyn std::error::Error>> {
    let result_data = postgres_utils::get_stock_data("spy");
    let data = result_data.unwrap();
    let rsi = indicators::rsi(14.0,&data);
    let ema = indicators::ema(20.0, &data);
    let sma = indicators::sma(20.0, &data);
    let obv = indicators::on_balance_volume(&data);
    let sd = indicators::standard_deviation(20.0, &data);
    draw_mean_reversion_chart(&data, rsi, ema, sma).expect("TODO: panic message");
    Ok(())
}

fn draw_mean_reversion_chart(stock_data: &[StockData], rsi: Vec<(NaiveDate, f32)>, ema: Vec<(NaiveDate, f32)>, sma: Vec<(NaiveDate, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    // Draw chart
    let root = BitMapBackend::new("algos.png", (640, 480)).into_drawing_area();
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
        rsi.into_iter().map(|x | {t_rsi+=1.0; (t_rsi, x.1) } ),
        &RED,
    ))?;

    let mut t_ema = 1.0;
    chart.draw_series(LineSeries::new(
        ema.into_iter().map(|x | {t_ema+=1.0; (t_ema, x.1) } ),
        &GREEN,
    ))?;

    let mut t_sma = 20.0;
    chart.draw_series(LineSeries::new(
        sma.into_iter().map(|x | {t_sma+=1.0; (t_sma, x.1) } ),
        &ORANGE,
    ))?;

    let mut t_close = 0.0;
    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        stock_data.into_iter().map(|x | {t_close+=1.0; (t_close, x.close) } ),
        &BLUE,
    ))?;
    root.present()?;
    Ok(())
}