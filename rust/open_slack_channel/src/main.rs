use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use skim::prelude::*;
use open;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

#[derive(Debug, Deserialize)]
struct Channel {
    id: String,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Dm {
    dm_id: String,
    name: String,
    url: String,
    user_id: String,
    username: String,
}

#[derive(Debug, Deserialize)]
struct SlackList {
    channels: Vec<Channel>,
    dms: Vec<Dm>,
}

// 선택지와 실제 URL을 연결하기 위한 구조
struct Entry {
    display: String,
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ctrl+C 처리 설정
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n[!] Ctrl+C 입력됨 - 프로그램 종료");
        r.store(false, Ordering::SeqCst);
    })?;

    // 홈 디렉토리에서 파일 읽기
    let home_dir = dirs::home_dir().ok_or("홈 디렉토리를 찾을 수 없습니다")?;
    let yaml_path: PathBuf = home_dir.join(".config/files/slack_list.yaml");

    let yaml_str = fs::read_to_string(&yaml_path)?;
    let data: SlackList = serde_yaml::from_str(&yaml_str)?;

    // display -> url 매핑을 위한 Entry 리스트 생성
    let mut entries: Vec<Entry> = vec![];

    for ch in &data.channels {
        entries.push(Entry {
            display: format!("[Channel] {}", ch.name),
            url: ch.url.clone(),
        });
    }

    for dm in &data.dms {
        entries.push(Entry {
            display: format!("[DM] {} {}", dm.name, dm.username),
            url: dm.url.clone(),
        });
    }

    // skim에 입력할 문자열 구성
    let input = entries
        .iter()
        .map(|e| e.display.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .prompt(Some("검색 > "))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(std::io::Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();
    
    // Ctrl+C 누른 경우 강제 종료
    if !running.load(Ordering::SeqCst) {
        std::process::exit(130); // 130: 표준 Ctrl+C 종료 코드
    }

    if let Some(item) = selected_items.get(0) {
        let selected_display = item.output();

        // display 문자열을 통해 URL 찾기
        if let Some(entry) = entries.iter().find(|e| e.display == selected_display) {
            println!("열기: {}", entry.url);
            open::that(&entry.url)?;
        }
    } else {
        println!("선택이 취소되었습니다.");
    }

    Ok(())
}
