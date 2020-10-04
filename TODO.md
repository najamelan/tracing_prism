# TODO

- perf with big log files -> framerate drops dramatically with big log files. Probably should use a web worker to offload the work.
- json input
  - hover time
  - click to get more details
  - let the user customize which fields

- rewrite logic for cross column hiding. Code is quite complicated right now, and we should probably just keep counters for each line keeping track of how many columns show it, and when they reach zero, update all colummns.
