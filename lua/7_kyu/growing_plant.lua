-- [[
-- Michael Persico
-- Sept. 27, 2021
-- Growing Plant
-- https://www.codewars.com/kata/58941fec8afa3618c9000184
-- ]]


function growing_plant(upSpeed, downSpeed, desiredHeight)
    height, day = 0, 1
    while (height <= desiredHeight) do
        height = height + upSpeed
        if ((height >= desiredHeight) == false ) then
            height = height - downSpeed 
            day = day + 1
        end
    end
    return day
end

print(growing_plant(100,10,910)) -- 10
print(growing_plant(10,9,4)) -- 1
print(growing_plant(5,2,5)) -- 1
print(growing_plant(5,2,6)) -- 2
