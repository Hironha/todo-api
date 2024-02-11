import { type Result } from "@domain/utils/result";
import { type Id } from "@domain/value-objects/id";
import { type TodoEntity } from "@domain/entities/todo";
import { type FindByIdError } from "@application/repositories/todo";

export type FindTodoInput = { id: Id };

export type FindTodoOutput = Result<TodoEntity, FindTodoError>;

export type FindTodoError = FindByIdError;
