--[[ 
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
]]

-- https://www.codewars.com/kata/5822d89270ca28c85c0000f3

function scramble(str, idxs)
    chars = {}
    for i=1, #idxs do chars[idxs[i]] = string.sub(str,i,i) end
    return table.concat(chars,"")
end

print(scramble("dwolr", {5,1,2,4,3})) -- world
