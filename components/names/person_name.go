package names

import (
	"github.com/Steelstone3/Star-Mechs/presenters"
	"github.com/Steelstone3/Star-Mechs/systems/generators"
)

type PersonName struct {
	FirstName string
	LastName  string
}

func ConstructRandomPersonName(seed int64) PersonName {
	firstNames := []string{"Bob", "Harry", "Harold", "Charlie", "Alex", "Sally", "Chloe", "Sophie", "Robert", "Max"}
	lastNames := []string{"Jones", "Smith", "Fitzgerald"}

	firstName := generators.GetRandomString(seed, firstNames)
	lastName := generators.GetRandomString(seed, lastNames)

	return PersonName{
		FirstName: firstName,
		LastName:  lastName,
	}
}

func (personName PersonName) DisplayPersonName() {
	name := "{FirstName} {Last Name}"

	name = presenters.StringReplace(name, "{FirstName}", personName.FirstName)
	name = presenters.StringReplace(name, "{LastName}", personName.LastName)

	presenters.PrintLine(name)
}
