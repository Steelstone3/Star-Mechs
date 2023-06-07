package main

import (
	"github.com/Steelstone3/Star-Mechs/entities"
	"github.com/Steelstone3/Star-Mechs/systems"
)

func main() {
	// TODO Create a commander with a starting location and cash
	game := entities.ConstructGame()

	// TODO Buy initial mechs
	game = systems.BuyMechs(game)

	// TODO display a bunch of star systems with a cost to travel to them
	game = systems.Explore(game)
	game = systems.GenerateMissions(game)
}
