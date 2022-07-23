-- [[ 
-- Michael Persico
-- Oct.08, 2021
-- Count characters in your string
-- https://www.codewars.com/kata/52efefcbcdf57161d4000091
-- ]]

function count_chars(s)
  char_list = {}
  for i=1, #s do
    if (char_list[string.sub(s,i,i)] == nil) then 
        char_list[string.sub(s,i,i)] = 1 
    else
       char_list[string.sub(s,i,i)] = char_list[string.sub(s,i,i)] + 1
    end
  end
  return char_list
end

list = count_chars("aa")
for i,v in pairs(list) do print(i,v) end -- a 2

