local t = {
    12,
    [1 + 1] = 32,
    [1] = 45,
    6,
    [5] = 33,
    1,
    2,
    3,
    [9] = 0,
    [9] = 8,
    [5] = 111,
    a, b, c = 1, 2, 3,
    [100.5] = 4,
    [34.9] = 10,
    [34.9] = 9,
    [{}] = 3,
    [function ()
        
    end] = 5,
}

for key, value in pairs(t) do
    print(key, value)
end
