    fn main() {
    let wanted_percentage_bitcoin = 50;
    let wanted_percentage_usd = 50;
    let amount_satoshis = 290000000;
    let amount_usd = 10000.00;
    let percentage_rebalance_threshold = 5.0;
    get_value_of_btc_in_usd(amount_satoshis);
    println!("Number of USD: {}", amount_usd);
    println!("Number of bitcoin is {}", get_satoshis_as_bitcoin(amount_satoshis));
    println!("Total value of BTC in portfolio in USD is {}",get_value_of_btc_in_usd(amount_satoshis));
    let value_of_btc_in_usd = get_value_of_btc_in_usd(amount_satoshis);
    println!("Current percentage BTC: {}%", current_percentage_btc(value_of_btc_in_usd, amount_usd));
    println!("Current percentage USD: {}%", current_percentage_usd(value_of_btc_in_usd, amount_usd));
}

fn get_value_of_btc_in_usd(amount_satoshis: i32) -> f64
{
     let price_btc = get_price();
     let amount_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
     price_btc * amount_of_bitcoin
}

fn get_price() -> f64
{   let price_btc = 4000.00;
    println!("Price of each bitcoin in USD: {}", price_btc);
    price_btc
}

fn get_satoshis_as_bitcoin(amount_satoshis: i32) -> f64
{
    let conversion_rate = 100000000; //100,000,000
   (amount_satoshis as f64 / conversion_rate as f64)
}

fn current_percentage_btc(amount_btc: f64, value_usd: f64) -> f64
{
    let total = amount_btc + value_usd;
    let percentage_btc = amount_btc * 100.00 / total;
    percentage_btc.round()
}

fn current_percentage_usd(amount_btc: f64, value_usd: f64) -> f64{
 let total = amount_btc + value_usd;
    let percentage_usd = value_usd * 100.00 / total;
    percentage_usd.round()
}

fn calculate_gap () {
    //calculates gap between amount_btc_in_usd and value_usd
}

fn calculate_threshold_up_price(amount_btc: f64, value_usd: f64, percentageRebalanceThreshold: f64) -> f64{
   //returns what price is needed to reach threshold_up
   //reached if gap between (amountSatoshis*priceBTC) and amoutUSD > percentageRebalanceThreshold

    50.0
}

fn calculate_threshold_down_price() -> f64 {
  //returns what price is needed to reach threshold_up and threshold_down
  //reached if gap between (amountSatoshis*priceBTC) and amoutUSD > percentageRebalanceThreshold
   
  50.0
}
