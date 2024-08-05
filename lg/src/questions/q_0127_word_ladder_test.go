package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_ladderLength(t *testing.T) {
	type args struct {
		beginWord string
		endWord   string
		wordList  []string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test-case-1",
			args: args{
				beginWord: "hit",
				endWord:   "cog",
				wordList:  []string{"hot", "dot", "dog", "lot", "log", "cog"},
			},
			want: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, ladderLength(tt.args.beginWord, tt.args.endWord, tt.args.wordList), "ladderLength(%v, %v, %v)", tt.args.beginWord, tt.args.endWord, tt.args.wordList)
		})
	}
}
