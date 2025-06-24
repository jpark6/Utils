import sys
import os
import webbrowser
import urllib.parse
import yaml

def load_engines(yaml_path=os.path.join(os.path.expanduser("~"), ".config", "files", "engines.yaml")):
    with open(yaml_path, "r", encoding="utf-8") as f:
        data = yaml.safe_load(f)

    # keyword → (url_template, name)
    keyword_map = {}
    for name, props in data.items():
        url = props.get("Url")
        keyword = props.get("Keyword")
        if url and keyword:
            keyword_map[keyword] = (url, name)

    return keyword_map

def main():
    if len(sys.argv) < 3:
        print("📘 사용법: s [검색엔진코드] [검색어]")
        return

    engine = sys.argv[1].lower()
    query = " ".join(sys.argv[2:])
    encoded_query = urllib.parse.quote(query)

    try:

        engines = load_engines()
    except Exception as e:
        print(f"❌ engines.yaml 로딩 실패: {e}")
        return

    if engine in engines:
        url_template, name = engines[engine]
        url = url_template.replace("%s", encoded_query)
        print(f"🔍 검색 중: [{name}] '{query}'")
        webbrowser.open(url)
    else:
        print(f"❌ 검색 엔진 지정이 올바르지 않습니다: {engine}\n")
        print("📑 사용 가능한 엔진 목록:")
        for code, (_, name) in engines.items():
            print(f"  🔹 {code}: {name}")

if __name__ == "__main__":
    main()

