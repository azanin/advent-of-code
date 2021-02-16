package example

import example.Day3.{Dimension, Free, Position, Tree}
import org.scalatest.funsuite.AnyFunSuite

class Day3Test extends AnyFunSuite {

  test("next position starting from (0, 0)") {

    val dimension = Dimension(322, 31)
    val position = Day3.evaluateNextPosition(Position(0, 0), dimension)

    assert(position.contains(Position(1, 3)))
  }

  test("next position starting from the last raw index") {
    val dimension = Dimension(322, 31)
    val position = Day3.evaluateNextPosition(Position(10, 30), dimension)

    assert(position.contains(Position(11, 2)))
  }

  test("next position from last row should be None") {
    val dimension = Dimension(322, 31)
    val position = Day3.evaluateNextPosition(Position(321, 28), dimension)

    assert(position.isEmpty)
  }

  test("travel") {
    val dimension = Dimension(4, 9)
    val map: List[List[Day3.Spot]] = List(
      List(Free(), Tree(), Free(), Free(), Tree(), Free(), Free(), Tree(), Free()),
      List(Free(), Tree(), Free(), Tree(), Tree(), Free(), Free(), Tree(), Free()),
      List(Free(), Tree(), Free(), Free(), Tree(), Free(), Free(), Tree(), Free()),
      List(Tree(), Tree(), Free(), Free(), Tree(), Free(), Free(), Tree(), Free())
    )

    val numTree = Day3.travel(map, dimension)

    assert(numTree == 2)
  }

}
