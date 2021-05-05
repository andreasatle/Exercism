''' Matrix handler class.

Given a string representing a matrix of numbers,
return the rows and columns of that matrix.
'''


class Matrix:
    '''Matrix class, returns row and columns of matrix.'''

    def __init__(self, matrix_string: str):
        '''Initializer of Matrix class.'''
        self.matrix = []
        for row in matrix_string.split("\n"):
            self.matrix.append([int(num) for num in row.split()])

    def row(self, index: int) -> list[int]:
        '''Returns row given by index.'''
        # It is safer to return a copy of the row, in case you change it.
        return self.matrix[index-1].copy()

    def column(self, index: int) -> list[int]:
        '''Returns column given by index.'''
        col = []
        for row in self.matrix:
            col.append(row[index-1])
        return col
