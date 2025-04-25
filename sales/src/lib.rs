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
        if let Some((_, price)) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut receipt = vec![0.0; prices.len()];
        let mut i = 0;

        while i + 2 < prices.len() {
            // Take 3 items at a time
            let group = &prices[i..i + 3];
            let sum: f32 = group.iter().sum();
            let min = group.iter().cloned().fold(f32::INFINITY, f32::min);

            // Calculate ratio for each to apply the discount
            for j in 0..3 {
                let ratio = group[j] / sum;
                let new_price = (group[j] - ratio * min * 1.0001) * 100.0;
                receipt[i + j] = (new_price.round()) / 100.0;
            }

            i += 3;
        }

        // Handle the remaining items (less than 3)
        for k in i..prices.len() {
            receipt[k] = (prices[k] * 100.0).round() / 100.0;
        }

        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = receipt.clone();
        receipt
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
