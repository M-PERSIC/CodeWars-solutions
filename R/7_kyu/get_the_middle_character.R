# Michael Persico
# Oct.02, 2021
# Get the Middle Character
# https://www.codewars.com/kata/56747fd5cb988479af000028

get_middle <- function(s) {
    lengthCorrect <- ifelse(nchar(s) %% 2 == 0, 1, 0)
    substr(s, ceiling(nchar(s)/2), ceiling(nchar(s)/2) + lengthCorrect)
}

get_middle("test") # es
get_middle("testing") # t
