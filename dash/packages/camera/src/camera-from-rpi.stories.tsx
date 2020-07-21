import React, { useEffect, useState, useRef, useCallback } from 'react'
// import { action } from '@storybook/addon-actions';
import { DownloadButton } from './DownloadButton'

import { Image as ImageMsg, Empty, Ping } from './stubs/things_pb'
import { ImageStreamerPromiseClient } from './stubs/things_grpc_web_pb'
import { DirectionalLightShadow } from 'three'

export default {
  title: 'Camera',
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
  const getBase64 = useCallback(() => {
    if (!lastImage.current) {
      return ''
    }
    return lastImage.current.getImageData_asB64()
  }, [])
  useEffect(() => {
    const service = new ImageStreamerPromiseClient('http://127.0.0.1:8080')
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

  return (
    <div>
      <canvas ref={canvas} width={width} height={height} />
      <DownloadButton getB64Content={getBase64} filePrefix="image" />
    </div>
  )
}
