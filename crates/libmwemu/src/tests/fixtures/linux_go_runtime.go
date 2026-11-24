package main

import (
	"fmt"
	"os"
	"os/exec"
	"sort"
	"sync"
	"time"
)

func worker(id int, jobs <-chan int, results chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()
	for j := range jobs {
		time.Sleep(time.Millisecond)
		results <- j * j
	}
}

func main() {
	jobs := make(chan int, 16)
	results := make(chan int, 16)
	var wg sync.WaitGroup
	for w := 0; w < 4; w++ {
		wg.Add(1)
		go worker(w, jobs, results, &wg)
	}
	for i := 0; i < 8; i++ {
		jobs <- i
	}
	close(jobs)
	wg.Wait()
	close(results)
	var squares []int
	for r := range results {
		squares = append(squares, r)
	}
	sort.Ints(squares)
	fmt.Println("squares:", squares)
	cmd := exec.Command("/bin/echo", "child")
	cmd.Stdin = os.Stdin
	if _, err := cmd.Output(); err != nil {
		fmt.Println("exec error:", err)
	}
	fmt.Println("done")
	os.Exit(7)
}
