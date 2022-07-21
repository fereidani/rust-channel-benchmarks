package main

import (
	"fmt"
	"runtime"
	"strings"
	"time"
)

const MESSAGES = 100000
const THREADS = 4
const MIN_BENCH_TIME = 500

func NewBig(i uint) [4]uint {
	return [4]uint{i, i, i, i}
}
func NewUsize(i uint) uint {
	return i
}

func spsc_empty(cap int) {
	var c = make(chan struct{}, cap)
	var done = make(chan bool)

	go func() {
		for i := 1; i < MESSAGES+1; i++ {
			c <- struct{}{}
		}
		done <- true
	}()

	for i := 0; i < MESSAGES; i++ {
		<-c
	}

	<-done
}

func spsc(cap int) {
	var c = make(chan uint, cap)
	var done = make(chan bool)

	go func() {
		for i := uint(1); i < MESSAGES+1; i++ {
			c <- NewUsize(i)
		}
		done <- true
	}()

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v < 1 {
			panic("invalid_value")
		}
	}

	<-done
}

func spsc_big(cap int) {
	var c = make(chan [4]uint, cap)
	var done = make(chan bool)

	go func() {
		for i := uint(1); i < MESSAGES+1; i++ {
			c <- NewBig(i)
		}
		done <- true
	}()

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v[0] < 1 {
			panic("invalid_value")
		}
	}

	<-done
}

func seq_empty(cap int) {
	var c = make(chan struct{}, cap)

	for i := 1; i < MESSAGES+1; i++ {
		c <- struct{}{}
	}

	for i := 0; i < MESSAGES; i++ {
		<-c
	}
}

func mpsc_empty(cap int) {
	var c = make(chan struct{}, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 1; i < MESSAGES/THREADS+1; i++ {
				c <- struct{}{}
			}
			done <- true
		}()
	}

	for i := 0; i < MESSAGES; i++ {
		<-c
	}

	for t := 0; t < THREADS; t++ {
		<-done
	}
}

func mpmc_empty(cap int) {
	var c = make(chan struct{}, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 1; i < MESSAGES/THREADS+1; i++ {
				c <- struct{}{}
			}
			done <- true
		}()

	}

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS; i++ {
				<-c
			}
			done <- true
		}()
	}

	for t := 0; t < THREADS; t++ {
		<-done
		<-done
	}
}

func seq(cap int) {
	var c = make(chan uint, cap)

	for i := uint(1); i < MESSAGES+1; i++ {
		c <- NewUsize(i)
	}

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v < 1 {
			panic("invalid_value")
		}
	}
}

func mpsc(cap int) {
	var c = make(chan uint, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := uint(1); i < MESSAGES/THREADS+1; i++ {
				c <- NewUsize(i)
			}
			done <- true
		}()
	}

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v < 1 {
			panic("invalid_value")
		}
	}

	for t := 0; t < THREADS; t++ {
		<-done
	}
}

func mpmc(cap int) {
	var c = make(chan uint, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := uint(1); i < MESSAGES/THREADS+1; i++ {
				c <- NewUsize(i)
			}
			done <- true
		}()

	}

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS; i++ {
				v := <-c
				if v < 1 {
					panic("invalid_value")
				}
			}
			done <- true
		}()
	}

	for t := 0; t < THREADS; t++ {
		<-done
		<-done
	}
}

func seq_big(cap int) {
	var c = make(chan [4]uint, cap)

	for i := uint(1); i < MESSAGES+1; i++ {
		c <- NewBig(i)
	}

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v[0] < 1 {
			panic("invalid_value")
		}
	}
}

func mpsc_big(cap int) {
	var c = make(chan [4]uint, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := uint(1); i < MESSAGES/THREADS+1; i++ {
				c <- NewBig(i)
			}
			done <- true
		}()
	}

	for i := 0; i < MESSAGES; i++ {
		v := <-c
		if v[0] < 1 {
			panic("invalid_value")
		}
	}

	for t := 0; t < THREADS; t++ {
		<-done
	}
}

func mpmc_big(cap int) {
	var c = make(chan [4]uint, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := uint(1); i < MESSAGES/THREADS+1; i++ {
				c <- NewBig(i)
			}
			done <- true
		}()

	}

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS; i++ {
				v := <-c
				if v[0] < 1 {
					panic("invalid_value")
				}
			}
			done <- true
		}()
	}

	for t := 0; t < THREADS; t++ {
		<-done
		<-done
	}
}

func run(name string, f func(int), cap int) {
	var sum_elapsed = time.Duration(0)
	var count = time.Duration(0)
	for {
		var now = time.Now()
		f(cap)
		var elapsed = time.Since(now)
		sum_elapsed += elapsed
		count++
		if sum_elapsed >= time.Millisecond*MIN_BENCH_TIME {
			break
		}
	}
	fmt.Printf("%s,%d\n", name, sum_elapsed/count)
}

func main() {

	fmt.Println(strings.Replace(runtime.Version(), "go", "go chan", 1))

	run("bounded0_mpmc(empty)", mpmc_empty, 0)
	run("bounded0_mpsc(empty)", mpsc_empty, 0)
	run("bounded0_spsc(empty)", spsc_empty, 0)

	run("bounded1_mpmc(empty)", mpmc_empty, 1)
	run("bounded1_mpsc(empty)", mpsc_empty, 1)
	run("bounded1_spsc(empty)", spsc_empty, 1)

	run("bounded_mpmc(empty)", mpmc_empty, MESSAGES)
	run("bounded_mpsc(empty)", mpsc_empty, MESSAGES)

	run("bounded_seq(empty)", seq_empty, MESSAGES)
	run("bounded_spsc(empty)", spsc_empty, MESSAGES)

	// usize test
	run("bounded0_mpmc(usize)", mpmc, 0)
	run("bounded0_mpsc(usize)", mpsc, 0)
	run("bounded0_spsc(usize)", spsc, 0)

	run("bounded1_mpmc(usize)", mpmc, 1)
	run("bounded1_mpsc(usize)", mpsc, 1)
	run("bounded1_spsc(usize)", spsc, 1)

	run("bounded_mpmc(usize)", mpmc, MESSAGES)
	run("bounded_mpsc(usize)", mpsc, MESSAGES)

	run("bounded_seq(usize)", seq, MESSAGES)
	run("bounded_spsc(usize)", spsc, MESSAGES)

	// big test
	run("bounded0_mpmc(big)", mpmc_big, 0)
	run("bounded0_mpsc(big)", mpsc_big, 0)
	run("bounded0_spsc(big)", spsc_big, 0)

	run("bounded1_mpmc(big)", mpmc_big, 1)
	run("bounded1_mpsc(big)", mpsc_big, 1)
	run("bounded1_spsc(big)", spsc_big, 1)

	run("bounded_mpmc(big)", mpmc_big, MESSAGES)
	run("bounded_mpsc(big)", mpsc_big, MESSAGES)

	run("bounded_seq(big)", seq_big, MESSAGES)
	run("bounded_spsc(big)", spsc_big, MESSAGES)

}
