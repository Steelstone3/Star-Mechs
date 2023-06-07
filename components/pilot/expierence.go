package pilot

import "github.com/Steelstone3/Star-Mechs/systems/generators"

type Level string

const (
	High     Level = "Highly"
	Moderate Level = "Moderately"
	Adequate Level = "Adequately"
	Low      Level = "Lowly"
)

type Expierence struct {
	Level Level
}

func constructRandomExpierence(seed int64) Expierence {
	expierenceLevels := []string{ string(High), string(Moderate), string(Adequate), string(Low) }
	level := generators.GetRandomString(seed, expierenceLevels)

	return Expierence{
		Level: toLevel(level),
	}
}

func toLevel(level string) Level {
	switch level {
	case "Highly":
		return High
	case "Moderately":
		return Moderate
	case "Adequately":
		return Adequate
	case "Lowly":
		return Low
	default :
		panic("Invalid expierence level")
	}
}