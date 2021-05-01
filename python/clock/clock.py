class Clock:
    def __init__(self, hour: int, minute: int):
        self.minute = (60*hour + minute) % 1440
        self.str = self._init_str()
        pass

    def __repr__(self):
        return self.str

    def __eq__(self, other):
        return self.minute == other.minute

    def __add__(self, minutes: int):
        return Clock(0, self.minute+minutes)

    def __sub__(self, minutes: int):
        return Clock(0, self.minute-minutes)

    def _init_str(self):
        hour = str(self.minute//60)
        if len(hour) == 1:
            hour = "0"+hour
        minute = str(self.minute % 60)
        if len(minute) == 1:
            minute = "0"+minute
        return f"{hour}:{minute}"
