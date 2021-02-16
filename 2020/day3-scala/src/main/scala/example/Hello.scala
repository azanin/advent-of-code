package example

import example.Day3.{Dimension, MoveStrategy, Spot}

import scala.io.Source

object Hello extends App {
  val lines = Source.fromFile("input.txt").getLines().toList

  val strategy1 = MoveStrategy(1,1)
  val strategy2 = MoveStrategy(1, 3)
  val strategy3 = MoveStrategy(1, 5)
  val strategy4 = MoveStrategy(1, 7)
  val strategy5 = MoveStrategy(2, 1)

  val map: List[List[Spot]] = lines.map(Day3.parse)
  val dimension = Dimension(map.size, map.lastOption.map(_.size).getOrElse(0))

  val num =
    Day3.travel(map, dimension, strategy1) *
    Day3.travel(map, dimension, strategy2) *
    Day3.travel(map, dimension, strategy3) *
    Day3.travel(map, dimension, strategy4) *
    Day3.travel(map, dimension, strategy5)



  println(num)
}
