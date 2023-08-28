# <p align="center">Hacker News Alert</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/hackernews-alert">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-the-hackernews-alert-app), and you will reveice HackerNews post alerts each hour based on a keyword you set. The below screenshot shows alert messages of the keyword "GPT".

<img width="658" alt="image" src="https://user-images.githubusercontent.com/45785633/227419393-d7a438f1-51c9-42bc-bb9a-bac1cd3e5581.png">

## Deploy your own HN alert bot in 2 simple steps

1. Load [the Hacker News alert template](https://flows.network/flow/createByTemplate/hackernews-alert)
2. Configure and authorize your Slack access

### 0 Prerequisites

You will need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Load the Hacker News Alert template

[**Just click here**](https://flows.network/flow/createByTemplate/hackernews-alert)

Set the `KEYWORD` variable. It is the word you want to monitor on Hacker News and get alert of. For each bot you can only set one keyword. For more keywords, you can set more a bot for each of them.
![image](https://github.com/flows-network/hackernews-alert/assets/37167103/6faed67d-e596-48fb-820c-eff56709859e)

Click on the **Create and Deploy** button.

### 2 Configure the bot to access your Slack

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

Next, you will tell the bot which Slack channel you want your alert messages to be sent to.

* `slack_channel`
Slack organization of the Slack channel where you want to deploy the bot. Case sensitive.

* `slack_workspace`
The Slack channel where you want to deploy the bot. Case sensitive.

![image](https://github.com/flows-network/hackernews-alert/assets/37167103/85fb6038-7632-4aa0-bf74-3a8f3ab79fd2)


Next, let's grant flows.network to access the Slack channel you just input.

Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network` bot on your workspace. This workspace is the one you entered into the `slack_workspace` above.

> If you have authenticated the workspace before,you can see the purple Connect button turns gray Connected button. Just ingore this step and click Check button.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the Hacker News alert App goes live. You will get a salck message at the 50th minute of every hour !

![image](https://github.com/flows-network/hackernews-alert/assets/37167103/f0a2928f-a918-443b-a58f-aae348710490)



