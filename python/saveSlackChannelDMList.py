import yaml
from slack_sdk import WebClient
from slack_sdk.errors import SlackApiError

# token.txt에서 토큰 읽어옴.
# token은 텍스트만 저장 (""없이 xoxp-...)
def load_token(filename="token.txt"):
    with open(filename, "r", encoding="utf-8") as f:
        return f.read().strip()

SLACK_TOKEN = load_token()
client = WebClient(token=SLACK_TOKEN)

# 워크스페이스 ID 가져오기 (team.info 호출)
def get_workspace_id():
    try:
        response = client.team_info()
        return response["team"]["id"]
    except SlackApiError as e:
        print(f"워크스페이스 ID 가져오기 실패: {e.response['error']}")
        return ""

def fetch_channels(workspace_id):
    try:
        response = client.conversations_list(types="public_channel,private_channel")
        channels = response["channels"]
        return [{
            "name": c["name"],
            "id": c["id"],
            "url": f"slack://channel?team={workspace_id}&id={c['id']}"
        } for c in channels]
    except SlackApiError as e:
        print(f"채널 가져오기 실패: {e.response['error']}")
        return []

def fetch_dms(workspace_id):
    try:
        response = client.conversations_list(types="im")
        dms = []
        for dm in response["channels"]:
            user_id = dm.get("user")
            if user_id:
                try:
                    user_info = client.users_info(user=user_id)
                    profile = user_info["user"]["profile"]
                    real_name = profile.get("real_name", "")
                    name = user_info["user"]["name"]
                    display_name = profile.get("display_name", "")
                    name_to_show = real_name or display_name or user_id

                    dms.append({
                        "username": name_to_show,
                        "name": name,
                        "user_id": user_id,
                        "dm_id": dm["id"],
                        "url": f"slack://channel?team={workspace_id}&id={dm['id']}"
                    })
                except SlackApiError:
                    continue
        return dms
    except SlackApiError as e:
        print(f"DM 가져오기 실패: {e.response['error']}")
        return []

def save_to_yaml(channels, dms, filename="slack_list.yaml"):
    data = {
        "channels": channels,
        "dms": dms
    }
    with open(filename, "w", encoding="utf-8") as f:
        yaml.dump(data, f, allow_unicode=True)
    print(f"✅ YAML 파일 저장 완료: {filename}")

if __name__ == "__main__":
    workspace_id = get_workspace_id()
    if workspace_id:
        channels = fetch_channels(workspace_id)
        dms = fetch_dms(workspace_id)
        save_to_yaml(channels, dms)
