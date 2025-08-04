// cli/bpm/search.go
package bpm

import (
    "encoding/json"
    "errors"
    "fmt"
    "net/http"
    "net/url"
)

// PackageInfo represents minimal package info returned by registry search.
type PackageInfo struct {
    Name        string `json:"name"`
    Version     string `json:"version"`
    Description string `json:"description"`
    Author      string `json:"author"`
}

// SearchPackages searches for packages matching the query in remote registry.
func SearchPackages(query string) ([]PackageInfo, error) {
    if query == "" {
        return nil, errors.New("search query cannot be empty")
    }

    baseURL := "https://registry.bpluslang.org/api/v1/packages/search"
    fullURL := fmt.Sprintf("%s?q=%s", baseURL, url.QueryEscape(query))

    resp, err := http.Get(fullURL)
    if err != nil {
        return nil, fmt.Errorf("failed to perform search request: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        return nil, fmt.Errorf("search failed with status: %s", resp.Status)
    }

    var results []PackageInfo
    decoder := json.NewDecoder(resp.Body)
    if err := decoder.Decode(&results); err != nil {
        return nil, fmt.Errorf("failed to decode search results: %w", err)
    }

    return results, nil
}
