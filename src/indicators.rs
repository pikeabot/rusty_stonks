use chrono::NaiveDate;
use postgres::{Client, Error, NoTls};
use crate::mean_reversion::postgres_utils::StockData;

pub fn rsi(days: f32, stock_data: &[StockData]) -> Vec<(NaiveDate, f32)> {
    let n= days as usize;
    // https://www.investopedia.com/terms/r/rsi.asp
    let mut init_avg_gain: Vec<f32> = Vec::new();
    let mut init_avg_loss: Vec<f32> = Vec::new();
    for i in n-1..stock_data.len() {
        let mut upward: f32 = 0.0;
        let mut downward: f32 = 0.0;
        for j in i+2-n..i {
            if stock_data[j].close > stock_data[j-1].close {
                upward = upward + stock_data[j].close;
            } else if stock_data[j].close < stock_data[j-1].close {
                downward = downward + stock_data[j].close;
            }
        }
        let daily_init_avg_gain = upward/days;
        let daily_init_avg_loss = downward/days;
        init_avg_gain.push(daily_init_avg_gain);
        init_avg_loss.push(daily_init_avg_loss);
    }
    let mut rsi: Vec<(NaiveDate, f32)> = Vec::new();
    for i in 1..init_avg_gain.len() {
        let avg_gain: f32 = (init_avg_gain[i-1] * 13.0 + init_avg_gain[i])/days;
        let avg_loss:f32  = (init_avg_loss[i-1] * 13.0 + init_avg_loss[i])/days;
        let rsi_0: f32 = 100.0 - (100.0/(1.0+avg_gain/avg_loss));
        let id = (stock_data[i+n-1].date, rsi_0 );
        rsi.push(id);
    }
    rsi

}

pub fn sma(days: f32, stock_data: &[StockData]) -> Vec<(NaiveDate, f32)>{
    //https://www.investopedia.com/terms/s/sma.asp#toc-what-is-a-simple-moving-average-sma
    let mut sma: Vec<(NaiveDate, f32)> = Vec::new();
    let n = days as usize;
    for i in n-1..stock_data.len() {
        let mut total = 0.0;
        for j in i+1-n..i {
            total = total + stock_data[j].close;
        }
        sma.push(( stock_data[i].date.clone(), total/days ));
    }
    sma
}

pub fn ema(days: f32, stock_data: &[StockData]) -> Vec<(NaiveDate, f32)>{
    // https://www.investopedia.com/terms/e/ema.asp#toc-calculating-the-ema
    let smoothing = 2.0;
    let m = smoothing/(1.0 + days);
    let n = days as usize;
    let mut ema: Vec<(NaiveDate, f32)> = Vec::new();
    ema.push((stock_data[0].date, stock_data[0].close * m));
    for i in 1..stock_data.len() {
        ema.push((stock_data[i].date, stock_data[i].close * m + ema[i-1].1 * (1.0 - m)));
    }
    ema
}

pub fn bollinger_bands() {}

pub fn on_balance_volume(stock_data: &[StockData]) -> Vec<(NaiveDate, i64)> {
    let mut obv: Vec<(NaiveDate, i64)> = Vec::new();
    obv.push((stock_data[0].date, stock_data[0].volume as i64));
    for i in 1..stock_data.len() {
        let mut vol: i64 = 0;
        if stock_data[i].close > stock_data[i-1].close {
            vol = obv[i-1].1 + stock_data[i].volume as i64;
        }   else if stock_data[i].close < stock_data[i-1].close  {
            vol = obv[i-1].1 - stock_data[i].volume as i64;
        }
        obv.push((stock_data[i].date, vol));
    }
    obv
}

pub fn standard_deviation() {}


