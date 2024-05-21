package lib

import (
	"os"
)

func Contains(s string, list ...string) bool {
	for _, item := range list {
		if item == s {
			return true
		}
	}
	return false
}

func FileExists(fileName string) bool {
	_, err := os.Stat(fileName)
	return err == nil
}
