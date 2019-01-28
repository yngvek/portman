    fn main() {
    
    let default_percentage_bitcoin = 50.0;
    let default_percentage_usd = 50.0;
    //let amount_satoshis: i64 = 285714285; //default
    let amount_satoshis: i64 = 285714285;
    //let value_usd = 10000.00; //default
    let value_usd = 10000.0;
    let acceptable_deviation_threshold = 5.0;
    let number_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
    println!("--------Portfolio Manager------");
    println!("Number of BTC in portfolio: {}", number_of_bitcoin);
  
    let bitcoin_price_in_usd = get_price();
    println!("Price of BTC in USD: {}", bitcoin_price_in_usd);
    println!("Value of USD in portfolio: {}", value_usd);

    let value_of_btc_in_usd = get_value_of_btc_in_usd(amount_satoshis);
    println!("Value of BTC in portfolio: {} USD",value_of_btc_in_usd);
    println!("Total value of portfolio in USD: {}", value_usd + value_of_btc_in_usd);
    
    println!("Current percentage BTC: {:.1}%", current_percentage_btc(value_of_btc_in_usd, value_usd));
    println!("Current percentage USD: {:.1}%", current_percentage_usd(value_of_btc_in_usd, value_usd));
    println!("Threshold ({}%) price to sell bitcoin: > {:.1}", default_percentage_bitcoin+acceptable_deviation_threshold, calculate_threshold_up_price(number_of_bitcoin, 
        value_usd, default_percentage_usd, acceptable_deviation_threshold));
    println!("Threshold ({}%) price to buy bitcoin: < {:.1}", default_percentage_bitcoin-acceptable_deviation_threshold, calculate_threshold_down_price(number_of_bitcoin, 
        value_usd, default_percentage_usd, acceptable_deviation_threshold));
    println!("------------------------------");
    println!("Is threshold for selling bitcoin reached?: {}", is_threshold_to_sell_bitcoin_reached(value_of_btc_in_usd, 
        value_usd, default_percentage_bitcoin, acceptable_deviation_threshold));
    let value_of_bitcoin_in_usd_to_sell_for_rebalance = number_of_bitcoin_to_sell_for_rebalance(
        value_of_btc_in_usd, value_usd, default_percentage_bitcoin, acceptable_deviation_threshold);
    println!("How much bitcoin in USD to sell to reach full rebalance: {}", value_of_bitcoin_in_usd_to_sell_for_rebalance);   
    println!("Number of bitcoin to sell: {}", value_of_bitcoin_in_usd_to_sell_for_rebalance / bitcoin_price_in_usd);
    println!("Number of satoshis after selling bitcoin: {}", (amount_satoshis as f64 - (value_of_bitcoin_in_usd_to_sell_for_rebalance / bitcoin_price_in_usd) * 100000000.0));
    println!("Number of bitcoin after selling bitcoin: {}", number_of_bitcoin - (value_of_bitcoin_in_usd_to_sell_for_rebalance / bitcoin_price_in_usd));
    println!("Number of USD after selling bitcoin: {}", value_usd + value_of_bitcoin_in_usd_to_sell_for_rebalance);
    println!("------------------------------");
    println!("Is threshold for buying bitcoin reached?: {}", is_threshold_to_buy_bitcoin_reached(value_of_btc_in_usd, 
        value_usd, default_percentage_bitcoin, acceptable_deviation_threshold));
     let value_of_bitcoin_in_usd_to_buy_for_rebalance = number_of_bitcoin_to_buy_for_rebalance(
        value_of_btc_in_usd, value_usd, default_percentage_bitcoin, acceptable_deviation_threshold);
    println!("How much bitcoin in USD to buy to reach full rebalance: {}", value_of_bitcoin_in_usd_to_buy_for_rebalance); 
    println!("Number of bitcoin to buy: {}", value_of_bitcoin_in_usd_to_buy_for_rebalance / bitcoin_price_in_usd);
    println!("Number of satoshis after buying bitcoin: {}", (amount_satoshis as f64 + (value_of_bitcoin_in_usd_to_buy_for_rebalance / bitcoin_price_in_usd) * 100000000.0));
    println!("Number of bitcoin after buying bitcoin: {}",  number_of_bitcoin + (value_of_bitcoin_in_usd_to_buy_for_rebalance / bitcoin_price_in_usd));
    println!("Number of USD after buying bitcoin: {}", value_usd - value_of_bitcoin_in_usd_to_buy_for_rebalance);
    }

fn get_value_of_btc_in_usd(amount_satoshis: i64) -> f64
{
     let price_btc = get_price();
     let amount_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
     (price_btc * amount_of_bitcoin).round()
}

fn get_price() -> f64
{  
     //let price_btc = 3500.00;
     let price_btc = 5000.0;
    price_btc
}

fn get_price_api() -> f64
{
    // implement current BTC price from api
    3500.00
}
fn get_satoshis_as_bitcoin(amount_satoshis: i64) -> f64
{
    let conversion_rate = 100000000;
   (amount_satoshis as f64 / conversion_rate as f64)
}

fn current_percentage_btc(value_of_btc_in_usd: f64, value_usd: f64) -> f64
{
    let total = value_of_btc_in_usd + value_usd;
    let percentage_btc = value_of_btc_in_usd * 100.00 / total;
    percentage_btc
}

fn current_percentage_usd(value_of_btc_in_usd: f64, value_usd: f64) -> f64{
 let total = value_of_btc_in_usd + value_usd;
    let percentage_usd = value_usd * 100.00 / total;
    percentage_usd
}

fn calculate_threshold_up_price(number_of_bitcoin: f64, 
                                value_usd: f64, 
                                default_percentage_usd: f64, 
                                acceptable_deviation_threshold: f64)
                                -> f64
{
    let acceptable_band = default_percentage_usd - acceptable_deviation_threshold;
    let threshold_price = ((value_usd * 100.0 / acceptable_band) - value_usd) / number_of_bitcoin;
    threshold_price    
}

fn calculate_threshold_down_price(number_of_bitcoin: f64, 
                                value_usd: f64, 
                                default_percentage_usd: f64, 
                                acceptable_deviation_threshold: f64)
                                -> f64
{
    let acceptable_band = default_percentage_usd + acceptable_deviation_threshold;
    let threshold_price = ((value_usd * 100.0 / acceptable_band) - value_usd) / number_of_bitcoin;
    threshold_price    
}

fn is_threshold_to_sell_bitcoin_reached(value_of_btc_in_usd: f64, 
                                        value_usd: f64,
                                         default_percentage_bitcoin: f64, 
                                         acceptable_deviation_threshold: f64
                                         ) -> bool
{
    let acceptable_band = default_percentage_bitcoin + acceptable_deviation_threshold;
    if current_percentage_btc(value_of_btc_in_usd, value_usd) > acceptable_band
    {
        true
    }
    else {
        false
    }
 }

fn is_threshold_to_buy_bitcoin_reached(value_of_btc_in_usd: f64, 
                                        value_usd: f64,
                                         default_percentage_bitcoin: f64, 
                                         acceptable_deviation_threshold: f64
                                         ) -> bool
{
    let acceptable_band = default_percentage_bitcoin - acceptable_deviation_threshold;
    if current_percentage_btc(value_of_btc_in_usd, value_usd) < acceptable_band
    {
        true
    }
    else {
        false
    }
 }

fn number_of_bitcoin_to_sell_for_rebalance(value_of_btc_in_usd: f64, 
                                        value_usd: f64,
                                         default_percentage_bitcoin: f64, 
                                         acceptable_deviation_threshold: f64) -> f64
{                                         
     if is_threshold_to_sell_bitcoin_reached(value_of_btc_in_usd, 
        value_usd, default_percentage_bitcoin, acceptable_deviation_threshold)                          
        {
         let total_value = value_of_btc_in_usd + value_usd;
         let new_half_value = total_value / 2.0;
         new_half_value - value_usd
         }
    else {
        0.0
        }
   
}

fn number_of_bitcoin_to_buy_for_rebalance(value_of_btc_in_usd: f64, 
                                         value_usd: f64,
                                         default_percentage_bitcoin: f64, 
                                         acceptable_deviation_threshold: f64) -> f64
{                                         
     if is_threshold_to_buy_bitcoin_reached(value_of_btc_in_usd, 
        value_usd, default_percentage_bitcoin, acceptable_deviation_threshold)                          
        {
         let total_value = value_of_btc_in_usd + value_usd;
         let new_half_value = total_value / 2.0;
         value_usd - new_half_value
         }
    else {
        0.0
        }
   
}