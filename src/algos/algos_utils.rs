use chrono::NaiveDate;
use crate::postgres_utils::StockData;

pub fn buy_shares(balance: f32, portfolio_percentage: f32, price: f32, date: NaiveDate) -> (f32, f32) {
    let total_bet = balance * portfolio_percentage;
    let num_of_shares = (total_bet / price).floor();
    let cost = num_of_shares * price;
    let remainder = balance - cost;
    println!("Buying {} shares for {} for total cost of {}", num_of_shares, price, cost);
    println!("{}: Balance of {}", date, balance);
    (num_of_shares, remainder)
}

pub fn sell_shares(balance: f32, num_of_shares: f32, price: f32, date: NaiveDate) -> (f32, f32) {
    let profit = num_of_shares * price;
    let new_balance = balance + profit;
    let num_of_shares = 0.0;
    println!("Sold for {} for a total of {}", price, profit);
    println!("{}: Balance of {}", date, new_balance);
    (num_of_shares, new_balance)
}

pub fn display_final_results(balance: f32, num_of_shares: f32, price: f32, date: NaiveDate){
    let profit = num_of_shares * price;
    let final_balance = balance + profit;
    println!("Ending date: {}", date);
    if num_of_shares != 0.0 {
        println!("Ending price: {}", price);
    }
    println!("Total profit: {}", final_balance);
}

pub fn print_stock_data(stock_data: &Vec<(NaiveDate, f32)>) {
    for i in 0..stock_data.len() {
        println!("{:?}", stock_data[i]);
    }
}
