use std::collections::HashMap;

use crate::user::User;

pub trait Upsertable<'a> {
    type K;
    type V;
    fn upsert_all(&mut self, map: &HashMap<&'a Self::K, Self::V>);
}

impl<'a> Upsertable<'a> for HashMap<&'a User, f32> {
    type K = User;
    type V = f32;
    fn upsert_all(&mut self, map: &HashMap<&'a Self::K, Self::V>) {
        for (key, value) in map.iter() {
            match self.get_mut(key) {
                None => {
                    self.insert(key, *value);
                }
                Some(v) => {
                    *v += value;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upsert() {
        let user_one: User = User::new("user_one").unwrap();
        let user_two: User = User::new("user_two").unwrap();

        let mut hashmap_one: HashMap<&User, f32> = HashMap::new();
        hashmap_one.insert(&user_one, 1.23);
        hashmap_one.insert(&user_two, 4.512);

        let mut hashmap_two: HashMap<&User, f32> = HashMap::new();
        hashmap_two.insert(&user_one, 5.4);
        hashmap_two.insert(&user_two, 6.3);

        hashmap_two.upsert_all(&hashmap_one);

        assert_eq!(hashmap_two.get(&user_one), Some(&6.63));
        assert_eq!(hashmap_two.get(&user_two), Some(&10.812));
    }
}
