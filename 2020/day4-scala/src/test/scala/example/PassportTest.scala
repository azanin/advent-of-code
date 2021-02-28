package example

import org.scalatest.funsuite.AnyFunSuite

class PassportTest extends AnyFunSuite {

  test("parse rawInput and group in raw passoport data") {
    val input = List(
      "iyr:2015 cid:189 ecl:oth byr:1947 hcl:#6c4ab1 eyr:2026",
      "hgt:174cm",
      "pid:526744288",
      "                                                 ",
      "pid:688706448 iyr:2017 hgt:162cm cid:174 ecl:grn byr:1943 hcl:#808e9e eyr:2025"
    )

    val expectedResult = List(
      "iyr:2015 cid:189 ecl:oth byr:1947 hcl:#6c4ab1 eyr:2026\nhgt:174cm\npid:526744288",
      "pid:688706448 iyr:2017 hgt:162cm cid:174 ecl:grn byr:1943 hcl:#808e9e eyr:2025"
    )

    val actual = Passport.parse(input)

    assert(actual == expectedResult)
  }

  test("parse raw passport data in Passport case class") {

    val input =
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"

    val expected =
      Map(
        "ecl" -> "gry",
        "pid" -> "860033327",
        "eyr" -> "2020",
        "hcl" -> "#fffffd",
        "byr" -> "1937",
        "iyr" -> "2017",
        "cid" -> "147",
        "hgt" -> "183cm"
      )

    val actual = Passport.parse(input)

    assert(actual == expected)
  }

  test("isValid - true") {

    val input =
      Map(
        "ecl" -> "gry",
        "pid" -> "860033327",
        "eyr" -> "2020",
        "hcl" -> "#fffffd",
        "byr" -> "1937",
        "iyr" -> "2017",
        "cid" -> "147",
        "hgt" -> "183cm"
      )

    val actual = Passport.isValid(input)

    assert(actual)
  }

  test("isValid - false") {

    val input =
      Map(
        "pid" -> "860033327",
        "eyr" -> "2020",
        "hcl" -> "#fffffd",
        "byr" -> "1937",
        "iyr" -> "2017",
        "cid" -> "147",
        "hgt" -> "183cm"
      )

    val actual = Passport.isValid(input)

    assert(!actual)
  }
}
