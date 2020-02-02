package mr

import (
	"fmt"
	"log"
	"math/rand"
	"net"
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
	wg       sync.WaitGroup // @TODO need to reconsider the shutdown implement.
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
	i := rand.Intn(int(time.Now().UnixNano()))
	worker := initWorker(workerNamePrefix+strconv.Itoa(i), MasterSocketName, mapf, reducef)
	worker.wg.Wait()
	worker.Lock()
	err := worker.listener.Close()
	if err == nil {
		debug("RunWorker %s exit\n", worker.name)
	} else {
		log.Printf("RunWorker: %s exit failed", worker.name)
	}
	worker.Unlock()
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
	os.Remove(workerName) // only needed for "unix".
	listener, e := net.Listen("unix", workerName)
	if e != nil {
		log.Fatal("RunWorker: worker ", workerName, " error: ", e)
	}
	worker.listener = listener
	register(masterAddress, workerName)
	go func() {
		for {
			conn, err := worker.listener.Accept()
			if err == nil {
				go rpcServer.ServeConn(conn)
			} else {
				log.Printf("RunWorker: accept err %s", err)
				break
			}
		}
	}()
	return worker
}

// Tell the master we exist and ready to work.
func register(masterAddress string, workerName string) {
	args := new(RegisterArgs)
	args.WorkerName = workerName
	ok := call(masterAddress, "Master.WorkerRegister", true, args, new(struct{}))
	if ok == false {
		fmt.Printf("Register: RPC %s register error\n", masterAddress)
	}
}
