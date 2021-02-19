package example
object Day3 {

  sealed trait Spot
  case class Tree() extends Spot
  case class Free() extends Spot

  type Row = List[Spot]
  type Map = List[Row]


  case class Position(columnIndex: Int, rowIndex: Int)
  case class Dimension(columnLenght: Int, rowLength: Int)

  case class MoveStrategy(columnOffset: Int, rowOffset: Int)

  def evaluateNextPosition(actualPosition: Position, dimension: Dimension, moveStrategy: MoveStrategy): Option[Position] = {
    if(actualPosition.columnIndex == dimension.columnLenght - 1) None
    else Some(Position(actualPosition.columnIndex + moveStrategy.columnOffset, (actualPosition.rowIndex + moveStrategy.rowOffset) % dimension.rowLength))
  }

  def count(map: Map, position: Position) = {
    map(position.columnIndex)(position.rowIndex) match {m
      case Tree() => BigInt(1)
      case Free() => BigInt(0)
    }
  }

  def travel(map: Map, dimension: Dimension, moveStrategy: MoveStrategy = MoveStrategy(1, 3), startPosition: Position = Position(0, 0), counter: BigInt = 0): BigInt = {
    val nextPosition = evaluateNextPosition(startPosition, dimension, moveStrategy)

    nextPosition match {
      case None => count(map, startPosition) + counter
      case Some(position) => travel(map, dimension, moveStrategy,position, count(map, startPosition) + counter)
    }
  }

  def parse(line: String): List[Spot] = line.map(parse).toList

  def parse(spotRaw: Char) : Spot = spotRaw match {
    case '.' => Free()
    case '#' => Tree()
    case _ => throw new RuntimeException
  }
}
