package mr

import (
	"encoding/json"
	"log"
	"os"
	"sort"
)

func doReduce(reduceTaskNumber int, // which reduce task this is
	outFile string, // write the output here
	nMap int, // the number of map tasks that were run ("M" in the paper)
	reduceF func(key string, values []string) string) {
	var keys []string                   // store all keys in this partition
	var kvs = make(map[string][]string) // store all key-value pairs from nMap imm files

	// read nMap imm files from map workers
	for i := 0; i < nMap; i++ {
		fn := reduceFileName(i, reduceTaskNumber)
		imm, err := os.Open(fn)
		if err != nil {
			log.Printf("open immediate file %s failed", fn)
			continue
		}

		var kv KeyValue
		dec := json.NewDecoder(imm)
		err = dec.Decode(&kv)
		for err == nil {
			// is this key seen?
			if _, ok := kvs[kv.Key]; !ok {
				keys = append(keys, kv.Key)
			}
			kvs[kv.Key] = append(kvs[kv.Key], kv.Value)

			// decode repeatedly until an error
			err = dec.Decode(&kv)
		}
	}

	// Original MapReduce Paper 4.2 Ordering Guarantees
	// Keys in one partition are processed in increasing key order
	sort.Strings(keys)
	out, err := os.Create(outFile)
	if err != nil {
		log.Printf("create output file %s failed", outFile)
		return
	}
	enc := json.NewEncoder(out)
	for _, key := range keys {
		if err = enc.Encode(KeyValue{key, reduceF(key, kvs[key])}); err != nil {
			log.Printf("write [key: %s] to file %s failed", key, outFile)
		}
	}
	out.Close()
}
