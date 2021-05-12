# Tracing Prism

> Readable async logs.

This is a simple tool that allows you to split a log file in several columns based on a text filter or on log level. It is specifically aimed at tracing, but can be used on any text file.

The idea is that in async programs, or integration tests, it's hard to follow what is going on in the log because you get interleaved output from several concurrent tasks. _tracing-futures_ allows you to instrument executors and/or futures that you spawn so you can tag tasks with a specific string.

tracing-prism is a little wasm webgui that allows you to split that log into several columns and then see the flow of your program through the different concurrent components.

It's available online at github pages: https://najamelan.github.io/tracing_prism/.
When using the json format from tracing, only a minimal message is shown. The time/date is shown on hover in the bottom left corner. Other fields can be inspected by clicking on the log entries.

![Screenshot](/doc/screenshot.png)

_The screenshot above is generated from a sample application that spawns 3 different async tasks. Normally all the statements would be interleaved. You can check out the original [log file here](/doc/log.json) if you want a live demo. Just upload it to https://najamelan.github.io/tracing_prism/ and fill out the text filters near the top of the table._

# Limitations

There currently is no performance optimization. That is if you load a big log file, the whole file is just processed at once in your browser. That can get pretty slow if you load several megabytes (or more) of text at once. It's very nice for reading output from tests when debugging. It's not meant to replace production tools like elastic search.

# Privacy

This is entirely client based. No data is transmitted anywhere. The entire application runs in your browser.


## Compilation and running locally

1. Make sure you have Rust and wasm-pack installed.
2. `git clone https://github.com/najamelan/tracing_prism`
3. `cd tracing_prism`
4. `wasm-pack build --target web`
5. Make sure your browser allows loading scripts on `file://` urls.
6. Open `index.html`

