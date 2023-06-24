import * as E from '@helpers/either'

function something(): E.Either<string, number> {
  return E.right(20)
}

function main(): void {
  const foo = something()
  if (E.isLeft(foo)) {
    console.log(foo.value.at(0))
  } else {
    console.log(foo.value.toString())
  }
  console.log('todo')
}

main()
