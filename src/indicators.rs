use chrono::NaiveDate;
use ta::indicators::*;
use ta::{Next, DataItem};
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

pub fn get_keltner_channel_vec(window: usize, multiplier: f64, stock_data: &[StockData]) -> Vec<(NaiveDate, KeltnerChannelOutput)> {
    let mut kc_obj = KeltnerChannel::new(window, 2.0_f64).unwrap();
    let mut kc_vec: Vec<(NaiveDate, KeltnerChannelOutput)> = Vec::new();
    for i in 0..stock_data.len() {
        kc_vec.push((stock_data[i].date, kc_obj.next(stock_data[i].close as f64)));
    }
    kc_vec
}


pub fn get_on_balance_volume_vec(stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut obv_obj = OnBalanceVolume::new();
    let mut obv_vec: Vec<(NaiveDate, f64)> = Vec::new();
    for i in 0..stock_data.len() {
        let data_item = DataItem::builder()
            .high(stock_data[i].high as f64)
            .low(stock_data[i].low as f64)
            .close(stock_data[i].close as f64)
            .open(stock_data[i].open as f64)
            .volume(stock_data[i].volume as f64)
            .build().unwrap();
        obv_vec.push((stock_data[i].date, obv_obj.next(&data_item)));
    }
    obv_vec
}

pub fn get_on_average_true_range_vec(window: usize, stock_data: &[StockData]) -> Vec<(NaiveDate, f64)> {
    let mut atr_obj = AverageTrueRange::new(window).unwrap();
    let mut atr_vec: Vec<(NaiveDate, f64)> = Vec::new();
    for i in 0..stock_data.len() {
        let data_item = DataItem::builder()
            .high(stock_data[i].high as f64)
            .low(stock_data[i].low as f64)
            .close(stock_data[i].close as f64)
            .open(stock_data[i].open as f64)
            .volume(stock_data[i].volume as f64)
            .build().unwrap();
        atr_vec.push((stock_data[i].date, atr_obj.next(&data_item)));
    }
    atr_vec
}


fn calculate_sd(data_set:&Vec<f32>) -> f32 {
    // calculate the standard deviation of a given set of data
    let n = data_set.len() as f32;
    let mean_total:f32 = data_set.iter().sum();
    let mean = mean_total/n;
    let sd_total:f32 = data_set.iter().map(|x| (x-mean).powf(2.0)).sum();
    (sd_total/(n-1.0)).sqrt()
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
