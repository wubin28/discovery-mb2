# `micro::bit v2 Embedded Discovery Book`

> Discover the world of microcontrollers through [Rust]!

[Rust]: https://www.rust-lang.org/

This book is an introductory course on microcontroller-based embedded systems that uses Rust as the
teaching language rather than the usual C/C++.

## Scope

The following topics will be covered (eventually, I hope):

- How to write, build, flash and debug an "embedded" (Rust) program.

- Functionality ("peripherals") commonly found in microcontrollers: Digital input and output, Pulse
  Width Modulation (PWM), Analog to Digital Converters (ADC), common communication protocols like
  Serial, I2C and SPI, etc.

- Multitasking concepts: cooperative vs preemptive multitasking, interrupts, schedulers, etc.

- Control systems concepts: sensors, calibration, digital filters, actuators, open loop control,
  closed loop control, etc.

## Approach

- Beginner friendly. No previous experience with microcontrollers or embedded systems is required.

- Hands on. Plenty of exercises to put the theory into practice. *You* will be doing most of the
  work here.

- Tool centered. We'll make plenty use of tooling to ease development. "Real" debugging, with GDB,
  and logging will be introduced early on. Using LEDs as a debugging mechanism has no place here.

## Non-goals

What's out of scope for this book:

- Teaching Rust. There's plenty of material on that topic already. We'll focus on microcontrollers
  and embedded systems.

- Being a comprehensive text about electric circuit theory or electronics. We'll just cover the
  minimum required to understand how some devices work.

- Covering details such as linker scripts and the boot process. For example, we'll use existing tools
  to help get your code onto your board, but not go into detail about how those tools work.

Also I don't intend to port this material to other development boards; this book will make exclusive
use of the micro:bit development board.

## Reporting problems

The source of this book is in [this repository]. If you encounter any typo or problem with the code
report it on the [issue tracker].

[this repository]: https://github.com/rust-embedded/discovery-mb2
[issue tracker]: https://github.com/rust-embedded/discovery-mb2/issues

## Other embedded Rust resources

This Discovery book is just one of several embedded Rust resources provided by the
[Embedded Working Group]. The full selection can be found at [The Embedded Rust Bookshelf]. This
includes the list of [Frequently Asked Questions].

[Embedded Working Group]: https://github.com/rust-embedded/wg
[The Embedded Rust Bookshelf]: https://docs.rust-embedded.org
[Frequently Asked Questions]: https://docs.rust-embedded.org/faq.html
