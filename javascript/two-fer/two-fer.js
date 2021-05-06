/**
 * Two-fer or 2-fer is short for two for one. One for you and one for me.
 * 
 * Given a name, return a string with the message:
 * 
 * <p>One for name, one for me.
 * <p>Where "name" is the given name.
 * 
 * However, if the name is missing, return the string:
 * 
 * <p>One for you, one for me.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Returns a string 'One for {name}, one for me'.
 * @func
 * @param {string} [name='you'] The name of the person.
 * @returns {string} the wanted string.
 */
export const twoFer = (name = "you") => {
  return "One for " + name + ", one for me.";
};
