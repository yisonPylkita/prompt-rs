# prompt-rs
Shell prompt generator in Rust

### Why?
I want to see how long it takes for bash to run Rust app that generates shell
prompt compared to writing same prompt generator function in shell (I'll test on
ZSH)

### Roadmap
 - Write test scenario
 - Write Rust generator for test scenario
 - Write ZSH generator for test scenario

### Rules
 - Complete time of execution will be measured. So test shall look like this
```bash
# 1. Check start time with microseconds precision
# 2. Run generator and store its result as ENV string
# 3. Print execution time with microseconds precision
```

### How to run
```bash
echo "$(cargo run)"
```
