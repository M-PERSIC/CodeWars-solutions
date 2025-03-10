--[[ 
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
]]

-- https://www.codewars.com/kata/52fba66badcd10859f00097e

function disemvowel(s) return string.gsub(s, "[aeiouAEIOU]", "") end

print(disemvowel("This website")) -- Ths wbst
