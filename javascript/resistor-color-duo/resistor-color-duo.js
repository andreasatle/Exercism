/**
 * If you want to build something using a Raspberry Pi, you'll probably use resistors. For this exercise, you need to know two things about them:
 * 
 * Each resistor has a resistance value.
 * Resistors are small - so small in fact that if you printed the resistance value on them, it would be hard to read.
 * To get around this problem, manufacturers print color-coded bands onto the resistors to denote their resistance values. Each band has a position and a numeric value.
 * 
 * The first 2 bands of a resistor have a simple encoding scheme: each color maps to a single number. For example, if they printed a brown band (value 1) followed by a green band (value 5), it would translate to the number 15.
 * 
 * In this exercise you are going to create a helpful program so that you don't have to remember the values of the bands. The program will take color names as input and output a two digit number, even if the input is more than two colors!
 * 
 * The band colors are encoded as follows:
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
 * From the example above: brown-green should return 15 brown-green-violet should return 15 too, ignoring the third color.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */

/**
 * Decode the first two color bands on a resistor.
 * @param {string[]} colors An array of colors of atleast length 2.
 * @returns The decoded color.
 */
export const decodedValue = (colors) => {
  let num = 0;
  for (let i = 0; i < 2; i++) {
    num *= 10;
    num += COLORS.indexOf(colors[i]);
  }
  return num;
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
