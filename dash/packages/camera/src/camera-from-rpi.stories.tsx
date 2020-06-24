import React, { useEffect, useState, useRef } from 'react'
// import { action } from '@storybook/addon-actions';
import { Button } from '@storybook/react/demo'

import { Image as ImageMsg, Empty, Ping } from './stubs/things_pb'
import { ImageStreamerPromiseClient } from './stubs/things_grpc_web_pb'

export default {
  title: 'Camera',
  component: Button,
}

// @ts-ignore
const enableDevTools = window.__GRPCWEB_DEVTOOLS__ || (() => {})

function writeToContextWithURL(
  ctx: CanvasRenderingContext2D,
  b64image: string,
) {
  const img = new Image()
  img.onload = () => {
    ctx.drawImage(img, 0, 0)
  }
  img.onerror = (e) => {
    console.log('Error during loading image:', e)
  }
  img.src = 'data:image/jpeg;base64,' + b64image
}

let __r = 0

export const Raw = () => {
  const canvas = useRef<HTMLCanvasElement>(null)
  const lastImage = useRef<ImageMsg>(null)
  const [width, setWidth] = useState(0)
  const [height, setHeight] = useState(0)
  useEffect(() => {
    const service = new ImageStreamerPromiseClient(
      'http://127.0.0.1:8080',
    )
    enableDevTools([service])
    const stream = service.streamImages(new Empty())
    stream.on('data', function (image) {
      if (++__r % 1 === 0) {
        // @ts-ignore
        lastImage.current = image
      }
    })
    stream.on('status', function (status) {
      console.log('status.code', status.code)
      console.log('status.details', status.details)
      console.log('status.metadata', status.metadata)
    })
    stream.on('end', function () {
      console.log('stream end signal')
    })

    return () => stream.cancel()
  }, [])

  useEffect(() => {
    function draw() {
      if (lastImage.current && canvas.current) {
        // console.time('render pb')
        
        const image = lastImage.current
        if (!image) {
          return
        }
        setWidth(image.getWidth())
        setHeight(image.getHeight())
        const ctx = canvas.current.getContext('2d')!
        writeToContextWithURL(ctx, image.getImageData_asB64())
        // console.timeEnd('render pb')
      }
      idRaf = requestAnimationFrame(() => draw())
    }
    let idRaf = requestAnimationFrame(() => draw())

    return () => cancelAnimationFrame(idRaf)
  }, [])

  return <canvas ref={canvas} width={width} height={height} />
}
