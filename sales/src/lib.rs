#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    
    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total_items = prices.len();
        let free_items = total_items / 3;

        let mut free_price: f32 = 0.0;
        for _ in 0..free_items {
            if let Some(cheapest) = prices.first() {
                free_price += cheapest;
            }
            prices.remove(0);
        }

        let total_price: f32 = prices.iter().sum();
        let modified_price = total_price - free_price;

        let adjustment = (modified_price / prices.len() as f32) / 1000.0;

        self.receipt = prices.iter().map(|&price| (price - adjustment).round().into()).collect();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_item() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
        ]);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));

        assert_eq!(cart.items.len(), 2);
        assert_eq!(cart.items[0].0, "product A");
        assert_eq!(cart.items[1].0, "product B");
    }
    #[test]
    fn test_generate_receipt() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
        ]);

        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));

        let receipt = cart.generate_receipt();
        assert_eq!(receipt.len(), 3);
    }
}
