package main

import (
	"fmt"
	"runtime"
	"strings"
	"time"
)

const MESSAGES = 5000000
const THREADS = 4
const THREADS_SELECT = 4

type Message = [1]int

func NewMessage(i int) Message {
	return Message{i}
}

func seq(cap int) {
	var c = make(chan Message, cap)

	for i := 0; i < MESSAGES; i++ {
		c <- NewMessage(i)
	}

	for i := 0; i < MESSAGES; i++ {
		<-c
	}
}

func spsc(cap int) {
	var c = make(chan Message, cap)
	var done = make(chan bool)

	go func() {
		for i := 0; i < MESSAGES; i++ {
			c <- NewMessage(i)
		}
		done <- true
	}()

	for i := 0; i < MESSAGES; i++ {
		<-c
	}

	<-done
}

func mpsc(cap int) {
	var c = make(chan Message, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS; i++ {
				c <- NewMessage(i)
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

func mpmc(cap int) {
	var c = make(chan Message, cap)
	var done = make(chan bool)

	for t := 0; t < THREADS; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS; i++ {
				c <- NewMessage(i)
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

func select_rx(cap int) {

	var c0 = make(chan Message, cap)
	var c1 = make(chan Message, cap)
	var c2 = make(chan Message, cap)
	var c3 = make(chan Message, cap)
	var done = make(chan bool)

	var producer = func(c chan Message) {
		for i := 0; i < MESSAGES/THREADS_SELECT; i++ {
			c <- NewMessage(i)
		}
		done <- true
	}
	go producer(c0)
	go producer(c1)
	go producer(c2)
	go producer(c3)

	for i := 0; i < MESSAGES; i++ {
		select {
		case <-c0:
		case <-c1:
		case <-c2:
		case <-c3:
		}
	}

	for t := 0; t < THREADS; t++ {
		<-done
	}
}

func select_both(cap int) {

	var c0 = make(chan Message, cap)
	var c1 = make(chan Message, cap)
	var c2 = make(chan Message, cap)
	var c3 = make(chan Message, cap)
	var done = make(chan bool)

	var producer = func(c0 chan Message, c1 chan Message, c2 chan Message, c3 chan Message) {
		for i := 0; i < MESSAGES/THREADS_SELECT; i++ {
			select {
			case c0 <- NewMessage(i):
			case c1 <- NewMessage(i):
			case c2 <- NewMessage(i):
			case c3 <- NewMessage(i):
			}
		}
		done <- true
	}
	go producer(c0, c1, c2, c3)
	go producer(c0, c1, c2, c3)
	go producer(c0, c1, c2, c3)
	go producer(c0, c1, c2, c3)

	for t := 0; t < THREADS_SELECT; t++ {
		go func() {
			for i := 0; i < MESSAGES/THREADS_SELECT; i++ {
				select {
				case <-c0:
				case <-c1:
				case <-c2:
				case <-c3:
				}
			}
			done <- true
		}()
	}

	for t := 0; t < THREADS_SELECT; t++ {
		<-done
		<-done
	}
}

func run(name string, f func(int), cap int) {
	var now = time.Now()
	f(cap)
	var elapsed = time.Since(now)
	fmt.Printf("%s,%d\n", name, elapsed)
}

func main() {

	fmt.Println(strings.Replace(runtime.Version(), "go", "go chan", 1))

	run("bounded0_mpmc", mpmc, 0)
	run("bounded0_mpsc", mpsc, 0)
	run("bounded0_select_both", select_both, 0)
	run("bounded0_select_rx", select_rx, 0)
	run("bounded0_spsc", spsc, 0)

	run("bounded1_mpmc", mpmc, 1)
	run("bounded1_mpsc", mpsc, 1)
	run("bounded1_select_both", select_both, 1)
	run("bounded1_select_rx", select_rx, 1)
	run("bounded1_spsc", spsc, 1)

	run("bounded_mpmc", mpmc, MESSAGES)
	run("bounded_mpsc", mpsc, MESSAGES)
	run("bounded_select_both", select_both, MESSAGES)
	run("bounded_select_rx", select_rx, MESSAGES)
	run("bounded_seq", seq, MESSAGES)
	run("bounded_spsc", spsc, MESSAGES)

}
