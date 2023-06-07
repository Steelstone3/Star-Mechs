package entities

import (
	"github.com/Steelstone3/Star-Mechs/components/economy"
	"github.com/Steelstone3/Star-Mechs/components/names"
)

type StarSystem struct {
	Name       names.StarName
	Planets    []names.PlanetName
	TravelCost economy.Currency
}
