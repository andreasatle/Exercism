/**
 * If you want to build something using a Raspberry Pi, you'll probably use resistors. For this exercise, you need to know two things about them:
 * 
 * Each resistor has a resistance value.
 * Resistors are small - so small in fact that if you printed the resistance value on them, it would be hard to read.
 * To get around this problem, manufacturers print color-coded bands onto the resistors to denote their resistance values. Each band has a position and a numeric value.
 * 
 * The first 2 bands of a resistor have a simple encoding scheme: each color maps to a single number.
 * 
 * In this exercise you are going to create a helpful program so that you don't have to remember the values of the bands.
 * 
 * These colors are encoded as follows:
 * 
 * <p>Black: 0
 * <p>Brown: 1
 * <p>Red: 2
 * <p>Orange: 3
 * <p>Yellow: 4
 * <p>Green: 5
 * <p>Blue: 6
 * <p>Violet: 7
 * <p>Grey: 8
 * <p>White: 9
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Determine the digit of the given color band on a resistor.
 * @func
 * @param {string} color Color of the resistor band.
 * @returns {number} The digit that the color encodes.
 */
export const colorCode = (color) => {
  return COLORS.indexOf(color);
};

/**
 * Color bands
 * @const {string[]}
 */
export const COLORS = [
  'black',
  'brown',
  'red',
  'orange',
  'yellow',
  'green',
  'blue',
  'violet',
  'grey',
  'white'
];
