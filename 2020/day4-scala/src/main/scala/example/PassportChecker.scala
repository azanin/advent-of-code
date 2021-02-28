package example

import scala.io.Source

object PassportChecker {

  def run() = {
    val input: List[String] = Source.fromFile("input.txt").getLines().toList
    Passport.parse(input).map(Passport.parse).count(Passport.isValid)
  }
}
