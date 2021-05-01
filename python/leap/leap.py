def leap_year(year):
    leap = year % 4 == 0
    if year % 100 == 0:
        leap = False
    if year % 400 == 0:
        leap = True
    return leap
