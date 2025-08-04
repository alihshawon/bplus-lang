// cli/bpm/utils/network.go
package bpm

import (
    "fmt"
    "io"
    "net/http"
    "time"
)

// HttpGet performs an HTTP GET request with a timeout and returns the response body.
func HttpGet(url string) (io.ReadCloser, error) {
    client := &http.Client{
        Timeout: 15 * time.Second,
    }

    resp, err := client.Get(url)
    if err != nil {
        return nil, fmt.Errorf("HTTP GET request failed: %w", err)
    }

    if resp.StatusCode != http.StatusOK {
        resp.Body.Close()
        return nil, fmt.Errorf("HTTP GET request returned status: %s", resp.Status)
    }

    return resp.Body, nil
}
