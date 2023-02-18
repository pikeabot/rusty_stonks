use chrono::NaiveDate;
use plotters::prelude::*;
use plotters::prelude::full_palette::ORANGE;
use crate::postgres_utils::{get_stock_data, StockData};
use crate::indicators::*;


pub fn ema_crossing() -> Result<(), Box<dyn std::error::Error>> {
    let result_data = get_stock_data("spy");
    let data_raw = result_data.unwrap();
    let ema20_raw = ema(20.0, &data_raw);
    let ema50 = ema(50.0, &data_raw);
    let initial_date = ema50[0].0;
    let portfolio_percentage = 0.1;

    // find start date of data so they are all the same
    let mut data: &[StockData] = &[];
    for i in 0..data_raw.len() {
        if data_raw[i].date == initial_date {
            data = &data_raw[i..];
            break;
        }
    }

    let mut ema20: &[(NaiveDate, f32)] = &[];
    for i in 0..ema20_raw.len() {
        if ema20_raw[i].0 == initial_date {
            ema20 = &ema20_raw[i..];
            break;
        }
    }

    let mut balance = 10000.0;
    let mut num_of_shares = 0.0;

    for i in 0..data.len() {
        if num_of_shares == 0.0 {
            // buy
            if ema20[i].1 > ema50[i].1 {
                let total_bet = balance * portfolio_percentage;
                num_of_shares = (total_bet/data[i].close).floor();
                let cost = (num_of_shares * data[i].close);
                balance = balance - cost;
                println!("Buying {} shares for {} for total cost of {}", num_of_shares, data[i].close, cost);
                println!("{}: Balance of {}", data[i].date, balance);
            }
        } else {
            // sell
            if ema20[i].1 < ema50[i].1 {
                let profit = (num_of_shares * data[i].close);
                balance = balance + profit;
                num_of_shares = 0.0;
                println!("Sold for {} for a total of {}", data[i].close, profit);
                println!("{}: Balance of {}", data[i].date, balance);
            }
        }
    }
    println!("Total profit: {}", balance);
    draw_chart(&data_raw, ema20_raw, ema50).expect("TODO: panic message");
    Ok(())
}


fn draw_chart(stock_data: &[StockData], ema20: Vec<(NaiveDate, f32)>, ema50: Vec<(NaiveDate, f32)>) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut t_ema20 = 1.0;
    chart.draw_series(LineSeries::new(
        ema20.into_iter().map(|x | {t_ema20+=1.0; (t_ema20, x.1) } ),
        &GREEN,
    ))?;

    let mut t_ema50 = 50.0;
    chart.draw_series(LineSeries::new(
        ema50.into_iter().map(|x | {t_ema50+=1.0; (t_ema50, x.1) } ),
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