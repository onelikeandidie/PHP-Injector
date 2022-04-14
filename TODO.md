There's some stuff I would like to add to make this a little faster

- Incremental builds >
  Keep the original source mappings on "watch" so that the compilation is 
  faster.
- Raw Injections >
  Currently injections can only inject functions into the file. This is good
  since it keeps the files clean and debugging can be done on the injection
  files but some developers probably wouldn't like to keep their injections
  dir accessible on their server. Raw injections would inject the code of the
  mixin directly into the compile target files.
- Compiler Refactor >
  The compiler file is a little messy to say the least. I would like to move
  some procedures into functions so that there is more flexibility with the
  code and the library overall
- PHP Integration >
  I know PHP can start child processes or run bash commands, a way to compile
  these or refresh these injections directly from a PHP function could prove
  to be useful for developers.
- Rust library/crate >
  Enough documentation could make it easy to create a library other devs could
  use or even a tool that could be installed with "cargo install". Food for
  thought.
- Efficient Source Map >
  I don't like how it maps all the sources even if the mixins are only on a 
  couple of files. I want to waste processing time on that.