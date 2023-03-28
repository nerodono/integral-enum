# integral-enum
Procedural macro for easy integer-like enums definition

## Usage

```rust
use integral_enum::integral_enum;

// Discriminant will be automatically determined based on the variants count (from u8 to u64).
#[integral_enum]
// After macro expansion repr will be added automatically
// #[repr(u8)]
pub enum Animal {
    Cat,
    Dog,
    Human,
}

// But discriminant type can be defined manually.
#[integral_enum(u64)]
// Same here
// #[repr(u64)]
pub enum Person {
    Nero,
    // Explicit discriminants also supported
    NotNero = 102400,
    PossiblyNero, // = 102401
    AlmostNero,
}

// integral_enum derives the following traits automatically: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord.
// And additionally creates implementation for the TryFrom trait:
assert_eq!(Animal::try_from(0), Ok(Animal::Cat));
assert_eq!(Animal::try_from(1), Ok(Animal::Dog));
assert_eq!(Animal::try_from(2), Ok(Animal::Human));

assert_eq!(Person::try_from(0), Ok(Person::Nero));
assert_eq!(Person::try_from(102400), Ok(Person::NotNero));
assert_eq1(Person::try_from(102401), Ok(Person::PossiblyNero));
```
