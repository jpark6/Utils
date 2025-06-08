# 🛠️ Utils
> script files

## Files
### python

|name|desc|
|-|-|
|getSlackChannelDMList.py|print slack channel and DM List(./token.txt file is needed: slack token "xoxp-...")|
|openSlackChannel.py|load list from slack_list.yaml and open select channel or DM(./token.txt file is needed: slack token "xoxp-...")|
|saveSlackChannelDMList.py|save slack channel and DM list to slack_list.yaml|
|webSearch.py|search web|

## Usage
### python
#### openSlackChannel.py
- save token to token.txt
- get token in slack app page
- save to `./token.txt` (same dir to py file)
  ```txt
  xoxp-...
  ```
- run `getSlackChannelDMList.py`
  ```shell
  $ python openSlackChannel.py
  ```

#### webSearch.py
- save engine list to `engines.yaml`
  ```yaml
  Google:
    Url: https://www.google.com/search?q=%s
    Keyword: g

  Naver:
    Url: https://search.naver.com/search.naver?query=%s
    Keyword: n

  NaverMap:
    Url: https://map.naver.com/v5/search/%s
    Keyword: nm
  ...
  ```
- run `webSearch.py`
  ```shell
  $ python webSearch.py g macbook # search macbook to google 
  $ python webSearch.py nm 인천공항 # search 인천공항 to Naver Map 
  ```
- My .zshrc save alias `s`
  ```shell
  alias s="/path/to/python/webSearch.py"
  ```
  ```shell
  $ s g macbook # search macbook to google
  $ s nm 인천공항 # search 인천공항 to Naver Map 
  ```
