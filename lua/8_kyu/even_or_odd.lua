--[[ 
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
]]

-- https://www.codewars.com/kata/53da3dbb4a5168369a0000fe

function even_or_odd(number) if number % 2 == 0 then return "Even" else return "Odd" end end

print(even_or_odd(1)) -- Odd
print(even_or_odd(-2)) -- Even

