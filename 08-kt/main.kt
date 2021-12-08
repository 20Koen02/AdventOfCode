import java.io.File
import kotlin.collections.listOf

typealias Entry = List<List<String>>

val DIGS =
    listOf("abcdeg", "ab", "acdfg", "abcdf", "abef", "bcdef", "bcdefg", "abd", "abcdefg", "abcdef")

fun readLines(): List<Entry> {
  return File("in.txt").bufferedReader().readLines().map { l ->
    l.split(" | ").map { it.split(" ") }
  }
}

fun String.permute(result: String = ""): List<String> =
    if (isEmpty()) listOf(result)
    else flatMapIndexed { i, c -> removeRange(i, i + 1).permute(result + c) }

fun partOne(entry: Entry): Int {
  return entry[1].filter { x -> listOf(2, 3, 4, 7).contains(x.length) }.count()
}

fun getValidDigit(perm: String, dig: String): Int {
  // Check if the digit is valid
  var decoded = dig.map { perm[it - 'a'] }.sorted().joinToString("")
  return DIGS.indexOf(decoded)
}

fun tryPerm(perm: String, entry: Entry): Boolean {
  // Check if all signal patterns are valid digits
  var invalid = entry[0].map { getValidDigit(perm, it) }.any { it == -1 }
  return !invalid
}

fun partTwo(entry: Entry): Int {
  // Find correct permutation
  val perm = "abcdefg".permute().find { tryPerm(it, entry) }

  // Concat each digit
  return entry[1].map { getValidDigit(perm!!, it) }.joinToString(separator = "").toInt()
}

fun main() {
  println("Running...\n")

  val lines = readLines()

  val partOne = lines.map { partOne(it) }.sum()
  println("Day 8 part one: $partOne")

  val partTwo = lines.map { partTwo(it) }.sum()
  println("Day 8 part two: $partTwo")
}
