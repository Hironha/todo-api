import * as Todo from '@functions/todo'

function main() {
  const input: Todo.CreateInput = { description: 'teste', title: 'teste', todoAt: new Date() }
  Todo.create({ repository: {} as any, input })
}

main()
