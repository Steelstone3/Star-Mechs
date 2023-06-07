package pilot

import "github.com/Steelstone3/Star-Mechs/systems/generators"

type Statistics struct {
	Expierence Expierence
	Strategy Expierence
	Teamwork Expierence
	Bravery Expierence
}

func ConstructPilotStatistics() Statistics {
	return Statistics{
		Expierence: constructRandomExpierence(generators.GenerateSeed()),
		Strategy:   constructRandomExpierence(generators.GenerateSeed()),
		Teamwork:   constructRandomExpierence(generators.GenerateSeed()),
		Bravery:    constructRandomExpierence(generators.GenerateSeed()),
	}
}