/*
                Table "public.spy"
 Column |  Type   | Collation | Nullable | Default
--------+---------+-----------+----------+---------
 date   | date    |           | not null |
 close  | real   |           | not null |
 volume | integer |           | not null |
 open   | real   |           | not null |
 high   | real   |           | not null |
 low    | real   |           | not null |
Indexes:
    "SPY_pkey" PRIMARY KEY, btree (date)
 */

use std::os::unix::fs::chroot;
// use actix_web::cookie::Expiration::DateTime;
use chrono::NaiveDate;
use postgres::{Client, Error, NoTls};


pub struct StockData {
    pub date: NaiveDate,
    pub close: f32,
    pub volume: i32,
    pub open: f32,
    pub high: f32,
    pub low: f32
}

pub fn get_stock_data(ticker: &str)  -> Result<Vec<StockData>, Error>{
    let mut client = Client::connect(
        "postgresql://postgres:bonitis@localhost/stonks",
        NoTls,
    )?;

    let mut row_vector: Vec<StockData> = Vec::new();
    // let sql_query:&str = &format!("SELECT * FROM public.{} ORDER BY date DESC LIMIT 5", ticker.to_lowercase());
    let sql_query:&str = &format!("SELECT * FROM public.{}", ticker.to_lowercase());
    for row in client.query(sql_query, &[])? {
        let dt = chrono::NaiveDate::parse_from_str(row.get(0), "%m/%d/%Y").unwrap();
        let stockdata = StockData {
            date: dt,
            close: row.get(1),
            volume: row.get(2),
            open: row.get(3),
            high: row.get(4),
            low: row.get(5)
        };
        row_vector.push(stockdata);
    }
   row_vector.sort_by_key(|x| x.date);

    Ok(row_vector)
}

pub fn get_stock_closes(stock_data: &[StockData]) -> Vec<f32> {
    let mut closing_prices: Vec<f32> = Vec::new();
    for daily_data in stock_data {
        closing_prices.push(daily_data.close);
    }
    closing_prices
}