import { Elysia } from 'elysia'

import { createTodoRouter } from '@framework/presentation/routes/todo'

function main() {
  const port = Number(process.env.PORT ?? 3000)
  if (Number.isNaN(port)) {
    throw new Error(`Invalid port ${port}`)
  }

  new Elysia().use(createTodoRouter('todos')).listen(port, () => {
    console.log(`todo API listening on port ${port}`)
  })
}

main()
