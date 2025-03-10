# SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
# 
# SPDX-License-Identifier: MIT-0

# https://www.codewars.com/kata/5656b6906de340bd1b0000ac

longest <- function(s1, s2) {
  s1 <- unlist(strsplit(s1,""))
  s2 <- unlist(strsplit(s2,""))
  chars <- c(s1,s2)
  return(paste(c(sort(unique(chars))), collapse=""))
}

longest("aretheyhere", "yestheyarehere") #aehrsty
longest("loopingisfunbutdangerous", "lessdangerousthancoding") #abcdefghilnoprstu
