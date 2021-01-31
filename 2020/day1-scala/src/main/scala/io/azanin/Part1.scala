package io.azanin

import scala.io.Source

object Part1 extends App {

  val input = Source.fromResource("input.txt").getLines().map(s => s.toInt).toList

  val way1 = for {
    a  <- input
    b  <- input
    res = a * b if a + b == 2020
  } yield res

  val way2 = input.combinations(2).find(_.sum == 2020).map(_.product)

  println(way1)
  println(way2)

  val way3 = input.combinations(3).find(_.sum == 2020).map(_.product)

  val way4 = for {
    a  <- input
    b  <- input
    c  <- input
    res = a * b * c if a + b + c == 2020
  } yield res

  println(way3)
  println(way4)
}
