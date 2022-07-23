# Michael Persico
# Oct.03, 2021
# Take a Ten Minute Walk
# https://www.codewars.com/kata/54da539698b8a2ad76000228

isValidWalk <- function(walk){
  if (length(walk) > 10 || length(walk) < 10) {
    return (FALSE)
  }
  directions <- c("n"=0, "s"=0, "e"=0, "w"=0)
  for (direction in walk) {
    if (direction == "n") {
      directions[["n"]] = directions[["n"]] + 1
    } else if (direction == "s") {
      directions[["s"]] = directions[["s"]] + 1
    } else if (direction == "e") {
      directions[["e"]] = directions[["e"]] + 1
    } else {
      directions[["w"]] = directions[["w"]] + 1
    }
  }
  ifelse((directions[["n"]] - directions[["s"]]) == 0 && (directions[["e"]] - directions[["w"]]) == 0, return (TRUE), return (FALSE))
}

isValidWalk(c("n", "e", "w", "n", "e", "w", "n", "e", "w", "s")) # False
