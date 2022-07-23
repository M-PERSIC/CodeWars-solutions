-- [[
-- Michael Persico
-- Sept. 25, 2021
-- Drying Potates (7 kyu)
-- https://www.codewars.com/kata/58ce8725c835848ad6000007
-- ]]

local kata = {}

function kata.potatoes(p0, w0, p1)
    return math.floor((w0 * (100 - p0))/(100 - p1))
end

print(kata.potatoes(82,127,80)) -- 114
print(kata.potatoes(93, 129, 91)) -- 100

