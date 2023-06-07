package entities

import (
	"github.com/Steelstone3/Star-Mechs/components/economy"
	"github.com/Steelstone3/Star-Mechs/components/names"
)

type Commander struct {
	Name              names.PersonName
	Currency          economy.Currency
	Mechs             []Mech
	CurrentStarSystem StarSystem
}

func (commander *Commander) UpdateCurrentStarSystem(starSystem StarSystem) {
	commander.CurrentStarSystem = starSystem
}
