import express from 'express'
import dotenv from 'dotenv'

import { createTodoRouter } from '@framework/presentation/routes/todo'
import { TodoStore } from '@framework/store/todo'

dotenv.config()

function main() {
  const app = express()
  const port = Number(process.env.PORT ?? 3000)

  app.use(express.json())
  app.use('/todos', createTodoRouter(new TodoStore()))

  app.listen(port, () => {
    console.log(`todo API listening on port ${port}`)
  })
}

main()
