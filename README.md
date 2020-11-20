# Tracing Prism

> Readable async logs.

This is a simple tool, still in development, that allows you to split a log file in several columns based on a text filter or on log level. It is specifically aimed at tracing, but can be used on any text file.

The idea is that in async programs, or integration tests, it's hard to follow what is going on in the log because you get interleaved output from several concurrent tasks. _tracing-futures_ allows you to instrument executors and/or futures that you spawn so you can tag tasks with a specific string.

tracing-prism is a little wasm webgui that allows you to split that log into several columns and then see the flow of your program through the different concurrent components.

It's available online at github pages: https://najamelan.github.io/tracing_prism/

It's in an early stage of development. The plan is to support json input, which will allow a better visualisation, like not repeating date/time in each column, and letting the user define which fields to show/show on hover/show on click.

# Privacy

This is entirely client based. No data is transmitted anywhere. The entire application runs in your browser.


# Contributing

For the moment, as the project is still in development and will still change a lot, it's probably not that useful to take contributions. However if you have ideas/questions, feel free to open an issue to discuss.

If you find this useful and want me to speed up development, feel free to star the repository so I know someone cares. In any case, I am developping this because I need to be able to read my logs properly, so I will continue the project.


## Compilation

1. Make sure you have Rust and wasm-pack installed.
2. `git clone https://github.com/najamelan/tracing_prism`
3. `cd tracing_prism`
4. `wasm-pack build --target web`
5. Make sure your browser allows loading scripts on `file://` urls.
6. Open `index.html`

