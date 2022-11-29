package shardkv

import "fmt"
import "../shardmaster"

type CommandRequest struct {
}

type CommandResponse struct {
	Err   Err
	Value string
}

type ShardOperationRequest struct {
}

type ShardOperationResponse struct {
}

type CommandType uint8

const (
	Operation CommandType = iota
	Configuration
	InsertShards
	DeleteShards
	EmptyEntry
)

type Command struct {
	Op   CommandType
	Data interface{}
}

func (command Command) String() string {
	return fmt.Sprintf("{Type:%v,Data:%v}", command.Op, command.Data)
}

func NewOperationCommand(request *CommandRequest) Command {
	return Command{Operation, *request}
}

func NewConfigurationCommand(config *shardmaster.Config) Command {
	return Command{Configuration, *config}
}

func NewInsertShardsCommand(response *ShardOperationResponse) Command {
	return Command{InsertShards, *response}
}

func NewDeleteShardsCommand(request *ShardOperationRequest) Command {
	return Command{DeleteShards, *request}
}

func NewEmptyEntryCommand() Command {
	return Command{EmptyEntry, nil}
}
