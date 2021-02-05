use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct TestcaseEntry {
    pub id: i32,
    pub is_random: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Testcase {
    pub id: i32,
    pub sources: Vec<File>,
    pub hidden: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub language: String,
    pub compile_args: Vec<String>,
    pub sources: Vec<File>,
    pub git_repo_name: Option<String>,
    pub entry_point: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Require {
    pub on: String,
    pub cond: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub check: Option<String>,
    pub run: Option<String>,
    pub compare: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Limits {
    pub time: Option<i64>,
    pub memory: Option<i64>,
    pub file: Option<i64>,
    pub proc: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub preset: Option<String>,
    pub require: Option<Require>,
    pub script: Option<Script>,
    pub limits: Option<Limits>,
    pub testcase: Option<TestcaseEntry>,
    pub grade: i32,
    pub replicas: Option<Vec<Stage>>,
}

#[derive(Serialize, Deserialize)]
pub struct JudgeConfig {
    pub id: i32,
    pub version: String,
    pub r#type: String,
    pub stages: Vec<Stage>,
    pub program: Program,
    pub random_generator: Option<Program>,
    pub custom_comparator: Option<Program>,
    pub testcases: Vec<Testcase>,
}

#[derive(Serialize, Deserialize)]
pub struct JudgeResult {
    pub id: i32,
    pub status: String,
}

impl Display for JudgeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self);
        match json {
            Ok(s) => f.write_str(s.as_str()),
            Err(_) => Err(std::fmt::Error {}),
        }
    }
}

impl Display for JudgeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self);
        match json {
            Ok(s) => f.write_str(s.as_str()),
            Err(_) => Err(std::fmt::Error {}),
        }
    }
}
