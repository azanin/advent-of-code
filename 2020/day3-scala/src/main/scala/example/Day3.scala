package example
object Day3 {

  sealed trait Spot
  case class Tree() extends Spot
  case class Free() extends Spot

  type Row = List[Spot]
  type Map = List[Row]


  case class Position(columnIndex: Int, rowIndex: Int)
  case class Dimension(columnLenght: Int, rowLength: Int)

  def evaluateNextPosition(actualPosition: Position, dimension: Dimension): Option[Position] = {
    if(actualPosition.columnIndex == dimension.columnLenght - 1) None
    else Some(Position(actualPosition.columnIndex + 1, (actualPosition.rowIndex + 3) % dimension.rowLength))
  }

  def count(map: Map, position: Position) = {
    map(position.columnIndex)(position.rowIndex) match {
      case Tree() => 1
      case Free() => 0
    }
  }

  def travel(map: Map, dimension: Dimension, startPosition: Position = Position(0, 0), counter: Int = 0): Int = {
    val nextPosition = evaluateNextPosition(startPosition, dimension)

    nextPosition match {
      case None => count(map, startPosition) + counter
      case Some(position) => travel(map, dimension, position, count(map, startPosition) + counter)
    }
  }

  def parse(line: String): List[Spot] = line.map(parse).toList

  def parse(spotRaw: Char) : Spot = spotRaw match {
    case '.' => Free()
    case '#' => Tree()
    case _ => throw new RuntimeException
  }
}
