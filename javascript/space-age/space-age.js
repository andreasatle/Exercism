/**
 * Given an age in seconds, calculate how old someone would be on:
 * 
 * <p>Mercury: orbital period 0.2408467 Earth years
 * <p>Venus: orbital period 0.61519726 Earth years
 * <p>Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
 * <p>Mars: orbital period 1.8808158 Earth years
 * <p>Jupiter: orbital period 11.862615 Earth years
 * <p>Saturn: orbital period 29.447498 Earth years
 * <p>Uranus: orbital period 84.016846 Earth years
 * <p>Neptune: orbital period 164.79132 Earth years
 * <p>So if you were told someone were 1,000,000,000 seconds old,
 * you should be able to say that they're 31.69 Earth-years old.
 * @file
 * @author Andreas Atle, atle.andreas@gmail.com
 */
// Converter from earth-seconds to planet-years
const EARTH_YEAR_IN_SECONDS = 31557600;
const EARTH_SECONDS = {
  'mercury': EARTH_YEAR_IN_SECONDS * 0.2408467,
  'venus':   EARTH_YEAR_IN_SECONDS * 0.61519726,
  'earth':   EARTH_YEAR_IN_SECONDS * 1.0,
  'mars':    EARTH_YEAR_IN_SECONDS * 1.8808158,
  'jupiter': EARTH_YEAR_IN_SECONDS * 11.862615,
  'saturn':  EARTH_YEAR_IN_SECONDS * 29.447498,
  'uranus':  EARTH_YEAR_IN_SECONDS * 84.016846,
  'neptune': EARTH_YEAR_IN_SECONDS * 164.79132
};

/**
 * Convert a number of seconds to planet years.
 * @func
 * @param {string} planet A planet in the solar system. {mercury, venus, earth, mars, jupiter, saturn, uranus, neptune}
 * @param {number} seconds Number of seconds to be converted.
 * @returns {number} The converted time in planet years.
 */
export const age = (planet, seconds) => {
  return +(seconds/EARTH_SECONDS[planet]).toFixed(2);
};
