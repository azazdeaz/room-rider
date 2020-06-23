import { exec } from "child_process";
import { dirname, basename } from "path";

const run = (cmd: string) => {
  console.log(cmd)
  return new Promise((resolve, reject) => {
    exec(cmd, (error, stdout, stderr) => {
      if (error) {
        reject(error);
        return;
      }
      resolve({ stderr, stdout });
    });
  });
};

type Options = {
  outDir: string;
  protoPath: string;
};

export async function genstubs({ outDir, protoPath }: Options) {
  const folder = dirname(protoPath);
  const filename = basename(protoPath);

  await run(`grpc_tools_node_protoc \
  -I=${folder} ${filename}   \
  --js_out=import_style=commonjs,binary:${outDir} \
  --grpc_out=${outDir} --plugin=protoc-gen-grpc=\`which grpc_tools_node_protoc_plugin\`  \
  --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:${outDir}`);
  
  await run(`protoc \
  --plugin=protoc-gen-ts=\`which protoc-gen-ts\` \
  --ts_out=${outDir} \
  -I=${folder} ${filename}`);
}

genstubs({outDir: '../proto/things.proto', protoPath: './stubs'})
// console.log('ad')