use std::{collections::HashMap, env, fs, process::Command};
use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
struct Engine {
    url: String,
    keyword: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("📝: s [검색엔진] [검색어]");
        return Ok(());
    }

    // Read the engines.yaml file
    let home_dir = dirs::home_dir().expect("홈 디렉토리를 찾을 수 없습니다.");
    let yaml_path = home_dir.join(".config").join("files").join("engines.yaml");
    let yaml_content = fs::read_to_string(&yaml_path)?;

    // Deserialize the YAML content into a HashMap
    let engines: HashMap<String, Engine> = serde_yaml::from_str(&yaml_content)?;

    let keyword = args[1].to_lowercase();
    let query = args[2..].join(" ");
    let encoded_query = encode(&query);

    let mut found_engine: Option<&Engine> = None;
    let mut found_engine_name: Option<String> = None;

    for (name, engine) in &engines {
        if engine.keyword == keyword {
            found_engine = Some(engine);
            found_engine_name = Some(name.clone());
            break;
        }
    }

    if let (Some(engine), Some(name)) = (found_engine, found_engine_name) {
        let search_url = engine.url.replace("%s", &encoded_query);

        #[cfg(target_os = "windows")]
        Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(&search_url)
            .spawn()?;

        #[cfg(target_os = "macos")]
        Command::new("open")
            .arg(&search_url)
            .spawn()?;

        #[cfg(target_os = "linux")]
        Command::new("xdg-open")
            .arg(&search_url)
            .spawn()?;

        println!("🌐{} 🔎{}", name, query);
    } else {
        println!("❌ 검색엔진 없음: {}", keyword);
        println!("📃 검색엔진 목록:");
        for (name, engine) in &engines {
            println!("  🔹{}: {}", engine.keyword, name);
        }
    }

    Ok(())
}
