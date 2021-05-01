class Garden:
    def __init__(self, diagram, students=None):
        self.create_flower_map()
        self.create_diagram(diagram)
        self.create_students(students)
        pass

    def __str__(self):
        return f"row 1: {self.row1},\nrow 2: {self.row2},\nstudents: {self.students},\nflower_map: {self.flower_map}"

    def create_diagram(self, diagram):
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

    def create_students(self, students):
        if students is None:
            students = ["Alice", "Bob", "Charlie", "David",
                        "Eve", "Fred", "Ginny", "Harriet",
                        "Ileana", "Joseph", "Kincaid", "Larry"]
        self.students = sorted(students)

    def create_flower_map(self):
        self.flower_map = {
            'G': 'Grass', 'C': 'Clover', 'R': 'Radishes', 'V': 'Violets'
        }

    def plants(self, student):
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
