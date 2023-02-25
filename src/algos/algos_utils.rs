use chrono::NaiveDate;
use std::fs::File;
use std::io::prelude::*;
use crate::postgres_utils::StockData;

/*
    let result_data = get_stock_data("spy");
    let data = result_data.unwrap();
    let rsi = rsi(14.0,&data);
    let ema = ema(20.0, &data);
    let sma = sma(20.0, &data);
    let obv = on_balance_volume(&data);
    let sd = standard_deviation(20.0, &data);

    let (bbu, bbl) = bollinger_bands(&data);
    let mut support:Vec<(NaiveDate, f32)> = Vec::new();
    let mut resistance:Vec<(NaiveDate, f32)> = Vec::new();
    (support, resistance) = get_support_resistance(5, &data);

 */

pub fn buy_shares(balance: f32, portfolio_percentage: f32, price: f32, date: NaiveDate, file: &mut File) -> (f32, f32) {
    let total_bet = balance * portfolio_percentage;
    let num_of_shares = (total_bet / price).floor();
    let cost = num_of_shares * price;
    let remainder = balance - cost;
    println!("Buying {} shares for {} for total cost of {}", num_of_shares, price, cost);
    println!("{}: Balance of {}", date, balance);
    writeln!(file, "Buying {} shares for {} for total cost of {}", num_of_shares, price, cost);
    writeln!(file, "{}: Balance of {}", date, balance);
    (num_of_shares, remainder)
}

pub fn sell_shares(balance: f32, num_of_shares: f32, price: f32, date: NaiveDate, file: &mut File) -> (f32, f32) {
    let profit = num_of_shares * price;
    let new_balance = balance + profit;
    let num_of_shares = 0.0;
    println!("Sold for {} for a total of {}", price, profit);
    println!("{}: Balance of {}", date, new_balance);
    writeln!(file, "Sold for {} for a total of {}", price, profit);
    writeln!(file, "{}: Balance of {}", date, new_balance);
    (num_of_shares, new_balance)
}

pub fn short_shares(balance: f32, portfolio_percentage: f32, price: f32, date: NaiveDate, file: &mut File) -> (f32, f32) {
    let total_bet = balance * portfolio_percentage;
    let num_of_shares = (total_bet / price).floor();
    let cost = num_of_shares * price;
    let remainder = balance + cost;
    println!("Shorting {} shares for {} for total cost of {}", num_of_shares, price, cost);
    println!("{}: Balance of {}", date, remainder);
    writeln!(file, "Shorting {} shares for {} for total cost of {}", num_of_shares, price, cost);
    writeln!(file, "{}: Balance of {}", date, remainder);
    (num_of_shares, remainder)
}

pub fn cover_shares(balance: f32, num_of_shares: f32, price: f32, date: NaiveDate, file: &mut File) -> (f32, f32) {
    let profit = num_of_shares * price;
    let new_balance = balance - profit;
    let num_of_shares = 0.0;
    println!("Covered for {} for a total of {}", price, profit);
    println!("{}: Balance of {}", date, new_balance);
    writeln!(file, "Covered for {} for a total of {}", price, profit);
    writeln!(file, "{}: Balance of {}", date, new_balance);
    (num_of_shares, new_balance)
}

pub fn display_final_results(balance: f32, num_of_shares: f32, price: f32, date: NaiveDate, file: &mut File){
    let profit = num_of_shares * price;
    let final_balance = balance + profit;
    println!("Ending date: {}", date);
    writeln!(file, "Ending date: {}", date);
    if num_of_shares != 0.0 {
        println!("Ending price: {}", price);
        writeln!(file, "Ending price: {}", price);
    }
    println!("Total profit: {}", final_balance);
    writeln!(file, "Total profit: {}", final_balance);
}

pub fn print_stock_data(stock_data: &Vec<(NaiveDate, f32)>) {
    for i in 0..stock_data.len() {
        println!("{:?}", stock_data[i]);
    }
}
