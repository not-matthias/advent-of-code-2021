import java.io.File

fun main() {
    println("[Day 1]")

    println(File("../../aoc-rs/input/2021/day1.txt").readLines().map { it.toInt() }.windowed(2).count { it[0] < it[1] })
    println(File("../../aoc-rs/input/2021/day1.txt").readLines().map { it.toInt() }.windowed(4)
        .count { it[0] + it[1] + it[2] < it[1] + it[2] + it[3] })
}

main()
