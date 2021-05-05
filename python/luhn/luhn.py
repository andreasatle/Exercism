''' Determin if a number is a valid Luhn formula.

Given a number determine whether or not it is valid per the Luhn formula.

The Luhn algorithm is a simple checksum formula used to validate a variety
of identification numbers, such as credit card numbers and Canadian Social
Insurance Numbers.

The task is to check if a given string is valid.

Validating a Number
Strings of length 1 or less are not valid. Spaces are allowed in the input,
but they should be stripped before checking. All other non-digit characters
are disallowed.

Example 1: valid credit card number
4539 1488 0343 6467
The first step of the Luhn algorithm is to double every second digit,
starting from the right. We will be doubling

4_3_ 1_8_ 0_4_ 6_6_
If doubling the number results in a number greater than 9 then subtract 9
from the product. The results of our doubling:

8569 2478 0383 3437
Then sum all of the digits:

8+5+6+9+2+4+7+8+0+3+8+3+3+4+3+7 = 80
If the sum is evenly divisible by 10, then the number is valid.
This number is valid!

Example 2: invalid credit card number
8273 1232 7352 0569
Double the second digits, starting from the right

7253 2262 5312 0539
Sum the digits

7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57
57 is not evenly divisible by 10, so this number is not valid.
'''


class Luhn:
    '''Class handling the Luhn property.'''

    def __init__(self, card_num: str):
        '''Initialize the class, by filtering the input and compute the Luhn property.'''
        self._init_card_num(card_num)
        self._init_validity()

    def __str__(self):
        '''Returns a string of the class Luhn.'''
        return f"card-number: {self.card_num}\nis_valid: {self.is_valid}"

    def _init_card_num(self, card_num: str):
        '''Helper function to read the card number.'''
        # Remove any white-space
        self.card_num = ""
        for digits in card_num.split():
            self.card_num += digits

    def _init_validity(self):
        '''Helper function to compute the Luhn-property.'''
        # Check that card-number is big enough and only contains numbers
        if len(self.card_num) <= 1 or not self.card_num.isnumeric():
            self.is_valid = False
            return

        # Check the Luhn-property
        sum = 0
        for i in range(1, len(self.card_num)+1):
            digit = int(self.card_num[-i])
            if i & 1 == 0:
                digit *= 2
                if digit >= 10:
                    digit -= 9
            sum += digit

        self.is_valid = sum % 10 == 0

    def valid(self):
        '''Return whether the Luhn property is True.'''
        return self.is_valid
