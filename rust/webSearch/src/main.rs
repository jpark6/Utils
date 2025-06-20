use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use serde::Deserialize;
use std::collections::HashMap;
use urlencoding::encode;
use webbrowser;

#[derive(Debug, Deserialize)]
struct Engine {
    Url: Option<String>,
    Keyword: Option<String>,
}

fn get_engines_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("홈 디렉토리를 찾을 수 없습니다.");
    home_dir.join(".config").join("websearch").join("engines.yaml")
}

fn load_engines(yaml_path: PathBuf) -> Result<HashMap<String, (String, String)>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(yaml_path)?;
    let parsed: HashMap<String, Engine> = serde_yaml::from_str(&content)?;

    let mut keyword_map = HashMap::new();
    for (name, props) in parsed {
        if let (Some(url), Some(keyword)) = (props.Url, props.Keyword) {
            keyword_map.insert(keyword.to_lowercase(), (url, name));
        }
    }
    Ok(keyword_map)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("📘 사용법: s [검색엔진코드] [검색어]");
        process::exit(1);
    }

    let engine_code = args[1].to_lowercase();
    let query = args[2..].join(" ");
    let encoded_query = encode(&query);

    let engines_path = get_engines_path();

    let engines = match load_engines(engines_path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("❌ engines.yaml 로딩 실패: {}", e);
            process::exit(1);
        }
    };

    if let Some((template, name)) = engines.get(&engine_code) {
        let url = template.replace("%s", &encoded_query);
        println!("🔍 검색 중: [{}] '{}'", name, query);
        if webbrowser::open(&url).is_err() {
            eprintln!("❌ 웹 브라우저를 여는 데 실패했습니다.");
        }
    } else {
        eprintln!("❌ 검색 엔진 지정이 올바르지 않습니다: {}\n", engine_code);
        println!("📑 사용 가능한 엔진 목록:");
        for (code, (_, name)) in &engines {
            println!("  🔹 {}: {}", code, name);
        }
    }
}
