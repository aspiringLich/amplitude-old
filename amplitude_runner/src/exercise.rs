use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{var_type::VariableType, lang::Language};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: VariableType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DynStruct {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionConfig {
    inputs: Vec<VariableType>,
    output: VariableType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseConfig {
    title: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(flatten)]
    functions: HashMap<String, FunctionConfig>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ExcerciseSerialize")]
pub struct Exercise {
    config: ExerciseConfig,
    code: HashMap<Language, String>,
}

impl Exercise {
    pub fn new(config: ExerciseConfig, code: HashMap<Language, String>) -> Self {
        Self { config, code }
    }
}

impl Into<ExcerciseSerialize> for Exercise {
    fn into(self) -> ExcerciseSerialize {
        ExcerciseSerialize {
            title: self.config.title,
            instructions: self.config.instructions,
            code: self.code,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExcerciseSerialize {
    title: String,
    instructions: String,
    code: HashMap<Language, String>,
}

