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

	// TODO Until all mechs are destoryed or all money is used up
	for {
		// TODO display a bunch of star systems with a cost to travel to them
		game = systems.Explore(game)

		// TODO display a bunch of missions for the system
		// TODO each mission has a payout
		// TODO There is a main payout that can be negotiated upwards
		// TODO Mechs will take damage that cost to repair this will come out of the insurance cost
		// TODO A difficulty rating will also be calculated
		// TODO systems in heavy conflict will pay more as will multi missions
		// TODO however these missions are harder
		game = systems.GenerateMissions(game)

		game = systems.SelectMission(game)

		// TODO options to do the following
		game = systems.BuyMechs(game)
		game = systems.RepairMechs(game)

		// TODO then mission starts
		game = systems.StartMission(game)

		// TODO once mission ends payout is recieved and a return to exploration mode
		game = systems.EndMission(game)
	}
}
