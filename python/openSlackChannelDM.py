import yaml
import webbrowser
import platform
from InquirerPy import inquirer

def load_slack_list(yaml_file="slack_list.yaml"):
    with open(yaml_file, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)

def build_choice_list(data):
    choices = []

    for ch in data.get("channels", []):
        label = f"[Channel] #{ch['name']}"
        choices.append({
            "name": label,
            "url": ch["url"]
        })

    for dm in data.get("dms", []):
        label = f"[DM] @{dm['username']}-{dm['name']}"
        choices.append({
            "name": label,
            "url": dm["url"]
        })

    return choices

def fuzzy_select_and_open(choices):
    selected = inquirer.fuzzy(
        message="📺 채널 or 💬 DM 검색:",
        choices=[item["name"] for item in choices],
        validate=lambda result: result in [item["name"] for item in choices]
    ).execute()

    selected_item = next(item for item in choices if item["name"] == selected)
    print(f"열기: {selected_item['name']}")
    webbrowser.open(selected_item["url"])

if __name__ == "__main__":
    os_name = platform.system()
    yaml_file = "D:\\Repos\\Utils\\python\\slack_list.yaml" if os_name=="Windows" else "/Users/jakepark/Repos/Utils/python/slack_list.yaml" 
    slack_data = load_slack_list(yaml_file)
    choice_list = build_choice_list(slack_data)
    if not choice_list:
        print("채널 또는 DM 항목이 없습니다.")
    else:
        fuzzy_select_and_open(choice_list)
