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

const (
	prodRpcPath  string = "go_rpc"
	debugRpcPath string = "debug_rpc"
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
func call(address string, rpcname string, path string, args interface{}, reply interface{}) (bool, error) {
	debug("RPC: A call to %s name %s\n", address, rpcname)
	client, err := rpc.DialHTTPPath("unix", address, path)
	if err != nil {
		log.Printf("dialing: %s\n", err)
		return false, err
	}
	defer client.Close()

	err = client.Call(rpcname, args, reply)
	if err == nil {
		log.Printf("RPC: A successful call to %s name %s\n", address, rpcname)
		return true, nil
	}
	log.Printf("RPC: A failed to %s name %s\n", address, rpcname)
	log.Printf("RPC call failed %s:", err)
	return false, nil
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
	return "mrtmp-" + strconv.Itoa(mapTask) + "-" + strconv.Itoa(reduceTask)
}

// gen output file name.
func outputFileName(reduceTask int) string {
	return "mr-out-" + strconv.Itoa(reduceTask)
}
