    fn main() {
  //  let wanted_percentage_bitcoin = 50;
   // let wanted_percentage_usd = 50;
    let amount_satoshis = 200000000;
    let value_usd = 10000.00;
    //let percentage_rebalance_threshold = 5.0;
    get_value_of_btc_in_usd(amount_satoshis);
    let number_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
    println!("Number of BTC: {}", number_of_bitcoin);
    println!("Price of BTC in USD: {}", get_price());
    println!("Value of USD: {}", value_usd);
    println!("Value of BTC: {} USD",get_value_of_btc_in_usd(amount_satoshis));
    let value_of_btc_in_usd = get_value_of_btc_in_usd(amount_satoshis);
    println!("Current percentage BTC: {}%", current_percentage_btc(value_of_btc_in_usd, value_usd));
    println!("Current percentage USD: {}%", current_percentage_usd(value_of_btc_in_usd, value_usd));
}

fn get_value_of_btc_in_usd(amount_satoshis: i32) -> f64
{
     let price_btc = get_price();
     let amount_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
     price_btc * amount_of_bitcoin
}

fn get_price() -> f64
{   let price_btc = 6000.00;
    price_btc
}

fn get_satoshis_as_bitcoin(amount_satoshis: i32) -> f64
{
    let conversion_rate = 100000000;
   (amount_satoshis as f64 / conversion_rate as f64)
}

fn current_percentage_btc(value_of_btc_in_usd: f64, value_usd: f64) -> f64
{
    let total = value_of_btc_in_usd + value_usd;
    let percentage_btc = value_of_btc_in_usd * 100.00 / total;
    percentage_btc.round()
}

fn current_percentage_usd(value_of_btc_in_usd: f64, value_usd: f64) -> f64{
 let total = value_of_btc_in_usd + value_usd;
    let percentage_usd = value_usd * 100.00 / total;
    percentage_usd.round()
}

fn calculate_threshold_up_price(number_of_bitcoin: f64, value_usd: f64, percentage_rebalance_threshold: f64) -> f64{
   //returns what price is needed to reach threshold_up
   //reached if gap between (amountSatoshis*priceBTC) and amoutUSD > percentageRebalanceThreshold
    


    50.0
}

// fn calculate_threshold_down_price() -> f64 {
//   //returns what price is needed to reach threshold_up and threshold_down
//   //reached if gap between (amountSatoshis*priceBTC) and amoutUSD > percentageRebalanceThreshold
   
//   50.0
// }
