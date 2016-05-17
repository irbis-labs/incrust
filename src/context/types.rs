use ::context::{EntityId, Var, Value, Array, Map, KIterator, VIterator, KVIterator};


impl <'a> Into<Var> for &'a str { fn into(self) -> Var { self.to_owned().into() } }


impl Into<Var> for HashMap<EntityId, Var> { fn into(self) -> Var { Var::Map(Box::new(self)) } }

impl Value for HashMap<EntityId, Var> {
    fn v_render(&self) -> String { format!("HashMap({})", self.len()) }
    fn f_value(&self) -> f64 { if self.is_empty() { 0 } else { 1 } }
}

impl Array for HashMap<EntityId, Var> {
    fn a_len(&self) -> usize { self.len() }
    fn a_is_empty(&self) -> bool { self.is_empty() }
    fn a_values(&self) -> VIterator {
        VIterator { me: self.values() }
    }
}
impl Map for HashMap<EntityId, Var> {
    fn m_keys(&self) -> KIterator {
        KIterator { me: self.keys() }
    }
    fn m_key_values(&self) -> KVIterator {
        KVIterator { me: self.iter() }
    }
}


#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    use ::context::{Var};
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn string() {
        let var = Var::ex("Hello, World!");
        let expected = "Hello, World!".to_owned();
        assert_eq!(expected, var.render());
        //        assert_eq!(expected.len(), var.len());
    }

    #[test]
    fn usize() {
        let var = Var::ex(42usize);
        let expected = "42".to_owned();
        assert_eq!(expected, var.render());
    }

    #[test]
    fn isize() {
        let var = Var::ex(-42isize);
        let expected = "-42".to_owned();
        assert_eq!(expected, var.render());
    }

    #[test]
    fn hashmap() {
        let var = Var::ex(hashmap!{
            "k1" => Var::ex("v1"),
            "k2" => Var::ex("v2"),
        });
        assert_eq!("HashMap(2)", var.render());
    }
}
