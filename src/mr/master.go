package mr

import (
	"fmt"
	"log"
	"sync"
)
import "net"
import "os"
import "net/rpc"
import "net/http"

const MasterSocketName string = "mr-socket-master7"

// Master holds all the state that the master needs to keep track of.
type Master struct {
	sync.Mutex

	doneChannel chan bool
	// protected by the mutex.
	newCond  *sync.Cond // signals when WorkerRegister() adds to workers[].
	workers  []string   // each worker's UNIX-domain socket name -- its RPC address.
	files    []string   // Input files.
	nReduce  int        // Number of reduce partitions.
	listener net.Listener
}

//
// start a thread that listens for RPCs from worker.go
//
func (master *Master) server() {
	rpc.Register(master)
	rpc.HandleHTTP()
	os.Remove("mr-socket")
	listener, e := net.Listen("unix", MasterSocketName)
	if e != nil {
		log.Fatal("listener error:", e)
	}
	go http.Serve(listener, nil)
	master.listener = listener
}

//
// main/mrmaster.go calls Done() periodically to find out
// if the entire job has finished.
//
func (master *Master) Done() bool {
	isDown := master.run()
	return isDown
}

//
// create a Master.
// main/mrmaster.go calls this function.
//
func MakeMaster(files []string, nReduce int) *Master {
	master := new(Master)
	master.newCond = sync.NewCond(master)
	master.doneChannel = make(chan bool)
	master.files = files
	master.nReduce = nReduce
	master.server()
	return master
}

// process the map and reduce work.
func (master *Master) run() bool {
	fmt.Print("Starting Map/Reduce task\n")
	master.process(mapPhase)
	master.process(reducePhase)
	isDown := master.killWorkers()
	if isDown == false {
		log.Fatal("Can not shutdown all worker\n")
	} else {
		log.Print("All worker shutdown\n")
	}
	err := master.listener.Close()
	if err != nil {
		log.Fatal("The master rpc close failed\n")
	}
	return isDown
}

// WorkerRegister is an RPC method that is called by workers after they have started
// up to report that they are ready to receive tasks.
func (master *Master) WorkerRegister(args *RegisterArgs, _ *struct{}) error {
	master.Lock()
	defer master.Unlock()
	debug("WorkerRegister: worker %s\n", args.WorkerName)
	master.workers = append(master.workers, args.WorkerName)
	// tell forwardWorker() that there's a new workers[] entry.
	master.newCond.Broadcast()
	return nil
}

// helper function that sends information about all existing
// and newly registered workers to channel ch. process()
// reads ch to learn about workers.
func (master *Master) forwardWorker(ch chan string) {
	i := 0
	for {
		master.Lock()
		if len(master.workers) > i {
			// there's a worker that we haven't told process() about.
			worker := master.workers[i]
			go func() {
				ch <- worker
				debug("Forward: worker %s\n", worker)
			}() // send without holding the lock.
			i = i + 1
		} else {
			// wait for WorkerRegister() to add an entry to workers[]
			// in response to an RPC from a new worker.
			master.newCond.Wait()
		}
		master.Unlock()
	}
}

// killWorkers cleans up all workers by sending each one a Shutdown RPC.
// It also returns the kill result.
func (master *Master) killWorkers() bool {
	master.Lock()
	defer master.Unlock()
	isDown := true
	for _, w := range master.workers {
		debug("Master: shutdown worker %s\n", w)
		var reply ShutdownReply
		ok := call(w, "MapReduceWorker.Shutdown", new(struct{}), &reply)
		if ok == false || reply.IsDown == false {
			fmt.Printf("Master: RPC %s shutdown error\n", w)
			isDown = false
		}
	}
	return isDown
}

// helper for process different phase.
func (master *Master) process(phase jobPhase) {
	ch := make(chan string)
	go master.forwardWorker(ch)
	scheduleWorker(master.files, master.nReduce, phase, ch)
}

// scheduleWorker will process the work form registerChan.
func scheduleWorker(mapFileNames []string, nReduce int, phase jobPhase, registerChan chan string) {
	log.Printf("Schedule: start %v\n", phase)
	var numberOfTasks int
	var numberOfOther int // number of inputs (for reduce) or outputs (for map).
	switch phase {
	case mapPhase:
		numberOfTasks = len(mapFileNames)
		numberOfOther = nReduce
	case reducePhase:
		numberOfTasks = nReduce
		numberOfOther = len(mapFileNames)
	}

	log.Printf("Schedule: %v %v tasks (%d I/Os)\n", numberOfTasks, phase, numberOfOther)

	// All numberOfTasks tasks have to be scheduled on workers, and only once all of
	// them have been completed successfully should the function return.
	// Remember that workers may fail, and that any given worker may finish
	// multiple tasks.

	// process will wait until all worker has done their jobs.
	var wg sync.WaitGroup

	// RPC httpCall parameter
	var task MapOrReduceTask
	task.NumOtherPhase = numberOfOther
	task.Phase = phase

	// task id will get from this channel.
	var taskChan = make(chan int)
	go func() {
		for i := 0; i < numberOfTasks; i++ {
			wg.Add(1)
			taskChan <- i
			debug("Schedule: push task %v into taskChan\n", i)
		}
		// wait all workers have done their job, then close taskChan.
		wg.Wait()
		debug("Schedule: %s task all down so close the taskChan\n", phase)
		close(taskChan)
	}()

	// assign all task to worker.
	for i := range taskChan {
		debug("Schedule: get task %v from taskChan\n", i)
		// get a worker from register channel.
		worker := <-registerChan
		debug("Schedule: assign task %v to %v in %v phase\n", i, worker, phase)
		task.TaskNumber = i
		if phase == mapPhase {
			task.FileName = mapFileNames[i]
		}

		// Note: must use parameter.
		go func(worker string, task MapOrReduceTask) {
			if call(worker, "MapReduceWorker.DoTask", &task, nil) {
				// only successful httpCall will httpCall wg.Done().
				wg.Done()

				// put idle worker back to register channel.
				log.Printf("Schedule: worker %s finished %s task %v", worker, phase,
					task.TaskNumber)
				registerChan <- worker
			} else {
				log.Printf("Schedule: assign %s task %v to %s failed", phase,
					task.TaskNumber, worker)

				// put failed task back to task channel.
				taskChan <- task.TaskNumber
			}
		}(worker, task)
	}
	log.Printf("Schedule: %v phase done\n", phase)
}
