package main

import (
	"os"
	"path/filepath"
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

func getExtension(fileName string) string {
	ext := filepath.Ext(fileName)
	switch ext {
	case ".c":
		return C
	case ".cpp":
		return Cpp
	case ".dart":
		return Dart
	case ".swift":
		return Swift
	case ".py":
		return Python
	case ".java":
		return Java
	case ".go":
		return Go
	default:
		return JavaScript
	}
}
