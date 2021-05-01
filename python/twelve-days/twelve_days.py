count = ["", "first", "second", "third", "fourth", "fifth", "sixth",
         "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"]

gift = [
    "",
    "a Partridge in a Pear Tree.",
    "two Turtle Doves, and ",
    "three French Hens, ",
    "four Calling Birds, ",
    "five Gold Rings, ",
    "six Geese-a-Laying, ",
    "seven Swans-a-Swimming, ",
    "eight Maids-a-Milking, ",
    "nine Ladies Dancing, ",
    "ten Lords-a-Leaping, ",
    "eleven Pipers Piping, ",
    "twelve Drummers Drumming, ",
]


def recite(start_verse, end_verse):
    # Ignore case where end_verse < start_verse (then do nothing.)
    if start_verse < 1 or end_verse > 12:
        raise ValueError("Wrong range of verses.")

    s = []
    for idx in range(start_verse, end_verse+1):
        s.append(days_of_xmas(idx))
    return s


def days_of_xmas(idx):
    s = f"On the {count[idx]} day of Christmas my true love gave to me: "
    for i in range(idx, 0, -1):
        s += gift[i]
    return s
