'''Determine plants for each child in the class.

Given a diagram, determine which plants each child in the kindergarten class is responsible for.

The kindergarten class is learning about growing plants. The teacher thought it would be a good idea to give them actual seeds, plant them in actual dirt, and grow actual plants.

They've chosen to grow grass, clover, radishes, and violets.

To this end, the children have put little cups along the window sills, and planted one type of plant in each cup, choosing randomly from the available types of seeds.

[window][window][window]
........................ # each dot represents a cup
........................
There are 12 children in the class:

Alice, Bob, Charlie, David,
Eve, Fred, Ginny, Harriet,
Ileana, Joseph, Kincaid, and Larry.
Each child gets 4 cups, two on each row. Their teacher assigns cups to the children alphabetically by their names.

The following diagram represents Alice's plants:

[window][window][window]
VR......................
RG......................
In the first row, nearest the windows, she has a violet and a radish. In the second row she has a radish and some grass.

Your program will be given the plants from left-to-right starting with the row nearest the windows. From this, it should be able to determine which plants belong to each student.

For example, if it's told that the garden looks like so:

[window][window][window]
VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV
Then if asked for Alice's plants, it should provide:

Violets, radishes, violets, radishes
While asking for Bob's plants would yield:

Clover, grass, clover, clover
'''


class Garden:
    '''Gargen manager class that keeps track of which child 
    has which flowers in the garden..'''

    def __init__(self, diagram: str, students: list[str] = None):
        '''Initializes the Garden class.'''
        self._create_flower_map()
        self._create_diagram(diagram)
        self._create_students(students)

    def __str__(self):
        '''Definition of string-output for the class.'''
        return f"row 1: {self.row1},\nrow 2: {self.row2},\nstudents: {self.students},\nflower_map: {self.flower_map}"

    def _create_diagram(self, diagram: str):
        '''Help function to create the diagram.'''
        rows = diagram.split("\n")
        if len(rows) != 2 or len(rows[0]) != len(rows[1]):
            raise ValueError(f"Can't process diagram: {diagram}!")
        self.row1 = rows[0]
        self.row2 = rows[1]

        # Check that the flowers are valid
        for flower in self.row1 + self.row2:
            if flower not in self.flower_map.keys():
                raise ValueError(
                    f"Flower: {flower} not in flower-map: {self.flower_map}")

    def _create_students(self, students: list[str]):
        '''Help function to initialize students.'''
        if students is None:
            students = ["Alice", "Bob", "Charlie", "David",
                        "Eve", "Fred", "Ginny", "Harriet",
                        "Ileana", "Joseph", "Kincaid", "Larry"]
        self.students = sorted(students)

    def _create_flower_map(self):
        '''Help function to initialize flower map.'''
        self.flower_map = {
            'G': 'Grass', 'C': 'Clover', 'R': 'Radishes', 'V': 'Violets'
        }

    def plants(self, student: str) -> list[str]:
        '''Returns the 4 plants for the given child.
        If the child is not in the class, we throw a ValueError.'''

        # Check that student is valid
        if student not in self.students:
            raise ValueError(
                f"Student {student} is not in class: {self.students}")

        i_student = self.students.index(student)
        if 2*i_student+1 >= len(self.row1):
            raise ValueError(
                f"Student-index {i_student} is larger than number of available plants.")

        return [
            self.flower_map[self.row1[2*i_student]],
            self.flower_map[self.row1[2*i_student+1]],
            self.flower_map[self.row2[2*i_student]],
            self.flower_map[self.row2[2*i_student+1]]
        ]
