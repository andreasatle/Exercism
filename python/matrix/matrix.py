class Matrix:
    def __init__(self, matrix_string):
        self.matrix = []
        for row in matrix_string.split("\n"):
            self.matrix.append([int(num) for num in row.split()])
        print(self.matrix)

    def row(self, index):
        # It is safer to return a copy of the row, in case you change it.
        return self.matrix[index-1].copy()

    def column(self, index):
        col = []
        for row in self.matrix:
            col.append(row[index-1])
        return col
