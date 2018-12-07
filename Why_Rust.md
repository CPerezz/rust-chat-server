# What's Rust?

- Rust is a systems programming language with a **focus on safety, especially safe concurrency**, supporting both **functional and imperative paradigms**. Rust is syntactically similar to C++, but its designers intend it to provide **better memory safety while still maintaining performance**.

- Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and many others. Its designers have refined the language through the experiences of writing the Rust compiler. **The compiler is free and open-source software, dual-licensed under the MIT License and Apache License 2.0**.



## Why rust??

![Imgur](https://i.imgur.com/FCfYg5g.png)

- Real control of machine and it's resources like C or C++. Limited resources and get as much as possible of them.

- Any small mistake can destroy the whole app on C or C++ and it's compilers doesn't help you that much. (Memory leaks, memory pointers pointing garbage, etc..).

Some Rust properties:

- Memory safety
	- The system is designed to be memory safe, and **it does not permit null pointers, dangling pointers, or data races in safe code**. Data values can only be initialized through a fixed set of forms, all of which require their inputs to be already initialized. 
	- To replicate the function in other languages of pointers being either valid or NULL, such as in linked list or binary tree data structures, **the Rust core library provides an option type, which can be used to test if a pointer has Some value or None.** Rust also introduces **added syntax to manage lifetimes, and the compiler reasons about these through its borrow checker**.

- Memory management
	- **Rust does not use an automated garbage collection system** like those used by Go, Java, or the .NET Framework. Instead, **memory and other resources are managed through resource acquisition is initialization (RAII)**, with optional reference counting. Rust provides deterministic management of resources, with very low overhead. Rust also favors stack allocation of values and does not perform implicit boxing.
	- **There is also a concept of references (using the & symbol)**, which do not involve run-time reference counting. **The safety of using such pointers is verified at compile time by the borrow checker**, preventing dangling pointers and other forms of undefined behavior.

- Ownership
	- Rust has an **ownership system where all values have a unique owner** where the scope of the value is the same as the scope of the owner. 
	- **Values can be passed by immutable reference using &T, by mutable reference using &mut T or by value using T**. (At all times, there can either be multiple immutable references or one mutable reference). 
	- The Rust compiler enforces these rules at compile time and also checks that all references are valid.

- Types and polymorphism
	- The type system **supports a mechanism similar to type classes, called "traits", inspired directly by the Haskell language**. This is a facility for ad hoc polymorphism, achieved by adding constraints to type variable declarations. Other features from Haskell, such as higher-kinded polymorphism, are not yet supported.

	- **Rust features type inference, for variables declared with the let keyword**. Such variables do not require a value to be initially assigned to determine their type. A compile time error results if any branch of code fails to assign a value to the variable. **Variables assigned multiple times must be marked with the mut keyword**.


To sumarize, you should love Rust because supports a multi-paradigm programming style, performs as well as C or C++ or even better, it's totally safe, and can be written at a pretty high level.

It also has Crates.io!!!! A library collection repository like NPM where you can find pretty much everyting you need.

![](https://rustycrate.ru/assets/2017-12-17-crates.io/teaser-7aa6533557df0c6793313f6a61dbab22af41d06d18cce74e3d65d4e671aec501.png)

## Does Rust Really perform that well? Is really that easy and cool to writte?
What can now happen..

<blockquote class="twitter-tweet" data-lang="ca"><p lang="en" dir="ltr">I love Rust.   But I feel that convincing people of its value is like convincing people to go vegan.   Everyone nods and respects you, but don’t really care and keep eating sausages.</p>&mdash; Miguel de Icaza (@migueldeicaza) <a href="https://twitter.com/migueldeicaza/status/1063575250485555208?ref_src=twsrc%5Etfw">16 de novembre de 2018</a></blockquote>


#### Let's make it visual with three different examples:
## Super fast performing and high level syntax.
>Soure: Niko Matsakis, Researcher at Mozilla and Core dev of Rust the Programming Language.


![Imgur](https://i.imgur.com/SSod6C6.png)

![Imgur](https://i.imgur.com/wGqgfa7.png)

![Imgur](https://i.imgur.com/S4HyVoJ.png)

##  High performing with 3rd party library while simple.
![Imgur](https://i.imgur.com/5uA86YB.png)

![Imgur](https://i.imgur.com/fdIkFWb.png)

## Code safety (Specially on concurrency).
![Imgur](https://i.imgur.com/oDkHjwH.png)

![Imgur](https://i.imgur.com/48rLMMJ.png)

## One of the greatests instructions I've ever seen: `match`.

I faced this a week ago:
>Given a year, report if it is a leap year.
>The tricky thing here is that a leap year in the Gregorian calendar occurs:

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```
What would you probably do facing this?

```rust
pub fn is_leap_year(year: u64) -> bool {
    println!("Received year: {:?}", year);
    if year % 4 != 0 {
        return false
    }
    if year % 100 == 0 && year % 400 != 0{
        return false
    }else if year % 100 == 0 && year % 400 == 0 {
        return true
    }
    true
}
```

The power of match will be something like this:
```rust
pub fn is_leap_year(year: i64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
```
Clean, Simple, and powerfull.


# How to become a Rustacean?

- https://www.rust-lang.org/en-US/
	- https://doc.rust-lang.org/rust-by-example/
	- https://doc.rust-lang.org/book/
- ## References

[Standard Library API Reference](https://doc.rust-lang.org/std/). Documentation for the standard library.

[docs.rs](https://docs.rs/). Documentation for all crates published to  [crates.io](https://crates.io/).

[The Rust Reference](https://doc.rust-lang.org/reference). While Rust does not have a specification, the reference tries to describe its working in detail. It tends to be out of date.

[Syntax Index](https://doc.rust-lang.org/book/syntax-index.html). This appendix from The Book contains examples of all syntax in Rust cross-referenced with the section of The Book that describes it.

[The Cargo Guide](http://doc.crates.io/guide.html). The documentation for Cargo, Rust’s package manager.

[Compiler Error Index](https://doc.rust-lang.org/error-index.html). Extended explanations of the errors produced by the Rust compiler.

[Release Notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md). A recording of changes made during each release.

[Platform Support](https://forge.rust-lang.org/platform-support.html). List of platforms in each support tier.
