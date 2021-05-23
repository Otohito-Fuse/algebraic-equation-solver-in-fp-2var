use std::cmp::Eq;
use std::fmt;
use std::hash::Hash;

/// 解集合
#[derive(Clone, Debug)]
pub struct SolutionSet<T: Hash> {
    solutions: std::collections::HashSet<T>,
}

impl<T: Hash> SolutionSet<T> {
    /// コンストラクタ。面倒なので所有権が移ることにしてしまった。
    pub fn new(s: std::collections::HashSet<T>) -> Self {
        Self { solutions: s }
    }

    pub fn size(&self) -> usize {
        self.solutions.len()
    }
}

impl<T: Hash + Clone> SolutionSet<T> {
    pub fn unwrap(&self) -> std::collections::HashSet<T> {
        self.solutions.clone()
    }
}

impl<T: Hash + Eq> SolutionSet<T> {
    /// 元を加える。
    pub fn insert(&mut self, t: T) {
        self.solutions.insert(t);
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
/// 型```T```がそもそも```Display```トレイトを実装していることを要求。
impl<T: fmt::Display + Hash> fmt::Display for SolutionSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();
        if self.solutions.len() == 0 {
            s.push_str(&"{ }");
        } else {
            s.push_str(&"{");
            for solution in &self.solutions {
                s.push_str(&solution.to_string());
                s.push_str(&", ");
            }
            s.pop();
            s.pop();
            s.push_str(&"}");
        }
        write!(f, "{}", s)
    }
}
