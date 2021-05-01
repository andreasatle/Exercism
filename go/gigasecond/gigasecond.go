// Package gigasecond adds 1Gs to a time.Time.
package gigasecond

import "time"

// AddGigasecond returns the time with 1Gs added to it.
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Duration(1000000000 * time.Second))
}
