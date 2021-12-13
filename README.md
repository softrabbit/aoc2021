# aoc2021
[Advent of Code 2021](https://adventofcode.com/2021)

Last year I maybe bit off more than I could chew in using a different language each day. So this time I'm going with only one, Rust. Not that I've ever used it before, but I hear good things about it. 

I'll try to chronicle what I touch on each day, or at least leave notes for myself. 

There might be better ways to do the things I do, but this code is what I arrive at after Google and Stckoverflow. Error checking is more or less nonexistant, something that might be a thing to fix later on.

## [Day 1](https://github.com/softrabbit/aoc2021/tree/main/01)
Lots of unwrapping, and some io::stdin.

## [Day 2](https://github.com/softrabbit/aoc2021/tree/main/02)
Conditional compilation, might be useful one day.

## [Day 3](https://github.com/softrabbit/aoc2021/tree/main/03)
Looks like the difference between iter() and into_iter() is important. Also found out that some of the error messages are less than helpful (or, in other words, way over my head). (Most are pretty nice, though.)

## [Day 4](https://github.com/softrabbit/aoc2021/tree/main/04)
Tuples seem useful, and I'm glad I didn't use the first overcomplicated contraption I found on Stackoverflow when looking for a way to see if the end of an iterator is reached. `peekable()` is the magic word. 

SO is particularly dangerous when looking for answers in a language as new and evolving as Rust, if you don't check the age of the answer you might end up with something deprecated, removed or just not a good solution... that takes a bit of getting used to.
 
## [Day 5](https://github.com/softrabbit/aoc2021/tree/main/05)

## [Day 6](https://github.com/softrabbit/aoc2021/tree/main/06)
Woohoo, got in the top 10K for once. Not going to start rising early in the 
morning for AoC, though. Checked out command line arguments, looks close enough to other languages to not be a big deal.

## [Day 7](https://github.com/softrabbit/aoc2021/tree/main/07)
Nothing new Rust-wise in todays solution, partly because I didn't have the time to put in any extra effort. Might come back to refine later. Not that even the most basic practice is wasted at this point, maybe I'll get something to compile on the first try some day.

## [Day 8](https://github.com/softrabbit/aoc2021/tree/main/08)

## [Day 9](https://github.com/softrabbit/aoc2021/tree/main/09)

## [Day 10](https://github.com/softrabbit/aoc2021/tree/main/10)

## [Day 11](https://github.com/softrabbit/aoc2021/tree/main/11)
Wow. Not a hard algorithm at all, but a lot of time went into the intricacies of looping and modifying a vector of vectors. Especially when there was a need to loop from possibly -1 to 1, which seems like a thing you'd like to now and then but seems to require jumping through quite a few hoops.

## [Day 12](https://github.com/softrabbit/aoc2021/tree/main/12)
This proved to be a tough one. Couldn't get this on the day, so I'll return to it (in January maybe?). To quote some guy on the Internet: "Modeling graph-like structures in Rust is not a simple problem." 

## [Day 13](https://github.com/softrabbit/aoc2021/tree/main/13)
Code reuse, I took my 2D vector code from day 5 to speed my progress. Tried `zip` for the first time. Couldn't quite figure out how to use the experimental td::iter::zip` function so I went with the "method on an iterator" strategy.
