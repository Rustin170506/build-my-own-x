package mr

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"os"
)

func doMap(mapTaskNumber int, // which map task this is
	inFile string,
	nReduce int, // the number of reduce task that will be run ("R" in the paper)
	mapf func(file string, contents string) []KeyValue) {
	log.Printf("Map: start map file %s\n", inFile)
	file, err := os.Open(inFile)
	if err != nil {
		log.Fatalf("cannot open %v", inFile)
	}
	contents, err := ioutil.ReadAll(file)
	if err != nil {
		log.Fatalf("cannot read %v", inFile)
	}
	file.Close()
	debug("Map: read file %s success\n", inFile)
	kvs := mapf(inFile, string(contents))
	// make R reduce file and encoder.
	var imm = make([]*os.File, nReduce)
	var enc = make([]*json.Encoder, nReduce)
	// crate file and init encoder.
	for i := 0; i < nReduce; i++ {
		if file, err := os.Create(reduceFileName(mapTaskNumber, i)); err != nil {
			log.Printf("create file %s failed", reduceFileName(mapTaskNumber, i))
		} else {
			imm[i] = file
			enc[i] = json.NewEncoder(file)
			log.Printf("create file %s success", file.Name())
		}
	}

	// random choose the reduce file to write.
	for _, kv := range kvs {
		r := ihash(kv.Key) % nReduce
		if enc[r] != nil {
			if err := enc[r].Encode(&kv); err != nil {
				log.Printf("wirte %v to file %s failed", kv, reduceFileName(mapTaskNumber, r))
			}
		}
	}

	// close immediate files
	for i := 0; i < nReduce; i++ {
		if imm[i] != nil {
			imm[i].Close()
		}
	}
	log.Printf("Map: finished map file %s\n", inFile)
}
