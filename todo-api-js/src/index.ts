import express from 'express'
import dotenv from 'dotenv'

import { router as todoRouter } from '@framework/presentation/todo'

dotenv.config()
const app = express()
const port = Number(process.env.PORT ?? 3000)

app.use(express.json())
app.use('/todos', todoRouter)

app.listen(port, () => {
  console.log(`todo API listening on port ${port}`)
})
