# PEAT Track Editor
This editor is used to edit `.peat` files (Pico Embedded Audio Track)

The `.peat` format is a custom format designed for use in this tracker and ease of raw export and for the Raspberry Pi Pico RP2040 microprocessor.

An example `.peat` file consists of the following data
(comments are not allowed, but are shown for explanation)
```peat
// The first 2 lines are the header
// The first line must be a version identifier
PEAT 1
// The second line must be the Note Per Minute Divisor
// The pico-pet project renders approximately 1256 notes per minute
// To compress audio, this divisor reduces the NPM
// In this scenario, the divisor is 2
// Therefore, the actual Notes Per Minute will be 628
NPMD 2
// The header ends with a blank line

// Now we reach the track itself
C4 // a note begins
. // sustain indicator
_ // rest indicator
C4 // a new C4 note begins
C4 // a repeated note will be rendered as a sustained note
_ // rests are useful for creating a staccato effect
C#4 // a new note begins
Db4 // This note would sustain as it is identical to C#4
D4 // a new note begins immediately with no staccato
```

- All notes are tuned to a baseline 440hz frequency.
- Notes are represented as square waves.
- The available notes are C4 through C7 inclusive.
