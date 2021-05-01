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

export const age = (planet, seconds) => {
  return +(seconds/EARTH_SECONDS[planet]).toFixed(2);
};
