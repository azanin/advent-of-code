import Dependencies._

ThisBuild / scalaVersion := "2.13.4"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "io.azanin"
ThisBuild / organizationName := "azanin"

lazy val root = (project in file("."))
  .settings(
    name := "day1-scala",
    libraryDependencies += scalaTest % Test
  )
