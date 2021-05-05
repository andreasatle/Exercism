'''Add the mine counts to a completed Minesweeper board.

Minesweeper is a popular game where the user has to find
the mines using numeric hints that indicate how many mines
are directly adjacent (horizontally, vertically, diagonally)
to a square.

In this exercise you have to create some code that counts the
number of mines adjacent to a given empty square and replaces
that square with the count.

The board is a rectangle composed of blank space (' ')
characters. A mine is represented by an asterisk ('*') character.

If a given space has no adjacent mines at all,
leave that square blank.

Exception ValueError is raised if the minefield is not rectangular.
'''


def annotate(minefield: list[str]) -> list[str]:
    output = []
    for i in range(len(minefield)):
        row = ''

        # Check that minefield is rectangular.
        if i > 0 and len(minefield[i]) != len(minefield[i-1]):
            raise ValueError('Non-rectangular minefield not allowed.')

        for j in range(len(minefield[i])):
            if minefield[i][j] == '*':
                row += '*'
                continue

            # Check that minefield only contains characters '*' and ' '.
            if minefield[i][j] != ' ':
                raise ValueError(
                    f'Character {minefield[i][j]} is not allowed.')

            nbrs = ''

            for i0 in range(max(0, i-1), min(len(minefield), i+2)):
                for j0 in range(max(0, j-1), min(len(minefield[i0]), j+2)):
                    nbrs += minefield[i0][j0]

            cnt = nbrs.count('*')
            row += str(cnt) if cnt > 0 else ' '

        output.append(row)

    return output
