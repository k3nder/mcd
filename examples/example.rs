use std::collections::HashMap;

use dwldutil::Downloader;
use log::error;
use mcd::{
    api::ApiClientUtil, command::Command, file::fetch_client, java::JavaUtil, libs::LibsUtil,
    resource::ResourceUtil,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let api_client = ApiClientUtil::new("./test/manifest.json")?;
    let java = JavaUtil::new();
    let libs = LibsUtil::new();
    let resources = ResourceUtil::new();
    let client = api_client.fetch("1.8.9", "./test/1.8.9.json")?;

    let mut files = Vec::new();

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
    match libs.fetch("./test/libs", "./test/bin", &client) {
        Ok(mut f) => files.append(&mut f),
        Err(mcd::errors::FetchError::PathAlredyExist(_)) => {}
        Err(e) => error!("{}", e),
    }
    let index = resources.index_of(&client, "./test/index.json")?;
    match resources.fetch(&index, "./test/assets") {
        Ok(mut f) => files.append(&mut f),
                        Err(mcd::errors::FetchError::PathAlredyExist(_)) => {}
                        Err(e) => error!("{}", e),
    }

    Downloader::new()
        .with_max_redirections(30)
        .with_files(files)
        .start();

    let mut data = HashMap::new();

    data.insert("natives_directory".to_owned(), "./test/bin".to_owned());
    data.insert("launcher_name".to_owned(), "mcdlib".to_owned());
    data.insert("launcher_version".to_owned(), "1.0-alpha".to_owned());
    data.insert(
        "classpath".to_owned(),
        "./test/libs/*:./test/game.jar".to_owned(),
    );
    data.insert("main_class".to_owned(), client.main_class.to_owned());
    data.insert("auth_player_name".to_owned(), "ddd".to_owned());
    data.insert("version_name".to_owned(), "1.21.5".to_owned());
    data.insert("game_directory".to_owned(), "test/".to_owned());
    data.insert("assets_root".to_owned(), "23".to_owned());
    data.insert(
        "game_assets".to_owned(),
        "./test/assets/virtual/legacy/".to_owned(),
    );
    data.insert("assets_index_name".to_owned(), client.assets.clone());
    data.insert("auth_uuid".to_owned(), "000".to_owned());
    data.insert("auth_access_token".to_owned(), "0".to_owned());
    data.insert("clientid".to_owned(), "0".to_owned());
    data.insert("auth_xuid".to_owned(), "90".to_owned());
    data.insert("user_type".to_owned(), "normal".to_owned());
    data.insert("version_type".to_owned(), "vanilla".to_owned());
    data.insert("user_properties".to_owned(), "".to_owned());

    Command::new(&client, data, HashMap::new()).stdout(|f| println!("{}", f)).stderr(|f| println!("{}", f)).execute(
        format!(
            "/home/kristian/Documentos/proyects/mcdu/mcd/test/java/{}/bin/java",
            java.id_of(client.java()).unwrap()
        ),
        vec![],
    );

    Ok(())
}
