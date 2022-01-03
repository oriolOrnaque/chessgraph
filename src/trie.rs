use std::{
    collections::HashMap,
    hash::Hash,
    clone::Clone,
    fmt::{Display, Formatter, Result},
};

use uuid::{Uuid};

use crate::gviz::{GvizNode};

#[derive(Debug)]
pub struct Trie<V>
where
    V: PartialEq + Eq + Hash + Clone + GvizNode,
{
    uuid: Uuid,
    children: HashMap<V, Trie<V>>,    
}

impl<V> Trie<V>
where
    V: PartialEq + Eq + Hash + Clone + GvizNode,
{
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            children: HashMap::<V, Trie<V>>::new(),
        }
    }

    pub fn add_path(&mut self, path: Vec<V>) {
        let mut node = self;
        for value in path {
            node = node.children.entry(value).or_insert(Trie::<V>::new());
        }
    }
}

impl<V> Display for Trie<V>
where
    V: PartialEq + Eq + Hash + Clone + GvizNode,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for key in self.children.keys() {
            writeln!(f, "id{}_{}", 
                self.uuid.to_string().replace("-", "_"),
                key.id_with_attributes()
            ).unwrap();
        }

        for value in self.children.values() {
            writeln!(f, "{}", value).unwrap();
        }

        for (key, value) in self.children.iter() {
            for (node2, _) in value.children.iter() {
                writeln!(f, "id{}_{} -> id{}_{}",
                    self.uuid.to_string().replace("-", "_"),
                    key.id(),
                    value.uuid.to_string().replace("-", "_"),
                    node2.id()
                ).unwrap();
            }
        }
        writeln!(f)
    }
}