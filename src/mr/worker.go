package mr

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"strconv"
	"sync"
)

const workerCount = 5

//
// mapF functions return a slice of KeyValue.
//
type KeyValue struct {
	Key   string
	Value string
}

//
// mapF or Reduce task.
//
type MapOrReduceTask struct {
	Phase      jobPhase
	FileName   string // map file name(only for map phase).
	TaskNumber int    // this task's index in the current phase.
	// NumOtherPhase is the total number of tasks in other phase; mappers
	// need this to compute the number of output bins, and reducers needs
	// this to know how many input files to collect.
	NumOtherPhase int
}

// MapReduceWorker holds the state for a server waiting for DoTask or Shutdown RPCs.
type MapReduceWorker struct {
	sync.Mutex

	name     string
	mapf     func(string, string) []KeyValue
	reducef  func(string, []string) string
	listener net.Listener
}

// Shutdown is called by the master when all work has been completed.
func (worker *MapReduceWorker) Shutdown(_ *struct{}, res *ShutdownReply) error {
	debug("Shutdown %s\n", worker.name)
	worker.Lock()
	defer worker.Unlock()
	err := worker.listener.Close()
	if err == nil {
		res.IsDown = true
	} else {
		res.IsDown = false
	}
	return nil
}

//
// main/mrworker.go calls this function.
//
func Worker(mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) {
	workerNamePrefix := "mr-socket-worker"
	for i := 0; i < workerCount; i++ {
		initWorker(workerNamePrefix+strconv.Itoa(i), MasterSocketName, mapf, reducef)
	}
}

// init the worker and start it server.
func initWorker(workerName string, masterAddress string, mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) {
	debug("RunWorker %s\n", workerName)
	worker := new(MapReduceWorker)
	worker.name = workerName
	worker.mapf = mapf
	worker.reducef = reducef
	rpc.Register(worker)
	os.Remove(workerName)
	listener, e := net.Listen("unix", workerName)
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(listener, nil)
	worker.listener = listener
	register(masterAddress, workerName)
	debug("RunWorker %s exit\n", workerName)
}

// Tell the master we exist and ready to work.
func register(masterAddress string, workerName string) {
	args := new(RegisterArgs)
	args.WorkerName = workerName
	ok := call(masterAddress, "Master.WorkerRegister", args, new(struct{}))
	if ok == false {
		fmt.Printf("Register: RPC %s register error\n", masterAddress)
	}
}
