package mr

import (
	"fmt"
	"log"
	"math/rand"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"strconv"
	"sync"
	"time"
)

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
	wg       sync.WaitGroup
}

// Shutdown is called by the master when all work has been completed.
func (worker *MapReduceWorker) Shutdown(_ *struct{}, res *ShutdownReply) error {
	worker.Lock()
	defer worker.Unlock()
	debug("Shutdown: worker  %s\n", worker.name)
	res.IsDown = true
	worker.wg.Done()
	return nil
}

// Do the map or reduce work.
func (worker *MapReduceWorker) DoTask(task *MapOrReduceTask, _ *struct{}) error {
	log.Printf("%s: given %v task #%d on file %s (nios: %d)\n",
		worker.name, task.Phase, task.TaskNumber, task.FileName, task.NumOtherPhase)
	switch task.Phase {
	case mapPhase:
		doMap(task.TaskNumber, task.FileName, task.NumOtherPhase, worker.mapf)
	case reducePhase:
		doReduce(task.TaskNumber, outputFileName(task.TaskNumber), task.NumOtherPhase, worker.reducef)
	}
	log.Printf("%s: %v task #%d done\n", worker.name, task.Phase, task.TaskNumber)
	return nil
}

//
// main/mrworker.go calls this function.
//
func Worker(mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) {
	workerNamePrefix := "mr-socket-worker"
	rand.Seed(time.Now().UnixNano())
	pid := os.Getpid()
	i := rand.Int()
	worker := initWorker(workerNamePrefix+strconv.Itoa(i)+strconv.Itoa(pid), MasterSocketName, mapf, reducef)
	worker.wg.Wait()
	err := worker.listener.Close()
	if err == nil {
		debug("RunWorker %s exit\n", worker.name)
	} else {
		log.Printf("RunWorker: %s exit failed", worker.name)
	}
}

// init the worker and start it server.
func initWorker(workerName string, masterAddress string, mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) *MapReduceWorker {
	debug("RunWorker %s\n", workerName)
	worker := new(MapReduceWorker)
	worker.name = workerName
	worker.mapf = mapf
	worker.reducef = reducef
	worker.wg.Add(1)
	rpcServer := rpc.NewServer()
	rpcServer.Register(worker)
	rpcServer.HandleHTTP(worker.name+prodRpcPath, worker.name+debugRpcPath)
	os.Remove(worker.name)
	listener, e := net.Listen("unix", worker.name)
	if e != nil {
		log.Fatal("listener error:", e)
	}
	go http.Serve(listener, nil)
	worker.listener = listener
	register(masterAddress, masterAddress+prodRpcPath, workerName)
	return worker
}

// Tell the master we exist and ready to work.
func register(masterAddress string, path string, workerName string) {
	args := new(RegisterArgs)
	args.WorkerName = workerName
	ok, err := call(masterAddress, "Master.WorkerRegister", path, args, new(struct{}))
	if ok == false || err != nil {
		fmt.Printf("Register: RPC %s register error\n", masterAddress)
	}
}
