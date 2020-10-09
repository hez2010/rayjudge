use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestcaseEntry {
    pub id: i32,
    pub is_random: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct Testcase {
    pub id: i32,
    pub sources: Vec<File>,
    pub hidden: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct File {
    pub path: String,
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Program {
    pub language: String,
    pub compile_args: Vec<String>,
    pub sources: Vec<File>,
    pub git_repo_name: Option<String>,
    pub entry_point: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Require {
    pub on: String,
    pub cond: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Script {
    pub check: Option<String>,
    pub run: Option<String>,
    pub compare: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Limits {
    pub time: Option<i64>,
    pub memory: Option<i64>,
    pub file: Option<i64>,
    pub proc: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct Stage {
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
struct JudgeConfig {
    pub version: String,
    pub r#type: String,
    pub stages: Vec<Stage>,
    pub program: Program,
    pub random_generator: Option<Program>,
    pub custom_comparator: Option<Program>,
    pub testcases: Vec<Testcase>,
}

impl Display for JudgeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self);
        return match json {
            Ok(s) => f.write_str(s.as_str()),
            Err(_) => Err(std::fmt::Error {})
        };
    }
}

fn main() {
    let x: JudgeConfig = JudgeConfig {
        version: "v5".to_string(),
        r#type: "programming".to_string(),
        stages: Vec::new(),
        program: Program {
            language: "csharp".to_string(),
            compile_args: Vec::new(),
            sources: Vec::new(),
            git_repo_name: None,
            entry_point: None,
        },
        random_generator: None,
        custom_comparator: None,
        testcases: Vec::new(),
    };

    println!("{}", x);
}
