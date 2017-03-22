class Solution(object):
    def updateBoard(self, board, click):
        x = click[0]
        y = click[1]
        if board[x][y] == 'M':
            board[x][y] = 'X'
			
		return board
		
        limitX = len(board)
        limitY = len(board[0])
		stack = [click]
		seen = {click}	
		while stack:
			square = stack.pop()
			x = square[0]
			y = square[1]
		    mineCount = 0
		    for i in range(-1, 2):
			    for j in range(-1, 2):
				    if (i == 0 and j == 0) or x + i < 0 or x + i >= limitX or \
				    y + j < 0 or y + j >= limitY:
					    continue
				    if board[x + i][y + j] == 'M':
					    mineCount += 1
			if mineCount > 0:
				board[x][y] = str(mineCount)
				continue
					
			board[x][y] = 'B'
			seen.add(square)
			for i in range(-1, 2):
				for j in range(-1, 2):
				    xx = x + i
					yy = y + j
					if [xx, yy] in seen or (i == 0 and j == 0) or \
					    xx < 0 or xx >= limitX or yy < 0 or yy >= limitY:
						continue
					stack.add([xx, yy])
				
        return board