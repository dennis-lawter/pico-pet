# PEAT File Format
**PEAT** stands for Plaintext Embedded Audio Track.

## Purpose
The purpose of this file is to encode a track played on a single channel with single notes played in sequence.
These files are able to encode audio in a very small format for embedded hardware.

## Driver
The audio driver supports 1,256 notes per minute (NPM).
Files may compress their notes significantly by using an integer note per minute divisor (NPMD).
NPMD must be between 1 and 255.

## Example File
For an example PEAT file, this is the classic Take Me Out To The Ball Game.
- The composition is in C Major.
- The time signature is in 3/4.
- With an NPMD of 2, the hardware plays 628 notes per minute.
- Each note, in this PEAT file, is representative of a 16th note.
- To encode a fully voiced quarter note with these settings, you could do: `C4 . . .`.
- Each note is purposefully ended with a 16th note rest.
- Ending notes with a single note of rest is useful when the song has repeated notes.
- Repeated notes will sustain; later in this song there are repeated notes.
- While those notes could have been separated with 16th note rests, I decided to place a rest after every note.
- This was not the only solution to the problem, feel free to experiment.

| File | Explanation |
| --- | --- |
| `PEAT 1` | *REQUIRED* Version identifier. |
| `NPMD 2` | *REQUIRED* NPMD signifier, must be 1-255. |
| `Take Me Out To The Ball Game` | *REQUIRED* Song title, only displayed in PEAT player. |
|  | *REQUIRED* Header ends with a blank line. |
| `C4  .   .   .   .   .   .   _` | Play a C4 for 7 notes, then rest for 1 note. |
| `C5  .   .   _` | Play a C5 for 3 notes, then rest for 1 note. |
| `A4  .   .   _` | The period is a "sustain" indicator. |
| `G4  .   .   _` | Without the rest, the notes would transition with no interruption. |
| `F4  .   .   _` | On the embedded hardware, there should be no interruption between these 3 F4 notes |
| `G4  .   .   .   .   .   .   .   .   .   .   _` | This G4 plays for an entire "measure" by lasting 15 notes + a rest |

## Important notes
The notes provided must lie between C4 and C7.

The voicing is always a square wave.

You may use the following to represent a C#4 note:
- `C#4`
- `Cs4`
- `Db4`

You may use `.` to "sustain" a note for an additional note.

The `_` note represents a rest.

Notes must be separated by at least one character whitespace(spaces, tabs, newlines).

There are no chords; the hardware can play at most one note at any given time.

Without a rest, there is no distinction between repeated notes.

Repeated notes are identical to a sustain.

That is to say, that these are all identical ways to represent the same wave format:
- `C4 C4 C4 C4`
- `C4 . . .`
- `C4 . C4 .`

# BEAT File Format
The purpose of a **PEAT** file is to compile into a **BEAT** (Binary Embedded Audio Track) file.

The first byte of a BEAT file is the NPMD, and the byte must be between `0x01` and `0xFF`.

Every byte after that represents the song.

A4 (440hz) has been chosen as the byte `0x80`.

Every half-step up from A4 is an increase of that base byte by 1.

Every half-step down from A4 is a decrease of that base byte by 1.

The byte `0x00` represents a rest.

There is no idea of "sustain", instead repeated notes are assumed to be a continued note.

The following is an example BEAT file, represented in hex, from the previous snippet of Take Me Out To The Ball Game.

```beat
02 77 77 77 77 77 77 77
00 83 83 83 00 80 80 80
00 7E 7E 7E 00 7C 7C 7C
00 7E 7E 7E 7E 7E 7E 7E
00
```

Here, the first byte is `0x02` representing the NPMD.

Then, seven `0x77` bytes and a `0x00` byte follow, representing the above `C4 . . . . . . _`

The song continues from there, each byte representing the note from the PEAT file.

It's worth noting that because of the single byte at the beginning of the file,
and our typical conventions of music revolving around groups of 4 notes,
it is very common that a file will be an odd number of bytes.

The entire Take Me Out To The Ball Game BEAT file is 397 bytes,
including the 12 continuous rests at the end used for resting before the track loops in another application.
