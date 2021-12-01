import java.io.File

fun main() {
    println("[Day 1]")

    println(File("../../aoc-rs/input/2021/day1.txt").readLines().map { line -> line.toInt() }.windowed(2)
        .count { window -> window[0] < window[1] })
    println(File("../../aoc-rs/input/2021/day1.txt").readLines().map { line -> line.toInt() }.windowed(4)
        .count { window -> window[0] + window[1] + window[2] < window[1] + window[2] + window[3] }
    )
}

main()
