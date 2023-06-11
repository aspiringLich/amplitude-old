use serde::{Serialize, Deserialize};

use crate::lang::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCases {
    #[serde(flatten)]
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    
}

pub struct TestCaseConfig {
    pub hidden_cases: u32,
    pub visible_cases: u32,
    pub seed: i64,
}

