-- [[
-- Michael Persico
-- Oct.18, 2021
-- Predict your age!
-- https://www.codewars.com/kata/5aff237c578a14752d0035ae
-- ]]

function predictAge(age1,age2,age3,age4,age5,age6,age7,age8)
  age_list, sum = {}, 0
  table.insert(age_list, age1 * age1)
  table.insert(age_list, age2 * age2)
  table.insert(age_list, age3 * age3)
  table.insert(age_list, age4 * age4)
  table.insert(age_list, age5 * age5)
  table.insert(age_list, age6 * age6)
  table.insert(age_list, age7 * age7)
  table.insert(age_list, age8 * age8)
  for i=1, #age_list do sum = sum + age_list[i] end
  return math.floor(math.sqrt(sum) / 2)
end

print(predictAge(65,60,75,55,60,63,64,45)) -- 86

