struct ProductOfNumbers {
    products: Vec<i32>
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            products: vec![1]
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.products = vec![1];
        } else {
            self.products.push(self.products.last().unwrap() * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        if k as usize >= self.products.len() {
            return 0;
        }

        self.products[self.products.len() - 1] / self.products[self.products.len() - k as usize - 1]
    }
}