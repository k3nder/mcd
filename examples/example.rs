use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use dwldutil::Downloader;
use mcd::{
    api::ApiClientUtil,
    command::{self, Command},
    file::fetch_client,
    java::JavaUtil,
    libs::LibsUtil,
    resource::ResourceUtil,
};
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let api_client = ApiClientUtil::new("./test/manifest.json")?;
    let java = JavaUtil::new();
    let libs = LibsUtil::new();
    let resources = ResourceUtil::new();
    let client = api_client.fetch("1.21.1", "./1.21.1.json")?;

    dbg!(&client);

    let mut files = Vec::new();

    dbg!(&client.java());

    match java.fetch(client.java(), "./test/java") {
        Ok(f) => files.push(f),
        Err(mcd::errors::FetchError::PathAlredyExist(_)) => {}
        Err(e) => error!("{}", e),
    };
    match fetch_client(&client, "test/game.jar") {
        Ok(f) => files.push(f),
        Err(mcd::errors::FetchError::PathAlredyExist(_)) => {}
        Err(e) => error!("{}", e),
    }
    let mut classpath = match libs.fetch("./test/libraries", "./test/bin", &client) {
        Ok((mut f, classpath)) => {
            debug!("LIBS: {}", f.len());
            files.append(&mut f);
            classpath
        }
        Err(mcd::errors::FetchError::PathAlredyExist(_)) => Vec::new(),
        Err(e) => {
            error!("{}", e);
            Vec::new()
        }
    };
    //classpath.push(String::from("test/game.jar"));

    //let mut file = File::create_new("./classpath")?;
    //file.write_all(classpath.join(":").as_bytes());
    //let mut file = File::open("classpath")?;
    //let mut classpath = String::new();
    //file.read_to_string(&mut classpath)?;

    //let index = resources.index_of(&client, "./test/index.json")?;
    //match resources.fetch(&index, "./test/assets") {
    //    Ok(mut f) => files.append(&mut f),
    //                    Err(mcd::errors::FetchError::PathAlredyExist(_)) => {}
    //                    Err(e) => error!("{}", e),
    //}
    Downloader::<dwldutil::indicator::indicatif::Indicatif>::new()
        .with_max_redirections(30)
        .with_max_concurrent_downloads(1)
        .with_files(files)
        .start();

    // let mut data = HashMap::new();

    // data.insert("natives_directory".to_owned(), "/home/kristian/Documentos/proyects/mcdu/mcd/test/bin".to_owned());
    // data.insert("launcher_name".to_owned(), "theseus".to_owned());
    // data.insert("launcher_version".to_owned(), "0.9.5".to_owned());
    // data.insert(
    //     "classpath".to_owned(),
    //     format!("{}", classpath.join(":")),
    // );
    // data.insert("main_class".to_owned(), client.main_class.to_owned());
    // data.insert("auth_player_name".to_owned(), "ddd".to_owned());
    // data.insert("version_name".to_owned(), "1.21.5".to_owned());
    // data.insert("game_directory".to_owned(), "/home/kristian/.local/share/ModrinthApp/profiles/dd".to_owned());
    // data.insert("assets_root".to_owned(), "/home/kristian/.local/share/ModrinthApp/meta/assets".to_owned());
    // data.insert(
    //     "game_assets".to_owned(),
    //     "./test/assets/virtual/legacy/".to_owned(),
    // );
    // data.insert("assets_index_name".to_owned(), client.assets.clone());
    // data.insert("auth_uuid".to_owned(), "d3ae2061edcd4cdebe44af586ca1a1a9".to_owned());
    // data.insert("auth_access_token".to_owned(), "0".to_owned());
    // data.insert("clientid".to_owned(), "c4502edb-87c6-40cb-b595-64a280cf8906".to_owned());
    // data.insert("auth_xuid".to_owned(), "90".to_owned());
    // data.insert("user_type".to_owned(), "msa".to_owned());
    // data.insert("version_type".to_owned(), "vanilla".to_owned());
    // data.insert("user_properties".to_owned(), "".to_owned());
    // data.insert("library_directory".to_owned(), "/home/kristian/Documentos/proyects/mcdu/mcd/test/libraries".to_owned());
    // data.insert("classpath_separator".to_owned(), ":".to_owned());
    // data.insert("log_file".to_owned(), "test/log4j2.xml".to_owned());
    // data.insert("file".to_owned(), "test/1.21.5.jar".to_owned());

    // let (game, jvm) = command::build_args(&client, HashMap::new());

    // Command::from_args(game, jvm, data).execute(
    //     "/home/kristian/.local/share/ModrinthApp/meta/java_versions/zulu21.42.19-ca-jre21.0.7-linux_x64/bin/java".to_owned(),
    //     vec![],
    //     client.java()
    // )?;

    Ok(())
}
