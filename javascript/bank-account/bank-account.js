/**
 * Simulate a bank account supporting opening/closing, withdrawals,
 * and deposits of money. Watch out for concurrent transactions!
 * 
 * A bank account can be accessed in multiple ways. Clients can make
 * deposits and withdrawals using the internet, mobile phones, etc.
 * Shops can charge against the account.
 * 
 * Create an account that can be accessed from multiple
 * threads/processes (terminology depends on your programming language).
 * 
 * It should be possible to close an account; operations against a
 * closed account must fail.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * class for simulating a bank account.
 */
export class BankAccount {

  /**
   * @constructor
   */
  constructor() {
    this.isOpen = false;
  }
  /**  
   * Open a bank account.
   * @public
   * @throws {ValueError} Trying to open an already opened account.
   */
  open() {
    if (this.isOpen) {
      throw new ValueError("Trying to open an already opened account.");
    }
    this.isOpen = true;
    this.amount = 0;
  }

  /** 
   * Close a bank account.
   * @public
   * @throws {ValueError} Trying to close an already closed account.
   */
  close() {
    if (!this.isOpen) {
      throw new ValueError("Trying to close an already closed account.");
    }
    this.isOpen = false;
  }

  /**
   * Deposit money into an open bank account. 
   * @public
   * @param {number} amount Amount to be deposited.
   * @throws {ValueError} Trying to deposit in closed account.
   * @throws {ValueError} Trying to deposit a negative amount.
   */
  deposit(amount) {
    if (!this.isOpen) {
      throw new ValueError("Trying to deposit in closed account.");
    }
    if (amount < 0) {
      throw new ValueError("Trying to deposit a negative amount.");
    }
    this.amount += amount;
  }

  /**
   * Withdraw money from an open bank account. 
   * @public
   * @param {number} amount Amount to be withdrawn.
   * @throws {ValueError} Trying to withdraw money in a closed account.
   * @throws {ValueError} Not enough money in account.
   * @throws {ValueError} Trying to withdraw a negative amount.
   */
withdraw(amount) {
    if (!this.isOpen) {
      throw new ValueError("Trying to withdraw money in a closed account.");
    }
    if (this.amount < amount) {
      throw new ValueError("Not enough money in account.");
    }
    if (amount < 0) {
      throw new ValueError("Trying to withdraw a negative amount.");
    }

    this.amount -= amount;
  }

  /**
   * Balance from an open bank account. 
   * @public
   * @throws {ValueError} Trying to get the balance in closed account.
   */
  get balance() {
    if (!this.isOpen) {
      throw new ValueError("Trying to get the balance in closed account.");
    }
    return +this.amount;
  }
}

/**
 * Class ValueError signals a Value exception.
 * @extends Error
 */
export class ValueError extends Error {
  constructor() {
    super('Bank account error');
  }
}
