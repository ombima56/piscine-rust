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
        let mut receipt = Vec::new();
        let mut i = 0;
        
        // Process items in groups of 3
        while i + 2 < prices.len() {
            let group = vec![prices[i], prices[i + 1], prices[i + 2]];
            let sum: f32 = group.iter().sum();
            let min = *group.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            
            // Each item gets discounted proportionally
            for &price in &group {
                let discount = (price / sum) * min;
                let discounted_price = price - discount;
                receipt.push(((discounted_price * 100.0).round()) / 100.0);
            }
            
            i += 3;
        }
        
        // Add remaining items without discount
        while i < prices.len() {
            receipt.push(((prices[i] * 100.0).round()) / 100.0);
            i += 1;
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
