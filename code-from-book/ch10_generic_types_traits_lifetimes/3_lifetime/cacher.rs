use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {}", cache.value(intensity));
        }
    }
}

struct Cacher<T, K, V>
where
    K: Eq + Hash + Copy,
    T: Fn(K) -> V,
{
    calculation: T,
    args_map: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    K: Eq + Hash + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            args_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        // `V` is the value of the hashmap to the key `K` supplied to it.
        // We have to implement `Copy` for `V`, because otherwise `V` would
        // move out from the hashmap, which will give error.
        *(self.args_map.entry(arg).or_insert((self.calculation)(arg)))
    }
}

fn main() {
    let sim_user_val = 10;

    let sim_rand_num = 7;

    generate_workout(sim_user_val, sim_rand_num);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cacher_with_different_args() {
        let mut cacher = Cacher::new(|a| a);

        let v1 = cacher.value(1);
        let v2 = cacher.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn cacher_with_different_closure_args() {
        let mut c = Cacher::new(|s: &str| -> usize { s.len() });

        let v1: usize = c.value("asd");
        let v2: usize = c.value("inodeska");

        assert_eq!(v1, 3);
        assert_eq!(v2, 8);
    }
}
