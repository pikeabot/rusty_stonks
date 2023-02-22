use chrono::NaiveDate;
use crate::postgres_utils::StockData;

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
        let data_set = &stock_data[i+1-n..i-1];
        let total:f32 = data_set.iter().map(|x| x.close).sum();
        sma.push(( stock_data[i].date.clone(), total/days ));
    }
    sma
}

pub fn ema(days: f32, stock_data: &[StockData]) -> Vec<(NaiveDate, f32)>{
    // https://www.investopedia.com/terms/e/ema.asp#toc-calculating-the-ema
    let smoothing = 2.0;
    let m = smoothing/(1.0 + days);
    let mut ema: Vec<(NaiveDate, f32)> = Vec::new();
    ema.push((stock_data[0].date, stock_data[0].close * m));
    for i in 1..stock_data.len() {
        ema.push((stock_data[i].date, stock_data[i].close * m + ema[i-1].1 * (1.0 - m)));
    }
    ema
}

pub fn on_balance_volume(stock_data: &[StockData]) -> Vec<(NaiveDate, i64)> {
    //https://www.investopedia.com/terms/o/onbalancevolume.asp
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

pub fn standard_deviation(days: f32, stock_data: &[StockData]) -> Vec<(NaiveDate, f32)>{
    /*
    Calculate the standard deviation of stock data using closing price and a given number
    of days.
    Returns a vector of standard deviations
     */
    //https://www.investopedia.com/terms/s/standarddeviation.asp
    let mut sd_vec: Vec<(NaiveDate, f32)> = Vec::new();
    let n = days as usize;
    for i in n-1..stock_data.len() {
        let data_set = &stock_data[i+1-n..i-1].iter().map(|x| x.close).collect();
        let sd = calculate_sd(data_set);
        sd_vec.push((stock_data[i].date.clone(), sd));
    }
    sd_vec
}

fn calculate_sd(data_set:&Vec<f32>) -> f32 {
    // calculate the standard deviation of a given set of data
    let n = data_set.len() as f32;
    let mean_total:f32 = data_set.iter().sum();
    let mean = mean_total/n;
    let sd_total:f32 = data_set.iter().map(|x| x-mean.powf(2.0)).sum();
    sd_total/(n-1.0).sqrt()
}

pub fn bollinger_bands(stock_data: &[StockData]) -> Vec<(NaiveDate, f32, f32)> {
    //https://www.investopedia.com/terms/b/bollingerbands.asp
    let m = 20.0; //smoothing period
    let n = 2.0; //standard deviations

    let mut bollinger_bands: Vec<(NaiveDate, f32, f32)> = Vec::new();
    let t = m as usize;
    for i in t-1..stock_data.len() {
        let mut total = 0.0;
        let typical_price = stock_data[i].high + (stock_data[i].low + stock_data[i].close)/3.0;
        for j in i+1-t..i {
            total += typical_price;
        }
        let tp_ma = total/m;
        // TODO: fix this
        let sigma = standard_deviation(m, stock_data)[0];
        let upper = tp_ma + m * sigma.1;
        let lower = tp_ma - m * sigma.1;
        bollinger_bands.push(( stock_data[i].date.clone(), upper, lower));
    }
    bollinger_bands
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

