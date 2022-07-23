-- [[ 
-- Michael Persico
-- Sept. 26, 2021
-- Build Tower
-- https://www.codewars.com/kata/576757b1df89ecf5bd00073b/
-- ]]

function towerBuilder(nFloors)
    tower = {}
    fullFloor = nFloors - 1
    star = 1
    row = ""
    for i = 1, nFloors do
        for j = 1, fullFloor do
            row = row .. " "
        end
        for k = 1, star do
            row = row .. "*"
        end
        for l = 1, fullFloor do
            row = row .. " "
        end
        table.insert(tower,row)
        star = star + 2
        fullFloor = fullFloor - 1
        row = ""
    end
    return tower
end

print(towerBuilder(1))
print(towerBuilder(2))
print(towerBuilder(3))
