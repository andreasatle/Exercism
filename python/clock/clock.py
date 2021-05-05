'''Implement a clock that handles times without dates.

You should be able to add and subtract minutes to it.

Two clocks that represent the same time should be equal to each other.
'''


class Clock:
    '''A clock that handles times without dates.'''

    def __init__(self, hour: int, minute: int):
        '''Initialize the time and its string-represenation.'''
        self.minute = (60*hour + minute) % 1440
        self.str = self._init_str()

    def __repr__(self):
        return self.str

    def __eq__(self, other):
        '''Check if two times are equal.'''
        return self.minute == other.minute

    def __add__(self, minutes: int) -> 'Clock':
        '''Add minutes to time.'''
        return Clock(0, self.minute+minutes)

    def __sub__(self, minutes: int) -> 'Clock':
        '''Subtract minutes from time.'''
        return Clock(0, self.minute-minutes)

    def _init_str(self):
        '''Generates the string-representation of time.'''
        hour = str(self.minute//60)
        if len(hour) == 1:
            hour = "0"+hour
        minute = str(self.minute % 60)
        if len(minute) == 1:
            minute = "0"+minute
        return f"{hour}:{minute}"
