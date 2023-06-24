import * as Todo from '@functions/todo'

function main() {
  Todo.create({ description: 'teste', title: 'teste', todoAt: new Date() })
}

main()
