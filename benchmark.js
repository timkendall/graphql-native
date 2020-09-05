const Benny = require('benny')
const GraphQL = require('graphql')
const NativeGraphQL = require('.')


const QUERY = `
{ user { foo bar } }
`


console.log(GraphQL.parse(QUERY))
console.log(NativeGraphQL.parse(QUERY))


Benny.suite(
  'Parsing',

  Benny.add('JS', () => {
    return GraphQL.parse(QUERY)
  }, { maxTime: 1 }),

  Benny.add('Native', () => {
    return NativeGraphQL.parse(QUERY)
  }, { maxTime: 1 }),

  Benny.add('Native 2', () => {
    return NativeGraphQL.parse2(QUERY)
  }, { maxTime: 1 }),

  Benny.cycle(),
  Benny.complete(),
)