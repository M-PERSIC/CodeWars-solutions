-- [
-- Michael Persico
-- Sept.30, 2021
-- String matchup
-- https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4
-- ]

function solve(a,b)
    strCounter, result = {}, {}
    for i = 1, #b do
        strB = b[i]
        strCounter[strB] = 0
        for j = 1, #a do
            strA = a[j]
            if (strB == strA) then
                strCounter[strB] = strCounter[strB] + 1
            end
        end
        table.insert(result, strCounter[strB])
    end
    return result
end

firstRun = solve({'abc', 'abc', 'xyz', 'abcd', 'cde'}, {'abc', 'cde', 'uap'})
for i = 1, #firstRun do print(firstRun[i]) end -- [2,1,0]
secondRun = solve({'abc', 'xyz', 'abc', 'xyz', 'cde'}, {'abc', 'cde', 'xyz'})
for i = 1, #secondRun do print(secondRun[i]) end -- [2,1,2]
thirdRun = solve({'quick', 'brown', 'fox', 'is', 'quick'}, {'quick', 'abc', 'fox'})
for i = 1, #thirdRun do print(thirdRun[i]) end -- [2,0,1]
