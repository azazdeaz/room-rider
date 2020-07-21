import React, { useEffect, useState, useRef, useCallback } from 'react'
// import { action } from '@storybook/addon-actions';
import { Button } from '@storybook/react/demo'

import { ProcessedImage, Empty, Ping } from './stubs/things_pb'
import { ProcessedImageStreamerPromiseClient } from './stubs/things_grpc_web_pb'
import { DownloadButton } from './DownloadButton'

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
  img.src = 'data:image/bmp;base64,' + b64image
}

function rgb2rgba(
  rgb: Uint8Array,
  width: number,
  height: number,
  output: Uint8ClampedArray,
) {
  let inIdx = 0
  let outIdx = 0

  for (let i = 0; i < width * height; i++) {
    const r = rgb[inIdx++]
    const g = rgb[inIdx++]
    const b = rgb[inIdx++]
    output[outIdx++] = r
    output[outIdx++] = g
    output[outIdx++] = b
    output[outIdx++] = 255
  }

  return output
}

let __r = 0

export const Default = () => {
  const canvas = useRef<HTMLCanvasElement>(null)
  const lastImage = useRef<ProcessedImage>(null)
  const [width, setWidth] = useState(640)
  const [height, setHeight] = useState(480)
  const getBase64 = useCallback(() => {
    if (!lastImage.current) {
      return ''
    }
    return lastImage.current.getImage()!.getImageData_asB64()
  }, [])
  useEffect(() => {
    const service = new ProcessedImageStreamerPromiseClient(
      'http://localhost:8080',
    )
    enableDevTools([service])
    const stream = service.streamProcessedImages(new Empty())
    stream.on('data', function (processedImage) {
      if (++__r % 1 === 0) {
        // @ts-ignore
        lastImage.current = processedImage
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
        const ctx = canvas.current.getContext('2d')!
        ctx.clearRect(0, 0, canvas.current.width, canvas.current.height)

        const image = lastImage.current.getImage()
        if (image) {
          setWidth(image.getWidth())
          setHeight(image.getHeight())
          const imageData = new ImageData(image.getWidth(), image.getHeight())
          rgb2rgba(
            image.getImageData_asU8(),
            image.getWidth(),
            image.getHeight(),
            imageData.data,
          )
          ctx.putImageData(imageData, 0, 0)
        }

        const detections = lastImage.current.getApriltagDetectionsList()
        for (const detection of detections) {
          const corners = detection.getCorners()?.toObject()
          if (!corners) {
            break
          }
          ctx.beginPath()
          ctx.strokeStyle = '#00ff11'
          ctx.moveTo(corners.d?.x ?? 0, corners.d?.y ?? 0)
          ctx.lineTo(corners.a?.x ?? 0, corners.a?.y ?? 0)
          ctx.lineTo(corners.b?.x ?? 0, corners.b?.y ?? 0)
          ctx.lineTo(corners.c?.x ?? 0, corners.c?.y ?? 0)
          ctx.lineTo(corners.d?.x ?? 0, corners.d?.y ?? 0)
          ctx.stroke()
        }
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
