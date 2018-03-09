
pub struct Trade<K> {
    pub buy_key: K,
    pub sell_key: K,

    pub price: f64,
    pub volume: f64,
}
