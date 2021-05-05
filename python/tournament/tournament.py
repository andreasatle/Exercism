'''Tally the results of a small football competition.

Based on an input file containing which team played against which and what the outcome was, create a file with a table like this:

Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
'''

from dataclasses import dataclass


@dataclass
class Team:
    '''Implementation of a Team class.'''
    name:   str
    wins:   int
    draws:  int
    losses: int

    def __lt__(self, other):
        return self.points() > other.points() or (self.points() == other.points() and self.name < other.name)

    def win(self):
        '''Increase wins by 1.'''
        self.wins += 1

    def draw(self):
        '''Increase draws by 1.'''
        self.draws += 1

    def lose(self):
        '''Increase losses by 1.'''
        self.losses += 1

    def matches(self) -> int:
        '''Returns the number of played matches.'''
        return self.wins + self.draws + self.losses

    def points(self) -> int:
        '''Returns the number of points for the team.'''
        return 3*self.wins + self.draws

    def outcome(self) -> tuple[str, int, int, int, int, int]:
        '''Returns a tuple with the outcome.'''
        return self.name, self.matches(), self.wins, self.draws, self.losses, self.points()


def read_tally(rows: list[str]) -> dict:
    '''Read the games played in the division.'''
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


def print_tally(teams: dict) -> list[str]:
    '''Return the formatted tally list.'''
    teams = list(teams.values())
    teams.sort()
    tally = [f"Team                           | MP |  W |  D |  L |  P"]
    for team in teams:
        name, matches, wins, draws, losses, points = team.outcome()
        tally.append(
            f"{name:30} | {matches:2} | {wins:2} | {draws:2} | {losses:2} | {points:2}")
    return tally


def tally(rows: list[str]) -> list[str]:
    '''Read the tally and output its string.'''
    return print_tally(read_tally(rows))
