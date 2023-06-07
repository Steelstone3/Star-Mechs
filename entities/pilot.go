package entities

import (
	"github.com/Steelstone3/Star-Mechs/components/names"
	"github.com/Steelstone3/Star-Mechs/components/pilot"
)

//TODO This will feed in to the combat effectiveness of the mech
type Pilot struct {
	Name names.PersonName
	Statistics pilot.Statistics
}