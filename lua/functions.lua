t = {
    a = 10,
    b = 7
}

function t.dosomthing()
    print(t.a)
end

t0 = {
    a = "HELLO"
}

function t:what()
    print(self.b)

    local t0 = {}
    function t0:hi()
        print(self.a)
    end

    t0:hi()
    
    function inside()
        print("inseid")
    end
end

t.dosomthing()
t:what()
t:dosomthing()
t.what(t)
t0:hi()
inside()

for key, value in pairs(t) do
    print(key, value)
end
