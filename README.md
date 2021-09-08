# leetcode debugger

> Today, I wanted to debug my code that i submitted on leetcode... after a few tries noticed that I am repeating, create a new file, add those initial includes copy paste leetcode code then in main function pass the input...
>
> Can't computers help automate repeatable tasks more easily :)

[WIP] GUI

## Initial plan

### Phase 1

* Use [orbtk](https://lib.rs/crates/orbtk), that also provides some experience with the GUI library of RedoxOS
* A text input, for pasting code from leetcode
* After providing input, it should output a .cpp file, that takes the test case as CLI args (simple use case only)

> Stop here for now

### Phase 2 (Complete GUI)

* Input fields (with different types maybe, initially int,string,vector of those), for providing test case input
* Pass inputs using CLI args
* Give two options... open gdb (The main focus, stop here for now), open .cpp file in VS Code (for better visual debugging)

