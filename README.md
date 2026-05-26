# calculator -- a platform agnostic calculation utility

Calculator is a simple command line utility for (respectively) adding, subtracting, multiplying and dividing integer values.

It delivers top-of-the-line performance through the use of cutting edge methods of compile-time optimization and will be available as a library for use in your project, too, after some further improvements.

# Building

```
git clone https://github.com/Grott3/calculator.git
cd calculator
cargo build --release
```

# Usage

```
>> calculator --help

  ABOUT:
      'calculator' is a simple, platform-agnostic command line calculator that relies on compile-time optimization to deliver fast results.
  
  USAGE:
      > calculator <expression>
  
  EXPRESSIONS:
      <number1><operation><number2>
  
      + => add
      - => subtract
      ° => multiply
      / => divide
  
      EXAMPLES:
          1 ° 2
          4 + 7
          -15 / 12
          11 - 0
```
