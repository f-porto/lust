local t = {
    a = { a = 2 }
}

function t.print_hi()
    for key, value in pairs(t) do
        print(key, value)
    end
    print "hi"
end

local function get_t()
    t = 10
    return "a"
end

t.print_hi()

print(t.a[get_t()])
