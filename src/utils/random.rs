use rand::Rng;

pub struct WeightedRandom<T> {
    items: Vec<(T, f32)>,
    total_weight: f32,
}

impl<T> WeightedRandom<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            total_weight: 0.0,
        }
    }

    pub fn add(&mut self, item: T, weight: f32) {
        self.total_weight += weight;
        self.items.push((item, weight));
    }

    pub fn select<R: Rng>(&self, rng: &mut R) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }

        let mut random = rng.gen::<f32>() * self.total_weight;

        for (item, weight) in &self.items {
            if random < *weight {
                return Some(item);
            }
            random -= weight;
        }

        // Fallback to last item (handles floating point precision issues)
        self.items.last().map(|(item, _)| item)
    }

    pub fn items(&self) -> &[(T, f32)] {
        &self.items
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.total_weight = 0.0;
    }
}

impl<T> Default for WeightedRandom<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_weighted_random() {
        let mut weighted = WeightedRandom::new();
        weighted.add("common", 100.0);
        weighted.add("rare", 10.0);
        weighted.add("legendary", 1.0);

        let mut rng = StdRng::seed_from_u64(42);

        let mut counts = std::collections::HashMap::new();
        for _ in 0..1000 {
            if let Some(item) = weighted.select(&mut rng) {
                *counts.entry(*item).or_insert(0) += 1;
            }
        }

        // Common should appear more often than rare, which should appear more than legendary
        assert!(counts["common"] > counts["rare"]);
        assert!(counts["rare"] > counts["legendary"]);
    }
}
