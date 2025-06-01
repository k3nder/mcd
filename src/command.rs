use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Child;

use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::api::client::{ArgumentValue, Client, ComplexArgument};
use crate::errors::CommandError;
use crate::util::{resolve_rules_feat, FillingUtil};

pub struct Command {
    pub fill: FillingUtil,
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}
impl Command {
    pub fn from_args(game: Vec<String>, jvm: Vec<String>, data: HashMap<String, String>) -> Self {
        let fill = FillingUtil::new().with_data(data);
                Command {
                    fill,
                    game,
                    jvm,
                }
    }

    fn build_args(&self, args: &Vec<String>) -> Vec<String> {
        args.par_iter()
            .map(|f| {
                let mut filled = self.fill.fill(f.clone()).unwrap_or(f.clone());
                if filled.len() >= 140 {
                    filled.insert(0, '\"');
                    filled.insert(filled.len(), '\"');
                }
                filled
            }).collect()
    }
    pub fn build_game_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.game)
    }
    pub fn build_jvm_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.jvm)
    }
    pub fn execute(self, java: String, jvm: Vec<String>, java_version: usize) -> Result<Child, CommandError> {
        let mut args = self.build_jvm_args();
        let mut game = self.build_game_args();
        let mut extra = jvm.clone();
        args.append(&mut extra);
        args.append(&mut game);

        debug!("{:?}", &args);

        let mut temp = temp_dir();
        temp.push(format!("command-run-{}.tmp", rand::random_range(0..20)));
        debug!("TEMP COMMAND ARGS: {}", temp.to_str().unwrap());
        if temp.exists() {
            fs::remove_file(&temp).unwrap();
        }
        let mut file = File::create_new(temp.as_path())?;
        file.write_all(args.join("\n").as_bytes())?;

        let mut java = std::process::Command::new(java);

        let args = if java_version == 8 {
            file_to_args(temp.to_str().unwrap())
        } else {
            vec![format!("@{}", temp.canonicalize().unwrap().to_str().unwrap())]
        };

        let child = java.args(args).envs(std::env::vars()).env("JAVA_HOME", "/home/kristian/.local/share/ModrinthApp/meta/java_versions/zulu21.42.19-ca-jre21.0.7-linux_x64/");
        Ok(child
            .spawn()?)
    }
}
pub fn build_args(client: &Client, options: HashMap<String, bool>) -> (Vec<String>, Vec<String>) {

    let mut game: Vec<String> = Vec::new();
    let mut jvm: Vec<String> = Vec::new();

    jvm.push(String::from("-Djava.library.path=${natives_directory}"));
    jvm.push(String::from("-cp"));
    jvm.push(String::from("${classpath}"));
    jvm.push(String::from("${main_class}"));

    if let Some(args) = &client.minecraft_arguments {
        let args: Vec<String> = args.split(" ").map(|f| f.to_owned()).collect();
        game = args;
    }
    if let Some(args) = &client.arguments {
        game = parse(&args.game, &options);
        jvm = parse(&args.jvm, &options);
        if !jvm.last().unwrap().eq(&String::from("${main_class}")) {
            jvm.push(String::from("${main_class}"));
        }
    }

    (game, jvm)
}
fn parse(arguments: &Vec<ArgumentValue>, options: &HashMap<String, bool>) -> Vec<String> {
        let mut result = Vec::new();

        for arg in arguments {
            match arg {
                ArgumentValue::Plain(str) => result.push(str.to_owned()),
                ArgumentValue::Complex(complex_argument) => {
                    if let Some(args) = resolve_complex(complex_argument, options) {
                        let mut args: Vec<String> = args.clone().iter().map(|f| f.to_owned()).collect();
                        result.append(&mut args);
                    }
                },
            }
        }

        result
    }
    fn resolve_complex(complex: &ComplexArgument, options: &HashMap<String, bool>) -> Option<Vec<String>> {
        if !resolve_rules_feat(&complex.rules, options) {
            return None;
        }
        let mut result = Vec::new();

        match &complex.value {
            crate::api::client::ValueField::Single(str) => result.push(str.to_owned()),
            crate::api::client::ValueField::Multiple(items) => result.append(&mut items.clone()),
        }

        Some(result)
    }
fn file_to_args(file: &str) -> Vec<String> {
    let mut result = Vec::new();
    let file = File::open(file).unwrap();
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let line = line.unwrap().trim_matches('\"').to_owned();
        result.push(line);
    }
    debug!("FILE READED TO {:?}", result);
    result
}
