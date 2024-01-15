function fn.a.b:c()
    io.write()
end

--[[a.b.c["fsdf"][d](10, false).b = 10

b = 10
a = (function (c)
    return b + c
end)(20)
]]
