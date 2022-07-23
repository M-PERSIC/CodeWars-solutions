# Michael Persico
# Sept. 25, 2021
# Two to One
# https://www.codewars.com/kata/5656b6906de340bd1b0000ac

longest <- function(s1, s2) {
  s1 <- unlist(strsplit(s1,""))
  s2 <- unlist(strsplit(s2,""))
  chars <- c(s1,s2)
  return(paste(c(sort(unique(chars))), collapse=""))
}

longest("aretheyhere", "yestheyarehere") #aehrsty
longest("loopingisfunbutdangerous", "lessdangerousthancoding") #abcdefghilnoprstu
