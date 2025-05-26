use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::Stdio;

use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::api::client::{ArgumentValue, Client, ComplexArgument};
use crate::util::{resolve_rules_feat, FillingUtil};

pub struct Command {
    pub fill: FillingUtil,
    pub game: Vec<String>,
    pub jvm: Vec<String>,
    pub stdout: fn(String),
    pub stderr: fn(String),
}
impl Command {
    pub fn new(client: &Client, data: HashMap<String, String>, options: HashMap<String, bool>) -> Self {
        let fill = FillingUtil::new().with_data(data);
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
            game = Self::parse(&args.game, &options);
            jvm = Self::parse(&args.jvm, &options);
            if !jvm.last().unwrap().eq(&String::from("${main_class}")) {
                jvm.push(String::from("${main_class}"));
            }
        }
        Command {
            fill,
            game,
            jvm,
            stdout: |f| {},
            stderr: |f| {}
        }

    }
    fn parse(arguments: &Vec<ArgumentValue>, options: &HashMap<String, bool>) -> Vec<String> {
        let mut result = Vec::new();

        for arg in arguments {
            match arg {
                ArgumentValue::Plain(str) => result.push(str.to_owned()),
                ArgumentValue::Complex(complex_argument) => {
                    if let Some(args) = Self::resolve_complex(complex_argument, options) {
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
    fn build_args(&self, args: &Vec<String>) -> Vec<String> {
        args.par_iter()
            .map(|f| self.fill.fill(f.clone()).unwrap_or(f.clone())).collect()
    }
    pub fn build_game_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.game)
    }
    pub fn build_jvm_args(&self) -> Vec<String> {
        Self::build_args(&self, &self.jvm)
    }
    pub fn stdout(mut self, stdout: fn(String)) {
        self.stdout = stdout;
    }
    pub fn stderr(mut self, stderr: fn(String)) {
        self.stderr = stderr;
    }
    pub fn execute(self, java: String, jvm: Vec<String>) {
        let mut args = self.build_jvm_args();
        let mut game = self.build_game_args();
        let mut extra = jvm;
        args.append(&mut extra);
        //args.push(self.fill.fill(String::from("${main_class}")).unwrap());
        args.append(&mut game);

        debug!("{:?}", &args);


        let mut java = std::process::Command::new(java);
        let child = java.args(args);
        let mut child = child
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                //println!("run");
                // Obtener el stdout del proceso hijo
                let stdout = child.stdout.take().expect("Failed to capture stdout");
                let stderr = child.stderr.take().expect("Failed to capture stderr");

                // Leer la salida del proceso hijo de manera asíncrona
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    (self.stdout)(line.unwrap())
                }

                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    (self.stderr)(line.unwrap())
                }

                // Esperar a que el proceso hijo termine
                child.wait().unwrap();
    }
}
