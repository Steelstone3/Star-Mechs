package entities

import "github.com/Steelstone3/Star-Mechs/components/economy"

type Mech struct {
	Cost economy.Currency
	Pilot Pilot
}