const { genstubs } = require('genstubs')
const { join } = require('path')
 console.log('genstubs', genstubs)
genstubs({
  outDir: join(__dirname, 'src/stubs'),
  protoPath: join(__dirname, '../../../proto/things.proto'),
})
console.log('---')