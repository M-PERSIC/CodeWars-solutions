-- [[
-- Michael Persico
-- Oct.16, 2021
-- Disemvowel Trolls
-- https://www.codewars.com/kata/52fba66badcd10859f00097e
-- ]]

function disemvowel(s) return string.gsub(s, "[aeiouAEIOU]", "") end

print(disemvowel("This website")) -- Ths wbst
