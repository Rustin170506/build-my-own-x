package mr

import (
	"fmt"
	"hash/fnv"
	"log"
	"net/rpc"
	"strconv"
)

// jobPhase indicates whether a task is scheduled as a map or reduce task.
type jobPhase string

const (
	mapPhase    jobPhase = "Map"
	reducePhase jobPhase = "Reduce"
)

// Debugging enabled?
const debugEnabled = true

// debug() will only print if debugEnabled is true
func debug(format string, a ...interface{}) (n int, err error) {
	if debugEnabled {
		n, err = fmt.Printf(format, a...)
	}
	return
}

//@TODO need refactor the call helper.
//
// send an http RPC request to the address, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func httpCall(address string, rpcname string, args interface{}, reply interface{}) bool {
	debug("RPC: A httpCall to %s mame %s\n", address, rpcname)
	c, err := rpc.DialHTTP("unix", address)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		log.Printf("RPC: A successful httpCall to %s mame %s\n", address, rpcname)
		return true
	}

	log.Fatal("RPC httpCall failed:", err)
	return false
}

//
// send an RPC request to the address, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func call(address string, rpcname string, args interface{}, reply interface{}) bool {
	debug("RPC: A call to %s mame %s\n", address, rpcname)
	c, err := rpc.Dial("unix", address)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		log.Printf("RPC: A successful httpCall to %s mame %s\n", address, rpcname)
		return true
	}

	log.Fatal("RPC call failed:", err)
	return false
}

//
// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by mapF.
//
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}

// gen the reduce file name.
func reduceFileName(mapTask int, reduceTask int) string {
	return "mrtmp.-" + strconv.Itoa(mapTask) + "-" + strconv.Itoa(reduceTask)
}

// gen output file name.
func outputFileName(reduceTask int) string {
	return "mr-out-" + strconv.Itoa(reduceTask)
}
