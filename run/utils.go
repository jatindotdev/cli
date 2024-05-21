package main

import (
	"os"
)

func contains(s string, list []string) bool {
	for _, item := range list {
		if item == s {
			return true
		}
	}
	return false
}

func fileExists(fileName string) bool {
	_, err := os.Stat(fileName)
	return err == nil
}
