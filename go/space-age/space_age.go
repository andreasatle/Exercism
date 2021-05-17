// Package space - Given an age in seconds, calculate how old someone would be on:
//
//  Mercury: orbital period 0.2408467 Earth years
//  Venus: orbital period 0.61519726 Earth years
//  Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
//  Mars: orbital period 1.8808158 Earth years
//  Jupiter: orbital period 11.862615 Earth years
//  Saturn: orbital period 29.447498 Earth years
//  Uranus: orbital period 84.016846 Earth years
//  Neptune: orbital period 164.79132 Earth years
//
// So if you were told someone were 1,000,000,000 seconds old,
// you should be able to say that they're 31.69 Earth-years old.
package space

const (
	Earth_year_in_seconds = 31557600.0
	Earth_year_ratio      = 1.0
	Mercury_year_ratio    = 0.2408467
	Venus_year_ratio      = 0.61519726
	Mars_year_ratio       = 1.8808158
	Jupiter_year_ratio    = 11.862615
	Saturn_year_ratio     = 29.447498
	Uranus_year_ratio     = 84.016846
	Neptune_year_ratio    = 164.79132
)

type Planet string

func Age(seconds float64, planet Planet) float64 {
	earth_year := seconds / Earth_year_in_seconds
	switch planet {
	case "Mercury":
		return earth_year / Mercury_year_ratio
	case "Venus":
		return earth_year / Venus_year_ratio
	case "Mars":
		return earth_year / Mars_year_ratio
	case "Jupiter":
		return earth_year / Jupiter_year_ratio
	case "Saturn":
		return earth_year / Saturn_year_ratio
	case "Uranus":
		return earth_year / Uranus_year_ratio
	case "Neptune":
		return earth_year / Neptune_year_ratio
	default:
		return earth_year
	}

}
