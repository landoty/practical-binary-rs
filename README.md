# Practical Binary Analysis *In Rust*

My main interest(s) revolve around binary analysis and, since I've been working on learning Rust, this felt like a natural move. I'm also focusing my master's research around both, so it's a good starting point.

I've been meaning to work through some of the material in [Practical Binary Analysis](https://practicalbinaryanalysis.com/) by *Dennis Andriesse*, which provides a C-based binary "loader". While their loader uses the GNU *libbfd* API which powers much of the `binutils` suite, I decided to use the [object](https://crates.io/crates/object) crate to keep my implementation in pure Rust. 

## What it does (and will do)

Only the skeleton of the loader is currently implemented which takes an ELF executable as input and prints the section names contained in it.

#### Functionality to come
- ELF Header Parsing
- Symbol Names
- Section digest (size, contents, type, etc.)
- Basic Analyses (CFG construction, dependencies, etc.)
- Searching? 