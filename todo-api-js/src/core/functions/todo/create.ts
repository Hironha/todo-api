type CreateTodoInput = {
  title: string
  description: string
  todoAt?: Date
}

export async function create(input: CreateTodoInput): Promise<void> {}
