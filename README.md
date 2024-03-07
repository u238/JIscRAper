# JIscRAper
A handful utility to extract statistics from JIRA.

## Setup
For making the script work you need to [generate a JIRA API](https://id.atlassian.com/manage-profile/security/api-tokens) token and save it inside a configuration file.
By default JIscRAper will look at `~/.config/jiscraper.toml`
```toml
[jira]
host = "https://<your-jira-space>.atlassian.net/"
email = "<email-address>"
# Create an API Token here: https://id.atlassian.com/manage-profile/security/api-tokens
api_token = "<api-token>"
```

## Build Image
```shell
podman build -t jiscraper .
```

## Run
### innovation time
In order to check how much innovation time was invested by you run
```shell
podman run -it localhost/jiscraper -v ~/.config/jiscraper.toml:/root/.config/jiscraper.toml:z
```

For making statistics over all users run
```shell
podman run -it -v ~/.config/jiscraper.toml:/root/.config/jiscraper.toml:z localhost/jiscraper --all-authors
```

## Hints
You can add an alias in your `.bashrc`/`.zshrc` file
```shell
alias jiscraper="podman run -it -v ~/.config/jiscraper.toml:/root/.config/jiscraper.toml:z localhost/jiscraper"
```
