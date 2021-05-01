class Luhn:
    def __init__(self, card_num: str):
        self.init_card_num(card_num)
        self.init_validity()

    def __str__(self):
        return f"card-number: {self.card_num}\nis_valid: {self.is_valid}"

    def init_card_num(self, card_num: str):
        # Remove any white-space (as defined by split() is suppose...)
        self.card_num = ""
        for digits in card_num.split():
            self.card_num += digits

    def init_validity(self):
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
        return self.is_valid
