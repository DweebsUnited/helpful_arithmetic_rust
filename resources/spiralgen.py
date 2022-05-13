def spiral(X, Y):
    x = y = 0
    dx = 0
    dy = -1
    eol = max(X, Y)**2
    for i in range(eol):
        print( "{} < {}".format( i, eol ) )
        print( "{}<{}<={} - {}<{}<={} -- {}x{}".format(
            -X/2, x, X/2,
            -Y/2, y, Y/2,
            x + X/2, y + Y/2
        ))
        if (-X/2 < x <= X/2) and (-Y/2 < y <= Y/2):
            print (x, y)
            # DO STUFF...
        if x == y or (x < 0 and x == -y) or (x > 0 and x == 1-y):
            dx, dy = -dy, dx
        x, y = x+dx, y+dy