
from slack_sdk import WebClient
from slack_sdk.errors import SlackApiError

# token.txt에서 토큰 읽어옴.
# token은 텍스트만 저장 (""없이 xoxp-...)
def load_token(filename="token.txt"):
    with open(filename, "r", encoding="utf-8") as f:
        return f.read().strip()

SLACK_TOKEN = load_token()
client = WebClient(token=SLACK_TOKEN)

def list_channels():
    try:
        result = client.conversations_list(types="public_channel,private_channel") 
        print("Channels:")
        for channel in result["channels"] or []:
            print(f"- {channel['name']} (ID: {channel['id']})")
    except SlackApiError as e:
        print(f"Error fetching channels: {e.response['error']}")

def list_dms():
    try:
        # DM 목록 가져오기
        result = client.conversations_list(types="im")
        print("Direct Messages:")
        for dm in result["channels"] or []:
            user_id = dm.get("user")
            if user_id:
                try:
                    # 사용자 이름 가져오기
                    user_info = client.users_info(user=user_id)
                    username = user_info["user"]["name"]
                except SlackApiError:
                    username = "(알 수 없음)"
                print(f"- DM ID: {dm['id']}, User ID: {user_id}, Username: {username}")
            else:
                print(f"- DM ID: {dm['id']} (user ID 없음)")
    except SlackApiError as e:
        print(f"DM 정보를 가져오는 중 오류 발생: {e.response['error']}")
if __name__ == "__main__":
    list_channels()
    print()
    list_dms()
