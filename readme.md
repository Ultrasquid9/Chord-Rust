# Chord (Rust Implementation)

Chord is a programming language designed to be simple, readable, and reasonably performant.

Currently, Chord is written in Rust, a language which it was heavily inspired by. However, once Chord is able to run, the plan is to write a new Chord compiler in Chord itself, which will supersede this one.

# Why Chord? 

Chord was designed with the following target audiences in mind:
- You want safety guarantees and reasonable performance, but do not need or care about low-level systems programming.
- You like Rust, but feel that there are some areas where it could use some improvement.

Once Chord is further along, there are plans for native Rust interop, which will allow you to painlessly incorporate Chord code into Rust projects (and vice versa).

# Examples

Here is a minimal "hello world" program in Chord:
```
funct main() => print("Hello, world!")
```

And here is a fibonacci sequence calculator written in Chord:
```
funct main() {
	let target = 10
	let prev = 0
	let current = 1

	let fib = fib(target, prev, current)
	print_fib(target, fib)
}

# Calculates the fibonacci number for the target input
funct fib(target: u32, prev: u32, current: u32) -> u32 => if target == 0 {
	current
} else {
	fib(target - 1, current, prev + current)
}

# Prints out given fibonacci number, with proper formatting applied
funct print_fib(target: u32, fib: u32) {
	let place = match target {
		1 => "st"
		2 => "nd"
		3 => "rd"
		_ => "th"
	}

	print("The {target}{place} Fibonaci number is: {fib}")
} 
```
