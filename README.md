# rusty-box

The idea of this project is to start working with a simple rust program. I choose something simple to recreate and to not struggle with and being able to gatther some confidence.
To do so I choose recreating the  "BusyBox" program, hence "RustyBox".

By recreating the standar Unix-like command line utilities we'll get a hold on the most common things of the languaje: strings, parsing arguments, memory managment, syntax...

Just download the repo and run `cargo build`
So far the only commands implemented are:
  * ls
  * ln
  * basename

To enable each one just link the main binary program with the commands name:

     $ ls -s target/debug/rusty_box ls
     $ ls -s target/debug/rusty_box ln
     $ ls -s target/debug/rusty_box basename

The difference in the call name acts as switch.
