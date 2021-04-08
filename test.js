const validate = require('.');
const test = require('tape')
const fs = require('fs');
const path = require('path')
const Log = require('async-append-only-log')
const generateFixture = require('ssb-fixtures')
const rimraf = require('rimraf')
const mkdirp = require('mkdirp')
const JITDB = require('jitdb')
const {
  query,
  fromDB,
  where,
  equal,
  slowEqual,
  toCallback,
} = require('jitdb/operators')
const seekType = require('jitdb/test/helpers')
const copy = require('jitdb/copy-json-to-bipf-async')

// define directory and paths 
const dir = '/tmp/validate-benchmark' 
const oldLogPath = path.join(dir, 'flume', 'log.offset') 
const newLogPath = path.join(dir, 'flume', 'log.bipf') 
const indexesDir = path.join(dir, 'indexes')

// generate fixture
rimraf.sync(dir)
mkdirp.sync(dir)

const SEED = 'sloop'
const MESSAGES = 5000
const AUTHORS = 1

test('generate fixture with flumelog-offset', (t) => {
  generateFixture({
    outputDir: dir,
    seed: SEED,
    messages: MESSAGES,
    authors: AUTHORS,
    slim: true,
  }).then(() => {
    t.true(fs.existsSync(oldLogPath), 'log.offset was created')
    t.end()
  })
})
    
test('move flumelog-offset to async-log', (t) => {
  copy(oldLogPath, newLogPath, (err) => {
    if (err) t.fail(err)
    setTimeout(() => {
      t.true(fs.existsSync(newLogPath), 'log.bipf was created')
      t.end()
    }, 4000)
  })
})

let raf
let db

test('core indexes', (t) => {
  const start = Date.now()
  raf = Log(newLogPath, { blockSize: 64 * 1024 })
  rimraf.sync(indexesDir)
  db = JITDB(raf, indexesDir)
  db.onReady(() => {
    const duration = Date.now() - start
    t.pass(`duration: ${duration}ms`)
    t.end()
  })
})

// read data from offset log and populate into msgs array
test('query for post messages and attempt validation', (t) => {
  db.onReady(() => {
    query(
      fromDB(db),
      //where(equal(seekType, 'post', { indexType: 'type' })),
      toCallback((err, msgs) => {
        if (err) t.fail(err)
        const start = Date.now()
        // validate array of successive messages from a feed
        validate.validateMsgArray(msgs)
        const duration = Date.now() - start
        t.pass(`validated ${MESSAGES} messages. duration: ${duration}ms`)
        t.end()
      })
    )
  })
})
