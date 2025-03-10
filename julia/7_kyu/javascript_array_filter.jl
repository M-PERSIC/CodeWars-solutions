#=
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
=#

# https://www.codewars.com/kata/514a6336889283a3d2000001

function getevennumbers(arr)
  filter(iseven, (arr))
end

if abspath("javascript_array_filter.jl") == @__FILE__
	println(getevennumbers([2,4,5,6])) # [2,4,6]
	println(getevennumbers([])) # []
	println(getevennumbers([1])) # [] 
	println(getevennumbers([1,2])) # [2]
	println(getevennumbers([1,2,3,4,5])) # [2,4]    
	println(getevennumbers([2,4,6,8])) # [2,4,6,8]    
end

