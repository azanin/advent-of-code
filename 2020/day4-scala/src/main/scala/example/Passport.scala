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

  val requiredFiels: Map[String, String => Boolean] = Map(
    "byr" -> { s => (1920 to 2002).contains(s.toInt) },
    "iyr" -> { s => (2010 to 2020).contains(s.toInt) },
    "eyr" -> { s => (2020 to 2030).contains(s.toInt) },
    "hgt" -> {
      case s"""${height}cm""" => (150 to 193).contains(height.toInt)
      case s"""${height}in""" => (59 to 76).contains(height.toInt)
      case _                  => false
    },
    "hcl" -> { s =>
      s.startsWith("#") && s.length == 7 && s.tail.forall(c => (('a' to 'f') ++ ('0' to '9')).contains(c))
    },
    "ecl" -> { s => List("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(s) },
    "pid" -> { s => s.forall(_.isDigit) && s.length == 9 }
  )

  def isValid(fields: Map[String, String]): Boolean = requiredFiels.keySet.forall(fields.contains)

  def isValid2(fields: Map[String, String]): Boolean = requiredFiels.forall { case (key, predicate) =>
    fields.get(key).exists(predicate)
  }

}
