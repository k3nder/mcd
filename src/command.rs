use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::api::client::{ArgumentValue, Client, ComplexArgument};
use crate::util::{FillingUtil, resolve_rules_feat};

pub struct Command {
    pub fill: FillingUtil,
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}
impl Command {
    pub fn from_args(game: Vec<String>, jvm: Vec<String>, data: HashMap<String, String>) -> Self {
        let fill = FillingUtil::new().with_data(data);
        Command { fill, game, jvm }
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
            })
            .collect()
    }
    pub fn build_game_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.game)
    }
    pub fn build_jvm_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.jvm)
    }
    pub fn build(&self, extra: Vec<String>) -> Vec<String> {
        let mut args = self.build_jvm_args();
        let mut game = self.build_game_args();
        let mut extra = extra.clone();
        args.append(&mut extra);
        args.append(&mut game);
        args
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
            }
        }
    }

    result
}
fn resolve_complex(
    complex: &ComplexArgument,
    options: &HashMap<String, bool>,
) -> Option<Vec<String>> {
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
