import { exec } from 'child_process'
import { dirname, basename, join } from 'path'
import mkdirp from 'mkdirp'

const run = (cmd: string) => {
  return new Promise((resolve, reject) => {
    exec(cmd, (error, stdout, stderr) => {
      if (error) {
        reject(error)
        return
      }
      resolve({ stderr, stdout})
    })
  })
}

type Options = {
  outDir: string
  protoPath: string
}

function bin_path(bin: string) {
  // TODO find a robust way to locate the executables
  return join(__dirname, '../node_modules/.bin', bin)
}

export async function genstubs({ outDir, protoPath }: Options) {
  await mkdirp(outDir)
  const folder = dirname(protoPath)
  const filename = basename(protoPath)
  await run(`${bin_path("grpc_tools_node_protoc")} \
  -I=${folder} ${filename}   \
  --js_out=import_style=commonjs,binary:${outDir} \
  --grpc_out=${outDir} --plugin=protoc-gen-grpc=${bin_path("grpc_tools_node_protoc_plugin")} \
  --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:${outDir}`)
  await run(`protoc \
  --plugin=protoc-gen-ts=${bin_path("protoc-gen-ts")} \
  --ts_out=${outDir} \
  -I=${folder} ${filename}`)
}
