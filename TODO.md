# TODO

- graphical design!!! Make something beautiful.
  - link to source code

- when filters change, if the user is scrolled down, it might liberate space above. However we don't want any currently visible text to move up out of view. Will be easier when we associate the view with a time line by showing time separately.
  https://stackoverflow.com/questions/9834143/jquery-keep-window-from-changing-scroll-position-while-prepending-items-to-a-l
  http://jsfiddle.net/Wexcode/tfszaocz/

- perf with big log files -> framerate drops dramatically with big log files.
  - Probably should use a web worker to offload the work (see wasm-thread)
  - only process parts of the text that are visible and handle the rest on demand on scroll

- json input
  - hover time
  - click to get more details
  - let the user customize which fields

- rewrite logic for cross column hiding. Code is quite complicated right now, and we should probably just keep counters for each line keeping track of how many columns show it, and when they reach zero, update all colummns.


