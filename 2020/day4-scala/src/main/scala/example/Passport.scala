package example

object Passport {

  def parse(rawLines: List[String]): List[String] =
    rawLines
      .foldLeft(Nil: List[List[String]]) { (acc, line) =>
        if (line.isBlank) acc :+ Nil
        else acc.lastOption.fold(List(List(line)))(lastList => acc.updated(acc.length - 1, lastList :+ line))
      }
      .foldLeft(Nil: List[String]) { (acc, curr) =>
        acc :+ curr.mkString("\n")
      }

  def parse(input: String): Map[String, String] =
    input
      .split(Array(' ', '\n'))
      .map(_.split(':').toList match {
        case first :: second :: Nil => (first, second)
      })
      .toMap

  val requiredFiels = List("ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt")

  def isValid(fields: Map[String, String]): Boolean = requiredFiels.forall(fields.contains)

}
