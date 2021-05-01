from dataclasses import dataclass


@dataclass
class Team:
    name:   str
    wins:   int
    draws:  int
    losses: int

    def __lt__(self, other):
        return self.points() > other.points() or (self.points() == other.points() and self.name < other.name)

    def win(self):
        self.wins += 1

    def draw(self):
        self.draws += 1

    def lose(self):
        self.losses += 1

    def matches(self):
        return self.wins + self.draws + self.losses

    def points(self):
        return 3*self.wins + self.draws

    def outcome(self) -> tuple[str, int, int, int, int, int]:
        return self.name, self.matches(), self.wins, self.draws, self.losses, self.points()


def read_tally(rows: list[str]) -> dict:
    teams = {}
    for row in rows:
        home, visitor, outcome = row.split(";")
        if home not in teams.keys():
            teams[home] = Team(home, 0, 0, 0)
        if visitor not in teams.keys():
            teams[visitor] = Team(visitor, 0, 0, 0)
        if outcome == 'win':
            teams[home].win()
            teams[visitor].lose()
        elif outcome == 'loss':
            teams[home].lose()
            teams[visitor].win()
        elif outcome == 'draw':
            teams[home].draw()
            teams[visitor].draw()
    return teams


def print_tally(teams: dict):
    teams = list(teams.values())
    teams.sort()
    tally = [f"Team                           | MP |  W |  D |  L |  P"]
    for team in teams:
        name, matches, wins, draws, losses, points = team.outcome()
        tally.append(
            f"{name:30} | {matches:2} | {wins:2} | {draws:2} | {losses:2} | {points:2}")
    return tally


def tally(rows: list[str]):
    return print_tally(read_tally(rows))
