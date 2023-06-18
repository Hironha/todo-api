import { right, Either } from '@helpers/either'

function something(): Either<string, number> {
  return right(20)
}

function main(): void {
  const foo = something()
  if (foo.isLeft()) {
    console.log(foo.value.at(0))
  } else {
    console.log(foo.value.toString())
  }
  console.log('todo')
}

main()
