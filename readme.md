# Befucked
Befucked is a programming language inspired by Brainfuck and Befunge. Befucked programs are written in a two-dimensional grid of ASCII characters, where the flow of the program is mostly linear with the exception of branches and control flow characters. A program ends when no character can be followed, or when a character that requires branches doesn't receive any branches. Instructions outside the flow of a program are interpreted as comments (unless these contain the character N, which would cause an error, as N is the character that determines where a program starts.)

## Memory
Befucked's memory has two arrays of memory cells, a mutable one called data space, and an immutable one called value space. It also has a pointer, which can contain a value and move in the memory table. The pointer's position in data space is at all times the same position it has in memory space. The pointer can grab a value from any space, which interhcanges the value it contains with that of the space. If a value space cell is grabbed, its value does not change.

The cells in value space are always set to be an integer number line, extending countably infinitely in both directions. The pointer starts at the data space value under the value space 0.

In theory, a cell can contain any integer number. 

## More Information
More information about Befucked, such as what instructions do and example programs, can be found in the esolangs wiki: https://esolangs.org/wiki/Befucked
