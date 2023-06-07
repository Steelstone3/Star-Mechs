package names

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructRandomPersonName(t *testing.T) {
	// Given
	seed := int64(1234)
	expectedPersonName := PersonName{
		FirstName: "Harold",
		LastName:  "Fitzgerald",
	}

	// When
	personName := ConstructRandomPersonName(seed)

	// Then
	assert.Equal(t, expectedPersonName, personName)
}