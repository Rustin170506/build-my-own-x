package mr

//
// RPC definitions.
//
// remember to capitalize all names.
//

// RegisterArgs is the argument passed when a worker registers with the master.
type RegisterArgs struct {
	WorkerName string // the worker's UNIX-domain socket name, i.e. its RPC address.
}

// ShutdownReply is the argument passed when a worker shutdown.
type ShutdownReply struct {
	IsDown bool
}
