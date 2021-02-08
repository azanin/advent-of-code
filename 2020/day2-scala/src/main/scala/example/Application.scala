package example

import scala.io.Source

case class Policy(atLeast: Int, atMost: Int, character: Char)
case class Password(value: String)

object Application extends App {

  val lines = Source.fromFile("input.txt").getLines()

  val parsedValues = lines.map((raw: String) => {
    val splitted = raw.split(Array(' ', '-', ':'))
    val policy = Policy(splitted(0).toInt, splitted(1).toInt, splitted(2).charAt(0))
    val password = splitted(4)
    (password, policy)
  }).toList

  val result = parsedValues
    .count { case (password, policy) =>
      val occurence = password.count(c => c == policy.character)
      occurence >= policy.atLeast && occurence <= policy.atMost
    }

  val result2 = parsedValues.count { case(password, policy) =>
    val char1: Char = password.charAt(policy.atMost - 1)
    val char2: Char = password.charAt(policy.atLeast - 1)
    List(char1, char2).count(c => c == policy.character) == 1
  }

  println(result)
  println(result2)

}