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

//
// send an RPC request to the address, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func call(address string, rpcname string, isHttp bool, args interface{}, reply interface{}) bool {
	debug("RPC: A call to %s name %s\n", address, rpcname)
	var client *rpc.Client
	var err error
	if isHttp {
		client, err = rpc.DialHTTP("unix", address)

	} else {
		client, err = rpc.Dial("unix", address)
	}
	if err != nil {
		log.Printf("dialing: %s\n", err)
		return false
	}
	defer client.Close()

	err = client.Call(rpcname, args, reply)
	if err == nil {
		log.Printf("RPC: A successful call to %s name %s\n", address, rpcname)
		return true
	}
	log.Printf("RPC: A failed to %s name %s\n", address, rpcname)
	log.Printf("RPC call failed %s:", err)
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
