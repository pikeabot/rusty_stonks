use chrono::NaiveDate;
use ta::indicators::*;
use ta::Next;
use crate::postgres_utils::StockData;

fn format_ta_64<T: Next<f64, Output = f64>>(mut ta_obj: T, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut ta_vec: Vec<(NaiveDate, f64)> = Vec::new();
    for i in 0..stock_data.len() {
        ta_vec.push((stock_data[i].date, ta_obj.next(stock_data[i].close as f64)));
    }
    ta_vec
}

pub fn get_rsi_vec(window: usize, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut rsi_obj = RelativeStrengthIndex::new(window).unwrap();
    let mut rsi_vec : Vec<(NaiveDate, f64)> = format_ta_64(rsi_obj, &stock_data);
    rsi_vec
}


pub fn get_sma_vec(window: usize, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut sma_obj = SimpleMovingAverage::new(window).unwrap();
    let mut sma_vec: Vec<(NaiveDate, f64)> = format_ta_64(sma_obj, &stock_data);
    sma_vec
}

pub fn get_ema_vec(window: usize, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut ema_obj = ExponentialMovingAverage::new(window).unwrap();
    let mut ema_vec: Vec<(NaiveDate, f64)> = format_ta_64(ema_obj, &stock_data);
    ema_vec
}

pub fn get_standard_deviation_vec(window: usize, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut sd_obj = StandardDeviation::new(window).unwrap();
    let mut sd_vec: Vec<(NaiveDate, f64)> = format_ta_64(sd_obj, &stock_data);
    sd_vec
}


pub fn get_bollinger_band_vec(window: usize, multiplier: f64, stock_data: &[StockData]) -> Vec<(NaiveDate, BollingerBandsOutput)> {
    let mut bb_obj = BollingerBands::new(window, multiplier).unwrap();
    let mut bb_vec: Vec<(NaiveDate, BollingerBandsOutput)> = Vec::new();
    for i in 0..stock_data.len() {
        bb_vec.push((stock_data[i].date, bb_obj.next(stock_data[i].close as f64)));
    }
    bb_vec
}

// pub fn on_balance_volume(stock_data: &[StockData]) -> Vec<(NaiveDate, i64)> {
//     //https://www.investopedia.com/terms/o/onbalancevolume.asp
//     let mut obv: Vec<(NaiveDate, i64)> = Vec::new();
//     obv.push((stock_data[0].date, stock_data[0].volume as i64));
//     for i in 1..stock_data.len() {
//         let mut vol: i64 = 0;
//         if stock_data[i].close > stock_data[i-1].close {
//             vol = obv[i-1].1 + stock_data[i].volume as i64;
//         }   else if stock_data[i].close < stock_data[i-1].close  {
//             vol = obv[i-1].1 - stock_data[i].volume as i64;
//         }
//         obv.push((stock_data[i].date, vol));
//     }
//     obv
// }

fn calculate_sd(data_set:&Vec<f32>) -> f32 {
    // calculate the standard deviation of a given set of data
    let n = data_set.len() as f32;
    let mean_total:f32 = data_set.iter().sum();
    let mean = mean_total/n;
    let sd_total:f32 = data_set.iter().map(|x| (x-mean).powf(2.0)).sum();
    (sd_total/(n-1.0)).sqrt()
}

pub fn bollinger_bands(stock_data: &[StockData]) -> (Vec<(NaiveDate, f32)>, Vec<(NaiveDate, f32)>) {
    //https://www.investopedia.com/terms/b/bollingerbands.asp
    let n = 20.0; //smoothing period
    let m = 2.0; //standard deviations

    let mut bb_upper: Vec<(NaiveDate, f32)> = Vec::new();
    let mut bb_lower: Vec<(NaiveDate, f32)> = Vec::new();
    let n_usize = m as usize;
    for i in n_usize-1..stock_data.len() {
        // let mut total = 0.0;
        // let typical_price = stock_data[i].high + (stock_data[i].low + stock_data[i].close)/3.0;
        // for j in i+1-musize..i {
        //     total += typical_price;
        // }
        let data_set = &stock_data[i+1-n_usize..i+1];
        let tp:Vec<f32> = data_set.iter().map(|x| (x.high + x.low + x.close)/3.0).collect();
        let tp_ma_sum : f32 = tp.iter().sum();
        let tp_ma = tp_ma_sum/m;
        let sigma = calculate_sd(&tp);
        let upper = tp_ma + m * sigma;
        let lower = tp_ma - m * sigma;
        bb_upper.push(( stock_data[i].date.clone(), upper));
        bb_lower.push(( stock_data[i].date.clone(), lower));
    }
    (bb_upper, bb_lower)
}

pub fn get_support_resistance(window: usize, stock_data: &[StockData]) -> (Vec<(NaiveDate, f32)> ,Vec<(NaiveDate, f32)>){
    //https://towardsdatascience.com/detection-of-price-support-and-resistance-levels-in-python-baedc44c34c9
    let mut support: Vec<(NaiveDate, f32)> = Vec::new();
    let mut resistance: Vec<(NaiveDate, f32)> = Vec::new();
    let candle_mean = calculate_candle_mean(&stock_data);
    for i in window..stock_data.len() {
        let is_support = check_support(&stock_data[i-window..i]);
        let is_resistance = check_resistance(&stock_data[i-window..i]);
        let mid = window/2 + 1;
        if is_support {
            if is_far_from_level(stock_data[i-mid].low, candle_mean, &support) {
                support.push((stock_data[i-mid].date, stock_data[i-mid].low));
            }
        }
        if is_resistance {
            if is_far_from_level(stock_data[i-mid].high, candle_mean,&resistance) {
                resistance.push((stock_data[i-mid].date, stock_data[i-mid].high));
            }
        }
    }
    (support, resistance)
}

fn check_support(fractal: &[StockData]) -> bool {
    // assumes fractal has a length of 5
    if fractal[0].low > fractal[1].low && fractal[1].low > fractal[2].low
        && fractal[2].low < fractal[3].low && fractal[3].low < fractal[4].low {
        return true
    }
    false
}

fn check_resistance(fractal: &[StockData]) -> bool {
    // assumes fractal has a length of 5
    if fractal[0].high < fractal[1].high && fractal[1].high < fractal[2].high
        && fractal[2].high > fractal[3].high && fractal[3].high > fractal[4].high {
        return true
    }
    false
}

fn calculate_candle_mean(stock_data: &[StockData]) -> f32 {
    // get the average length of a candlestick
    let mut total = 0.0;
    for s in stock_data {
        total += s.high - s.low;
    }
    total/stock_data.len() as f32
}

fn is_far_from_level(level: f32, candle_mean: f32, levels: &[(NaiveDate, f32)] ) -> bool{
    //def isFarFromLevel(l):
    //    return np.sum([abs(l-x) < s  for x in levels]) == 0
    if levels.len() == 0 {
        return true
    }
    for x in levels {
        if (level-x.1).abs() > candle_mean {
            return true
        }
    }
    false
}
