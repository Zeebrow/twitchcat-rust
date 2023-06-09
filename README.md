# twitchcat-rs
keep up with the latest memes from all of your favorite TwitchTV chats

## run

```
set `TWITCH_TOKEN=oauth:xxx`
set `TWITCH_NAME=yourname`
set `TWITCH_CHANNEL=channel1,channel2,...`
cargo run
```

type 'help' for help, and follow the instructions

## TODO

### easy
> load config from yaml file
> respond to commands

### harder
> make the bot truly threaded
> > one thread per channel joined
> integrate `helix` api for better data, such as getting colors of twitch streamers instead of setting yourself
> control via telnet

### super duper hard
> control via REST api
> integrate with TUI (from the modded [pet store example](https://github.com/zeebrow-fluff/rust-tui))
> > dynamically add/remove watched channels
> > one tab per watched chat channel


