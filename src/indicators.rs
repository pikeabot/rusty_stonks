use postgres::{Client, Error, NoTls};
use crate::mean_reversion::postgres_utils::StockData;

pub fn rsi(stock_data: &[StockData]) -> Vec<f32> {
    let n = 14.0;

    // for i in 0..10 {
    //     println!("{}", stock_data[i].close);
    // }

    // https://www.investopedia.com/terms/r/rsi.asp
    let mut init_avg_gain: Vec<f32> = Vec::new();
    let mut init_avg_loss: Vec<f32> = Vec::new();
    for i in 14..stock_data.len() {
        let mut upward: f32 = 0.0;
        let mut downward: f32 = 0.0;
        for j in i-13..i {
            if stock_data[j].close > stock_data[j-1].close {
                upward = upward + stock_data[j].close;
            } else if stock_data[j].close < stock_data[j-1].close {
                downward = downward + stock_data[j].close;
            }
        }
        let daily_init_avg_gain = upward/n;
        let daily_init_avg_loss = downward/n;
        init_avg_gain.push(daily_init_avg_gain);
        init_avg_loss.push(daily_init_avg_loss);
    }
    let mut avg_gain: Vec<f32> = Vec::new();
    let mut avg_loss: Vec<f32> = Vec::new();
    for i in 1..init_avg_gain.len() {
        let ag: f32 = (init_avg_gain[i-1] * 13.0 + init_avg_gain[i])/n;
        let al:f32  = (init_avg_loss[i-1] * 13.0 + init_avg_loss[i])/n;
        avg_gain.push(ag);
        avg_loss.push(al);
    }

    // for i in 0..10 {
    //     println!("{}", avg_gain[i]);
    // }

    let mut rsi: Vec<f32> = Vec::new();
    for i in 0..avg_gain.len() {
        let rsi_0: f32 = 100.0 - (100.0/(1.0+avg_gain[i]/avg_loss[i]));
        rsi.push(rsi_0);
    }

    rsi
    // for i in 0..30 {
    //     println!("{}", rsi[i]);
    // }
}

pub fn ema(days: f32, stock_data: &[StockData]) -> Vec<f32>{
    let smoothing = 2.0;
    let m = smoothing/(1.0 + days);

    let mut ema: Vec<f32> = Vec::new();
    ema.push(stock_data[0].close * m);
    for i in 1..stock_data.len() {
        ema.push(stock_data[i].close * m + ema[i-1] * (1.0 - m));
    }
    ema
}

pub fn bollinger_bands() {}

