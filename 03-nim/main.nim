
import std/sequtils
import std/strutils
import std/strformat
import std/sugar


proc rotate2D(seq2d: seq[seq[string]]): seq[seq[string]] = 
    for i, elem in seq2d[0]:
        result.add(seq2d.map(x => x[i]))


proc getLines(): seq =
    let f = open("in.txt")
    defer: f.close()

    return f.lines.toSeq().map(x => x.map(y => fmt"{y}"))


var lines = getLines()
let rotated = rotate2D(lines)
let bitmask = parseBinInt('1'.repeat(rotated.len))


proc powerConsumtion(seq2d: seq[seq[string]]): int =
    for x in seq2d:
        result = result or ord(x.count("1") > x.count("0"))
        result = result shl 1
    result = result shr 1


proc lifeSupport(seq2d: seq[seq[string]], opposite: bool = false): int = 
    var numbers = seq2d
    var i = 0

    while numbers.len > 1:
        let oneNumbers = numbers.filter(x => x[i] == "1")
        let zeroNumbers = numbers.filter(x => x[i] == "0")
        
        if oneNumbers.len >= zeroNumbers.len:
            numbers = if opposite: zeroNumbers else: oneNumbers
        else:
            numbers = if opposite: oneNumbers else: zeroNumbers

        i += 1
    
    return parseBinInt(numbers[0].join())


proc partOne(): int =
    let gamma = powerConsumtion(rotated)
    let epsilon = gamma xor bitmask

    return gamma * epsilon


proc partTwo(): int =
    let oxygen = lifeSupport(lines)
    let scrubber = lifeSupport(lines, true)

    return oxygen * scrubber


echo fmt"Day 3 part one: {$partOne()}"
echo fmt"Day 3 part two: {$partTwo()}"