package questions

type trieNode struct {
	children map[rune]*trieNode
	isEnd    bool
}

type Trie struct {
	root *trieNode
}

func Constructor() Trie {
	return Trie{
		root: &trieNode{
			children: make(map[rune]*trieNode),
			isEnd:    false,
		},
	}
}

func (this *Trie) Insert(word string) {
	cur := this.root
	for _, c := range word {
		if _, ok := cur.children[c]; !ok {
			cur.children[c] = &trieNode{
				children: make(map[rune]*trieNode),
				isEnd:    false,
			}
		}
		cur = cur.children[c]
	}
	cur.isEnd = true
}

func (this *Trie) traverse(word string) *trieNode {
	cur := this.root
	for _, c := range word {
		if _, ok := cur.children[c]; !ok {
			return nil
		}
		cur = cur.children[c]
	}
	return cur
}

func (this *Trie) Search(word string) bool {
	node := this.traverse(word)
	return node != nil && node.isEnd
}

func (this *Trie) StartsWith(prefix string) bool {
	return this.traverse(prefix) != nil
}
