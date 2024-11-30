package questions

type LRUCache struct {
	keys   []int
	values map[int]int
	cap    int
}

func ConstructorCache(capacity int) LRUCache {
	return LRUCache{
		keys:   make([]int, 0, capacity),
		values: make(map[int]int),
		cap:    capacity,
	}
}

func (this *LRUCache) Get(key int) int {
	if _, ok := this.values[key]; !ok {
		return -1
	}
	for i, k := range this.keys {
		if k == key {
			this.keys = append(this.keys[:i], this.keys[i+1:]...)
		}
	}

	this.keys = append(this.keys, key)
	return this.values[key]
}

func (this *LRUCache) Put(key int, value int) {
	if _, ok := this.values[key]; ok {
		for i, k := range this.keys {
			if k == key {
				this.keys = append(this.keys[:i], this.keys[i+1:]...)
			}
		}
	} else if len(this.keys)+1 > this.cap {
		delete(this.values, this.keys[0])
		this.keys = this.keys[1:]
	}

	this.values[key] = value
	this.keys = append(this.keys, key)
}
