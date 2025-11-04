use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MergeSplits {
    merged: HashMap<String, String>,
    reversed: HashMap<String, Vec<String>>,
}

impl From<&str> for MergeSplits {
    fn from(value: &str) -> Self {
        let mut result = HashMap::new();
        value
            .split('|')
            .filter_map(|v| {
                let mut all = v.split(':');
                let key = all.next()?;
                let data = all.next()?;

                Some((key.to_string(), data))
            })
            .for_each(|(key, data)| {
                data.split(',').for_each(|v| {
                    result.insert(v.to_string(), key.clone());
                });
            });

        // println!("{:?}", result);

        Self {
            reversed: Self::reverse(&result),
            merged: result,
        }
    }
}

impl Into<String> for &MergeSplits {
    fn into(self) -> String {
        let mut hash_map: HashMap<&String, Vec<&String>> = HashMap::new();

        for (split_name, split_result) in &self.merged {
            match hash_map.get_mut(split_result) {
                Some(vec) => vec.push(split_name),
                None => {
                    hash_map.insert(split_result, vec![split_name]);
                }
            }
        }

        let mut result = "".to_owned();
        let big_len = hash_map.len();
        for (id, (key, vec)) in hash_map.iter().enumerate() {
            result.push_str(&format!("{key}:"));
            let len = vec.len();
            for (id, elem) in vec.iter().enumerate() {
                if len == id + 1 {
                    result.push_str(elem);
                    break;
                }

                result.push_str(&format!("{elem},"));
            }

            if id + 1 < big_len {
                result.push_str("|");
            }
        }
        result
    }
}

impl MergeSplits {
    pub fn get_split(&self, split_name: &str) -> Option<&String> {
        self.merged.get(split_name)
    }

    pub fn get_req_splits(&self, split_name: &str) -> Option<&Vec<String>> {
        self.reversed.get(split_name)
    }

    fn reverse(merged: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for (key, value) in merged {
            match result.get_mut(value) {
                Some(vec) => vec.push(key.clone()),
                None => {
                    result.insert(value.clone(), vec![key.clone()]);
                }
            }
        }

        result
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LevelsMergeSplits {
    levels: HashMap<String, MergeSplits>,
}

impl LevelsMergeSplits {
    pub fn get_level(&self, level_obj: &str) -> Option<&MergeSplits> {
        self.levels.get(level_obj)
    }

    pub fn add_level(&mut self, level_obj: &str, data: impl Into<MergeSplits>) {
        self.levels.insert(level_obj.to_owned(), data.into());
    }
}
