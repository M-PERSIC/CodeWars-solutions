#=
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
=#

# https://www.codewars.com/kata/546e2562b03326a88e000020

function squareddigits(num)
  parse(Int, join(reverse(digits(num)) .|> (x -> x^2)))
end

if abspath("square_every_digit.jl") == @__FILE__
	println(squareddigits(9119)) # 811181
	println(squareddigits(0)) # 0	
end
