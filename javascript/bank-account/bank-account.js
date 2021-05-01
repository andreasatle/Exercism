export class BankAccount {

  constructor() {
    this.isOpen = false;
  }

  open() {
    if (this.isOpen) {
      throw new ValueError("Trying to open an already opened account.");
    }
    this.isOpen = true;
    this.amount = 0;
  }

  close() {
    if (!this.isOpen) {
      throw new ValueError("Trying to close an already closed account.");
    }
    this.isOpen = false;
  }

  deposit(amount) {
    if (!this.isOpen) {
      throw new ValueError("Trying to deposit in closed account.");
    }
    if (amount < 0) {
      throw new ValueError("Trying to deposit a negative amount.");
    }
    this.amount += amount;
  }

  withdraw(amount) {
    if (!this.isOpen) {
      throw new ValueError("Trying to deposit in closed account.");
    }
    if (this.amount < amount) {
      throw new ValueError("Not enough money in account.");
    }
    if (amount < 0) {
      throw new ValueError("Trying to withdraw a negative amount.");
    }

    this.amount -= amount;
  }

  get balance() {
    if (!this.isOpen) {
      throw new ValueError("Trying to get the balance in closed account.");
    }
    return +this.amount;
  }
}

export class ValueError extends Error {
  constructor() {
    super('Bank account error');
  }
}
