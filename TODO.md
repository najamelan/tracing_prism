# TODO

- re-enable plain text
- cargo integration
- performance

- graphical design!!! Make something beautiful.
  - link to source code
  - arrays and objects in field values are still using quotes.

- when filters change, if the user is scrolled down, it might liberate space above. However we don't want any currently visible text to move up out of view. Will be easier when we associate the view with a time line by showing time separately.
  https://stackoverflow.com/questions/9834143/jquery-keep-window-from-changing-scroll-position-while-prepending-items-to-a-l
  http://jsfiddle.net/Wexcode/tfszaocz/

- perf with big log files -> framerate drops dramatically with big log files.
  - Probably should use a web worker to offload the work (see wasm-thread)
  - only process parts of the text that are visible and handle the rest on demand on scroll
  - currently we are naively recalculating everything and re-manipulating everything if anything might have changed.
    eg. when removing a column, all other columns will run all filters again and manipulate the dom again even if
    nothing has to change in their view. Surely this isn't the most performant approach.

- json input
  - hover time
  - let the user customize which fields

  {
    "timestamp": "Feb 08 20:06:03.705",
    "level": "INFO",
    "target": "async_std::task::builder",
    "fields": {
      "message": "spawn",
      "log.target": "async_std::task::builder",
      "log.module_path": "async_std::task::builder",
      "log.file": "/home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/async-std-1.5.0/src/task/builder.rs",
      "log.line": 41
    }
  }


