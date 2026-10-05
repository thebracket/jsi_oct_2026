# Wrap Up

So that's a cleaned up version of my first encounter with Rust:

* Woah, this syntax is a bit strange. Everything's backwards!
* I can't send parameters to functions and use them again???
* I have to double-declare `&` everywhere for references?
* My god, it's `mut` everywhere...

It didn't take me very long to realize that the pain was worth it, because I'd eliminated:

* *Use after free*, one of the most common bugs out there.
* *Use after move*, which only seemed to hit me when I remembered what `std::move` does.
* Changing a function signature without checking all the usages.

So bear this in mind as we move into the whirlwind tour of Rust from a C++ perspective. Rust was
born out of Mozilla employees hitting the same bugs over and over again, and not wanting to keep
digging the same holes.
