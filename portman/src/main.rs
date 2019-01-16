    fn main() {
    let wanted_percentage_bitcoin = 50;
    let wanted_percentage_usd = 50;
    let amount_satoshis = 250000000;
    get_price();
    get_value_of_btc_in_usd(amount_satoshis);
}

fn get_price() -> f64
{   let price_btc = 4000.00;
    price_btc
}

fn get_value_of_btc_in_usd(amount_satoshis: i32) -> f64
{
     let price_btc = get_price();
     
     let amount_of_bitcoin = get_satoshis_as_bitcoin(amount_satoshis);
     println!("Amount of bitcoin is {}", amount_of_bitcoin );
      let value_of_btc_in_usd = price_btc * amount_of_bitcoin;
     println!("Total value of BTC in portfolie is {}",value_of_btc_in_usd );
     value_of_btc_in_usd
}

fn get_satoshis_as_bitcoin(amount_satoshis: i32) -> f64
{
    let conversion_rate = 100000000; //100,000,000
    println!("Amount of satoshis are {}", amount_satoshis);
   (amount_satoshis / conversion_rate) as f64
}

fn calculate_threshold_price() -> i32{
   //returns what price is needed to reach threshold
   //reached if gap between (amountSatoshis*priceBTC) and amoutUSD > percentageRebalanceThreshold
    let amount_satoshis = 100000000;
    let amount_usd = 5000;
    let percentage_rebalance_threshold = 5;

    50
}
