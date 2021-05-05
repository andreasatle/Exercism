'''Create a roster for a school.

Given students' names along with the grade that they are in,
create a roster for the school.
'''


class School:
    def __init__(self):
        self.students_per_grade = {}

    def add_student(self, name: str, grade: int):
        '''Add student to roster.'''
        if grade in self.students_per_grade.keys():
            self.students_per_grade[grade].append(name)
        else:
            self.students_per_grade[grade] = [name]
        # A bit overkill to sort entire grade...
        self.students_per_grade[grade].sort()

    def roster(self) -> list[str]:
        '''Return a roster for the school.'''
        out = []
        for grade, students in sorted(self.students_per_grade.items()):
            for student in students:
                out.append(student)
        return out

    def grade(self, grade_number: int) -> list[str]:
        '''Return the students in grade_number.'''
        if grade_number not in self.students_per_grade.keys():
            return []
        # The copy prevents outsiders to manipulate database.
        return self.students_per_grade[grade_number].copy()
