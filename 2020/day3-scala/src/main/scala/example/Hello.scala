package example

import example.Day3.{Dimension, Spot}

import scala.io.Source

object Hello extends App {
  val lines = Source.fromFile("input.txt").getLines().toList

  val map: List[List[Spot]] = lines.map(Day3.parse)
  val dimension = Dimension(map.size, map.lastOption.map(_.size).getOrElse(0))

  val num = Day3.travel(map, dimension)

  println(num)
}
