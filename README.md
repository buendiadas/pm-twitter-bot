# pm-twitter-bot
Scans Helena Open prediction markets and
post periodically a message on Twitter with the outcome probabilities


## How to run it
1. Edit ```$HOME/.crypto-bot.conf``` and put values your want in your config file
2. Issue ```cargo run```

## Config File
```
{
  "consumer_key": "<consumer-key>",
  "consumer_secret": "<consumer-secret>",
  "access_key": "<access-key>",
  "access_secret": "<access-secret>"
  "cron_expression": <cron_expression>
}
```

