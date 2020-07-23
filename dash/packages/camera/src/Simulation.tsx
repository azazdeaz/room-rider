import CameraControls from 'camera-controls'
import randomColor from 'randomcolor'
import React, { FC, useCallback, useEffect, useRef, useState } from 'react'
import * as THREE from 'three'

CameraControls.install({ THREE: THREE })

import {
  ProcessedImage,
  Empty,
  Ping,
  ApriltagDetection,
} from './stubs/things_pb'
import { ProcessedImageStreamerPromiseClient } from './stubs/things_grpc_web_pb'

// @ts-ignore
const enableDevTools = window.__GRPCWEB_DEVTOOLS__ || (() => {})

type Vector2 = [number, number]
type Vector3 = [number, number, number]
interface Detection {
  tag_family: string
  tag_id: number
  hamming: number
  decision_margin: number
  homography: [Vector3, Vector3, Vector3]
  center: Vector2
  corners: [Vector2, Vector2, Vector2, Vector2]
  pose_R: [Vector3, Vector3, Vector3]
  pose_t: [[number], [number], [number]]
  pose_err: number
}

interface ICreateThreePartsOptions {
  canvas: HTMLCanvasElement
  width: number
  height: number
}

interface IProps {
  width: number
  height: number
}

const createThreeParts = ({
  canvas,
  width,
  height,
}: ICreateThreePartsOptions) => {
  const scene = new THREE.Scene()
  const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000)
  camera.position.z = 2
  const renderer = new THREE.WebGLRenderer({ canvas })
  const controls = new CameraControls(camera, renderer.domElement)
  const clock = new THREE.Clock()

  const geometry = new THREE.BoxGeometry(1, 1, 1)
  const geo = new THREE.EdgesGeometry(geometry)
  const mat = new THREE.LineBasicMaterial({ color: 0xffffff, linewidth: 2 })
  const wireframe = new THREE.LineSegments(geo, mat)
  scene.add(wireframe)

  return {
    scene,
    camera,
    renderer,
    canvas,
    controls,
    planes: new Map<number, THREE.Mesh>(),
    clock,
  }
}

const createPlane = ({ color }: { color: string }) => {
  const geometry = new THREE.PlaneGeometry(0.08, 0.08, 32)
  const material = new THREE.MeshBasicMaterial({
    color,
    side: THREE.DoubleSide,
  })
  const plane = new THREE.Mesh(geometry, material)
  return plane
}

const updatePlanes = (
  threeParts: ReturnType<typeof createThreeParts>,
  detections: ApriltagDetection[],
) => {
  const { scene, camera, renderer, planes } = threeParts

  for (const [i, d] of detections.entries()) {
    const detection = d.toObject()
    console.log(detection)
    if (!planes.has(detection.id)) {
      const plane = createPlane({
        color: randomColor({ luminosity: 'light', seed: detection.id }),
      })
      planes.set(detection.id, plane)
    }
    const plane = planes.get(detection.id)!
    const { x, y, z } = detection.pose?.translation!
    plane.position.set(x, y, z)
    const r_ = detection.pose?.rotationList
    if (r_?.length !== 9) {
      throw Error()
    }
    const r = [
      [r_[0], r_[1], r_[2]],
      [r_[3], r_[4], r_[5]],
      [r_[6], r_[7], r_[8]],
    ]
    const tmat = new THREE.Matrix4()
    tmat.set(
      r[0][0],
      r[0][1],
      r[0][1],
      x,
      r[1][0],
      r[1][1],
      r[1][1],
      y,
      r[2][0],
      r[2][1],
      r[2][1],
      z,
      0,
      0,
      0,
      1,
    )
    plane.rotation.setFromRotationMatrix(tmat)
    plane.rotation.x += Math.PI / 2
    scene.add(plane)
  }
  planes.forEach((plane, index) => {
    if (!detections.some((d) => d.getId() === index)) {
      scene.remove(plane)
    }
  })
  renderer.render(scene, camera)
}

export const Simulation: FC<IProps> = (props) => {
  const lastImage = useRef<ProcessedImage>(null)
  const [threeParts, setThreeParts] = useState<ReturnType<
    typeof createThreeParts
  > | null>(null)
  useEffect(() => {
    if (threeParts) {
      const service = new ProcessedImageStreamerPromiseClient(
        'http://localhost:8080',
      )
      enableDevTools([service])
      const stream = service.streamProcessedImages(new Empty())
      stream.on('data', function (processedImage) {
        console.log('data in', threeParts)
        if (threeParts) {
          const detections = processedImage.getApriltagDetectionsList()
          // console.log({ detections })
          updatePlanes(threeParts, detections)
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
    }
  }, [threeParts])

  const ref = useCallback((node: HTMLCanvasElement | null) => {
    if (node !== null) {
      setThreeParts(
        createThreeParts({
          canvas: node,
          width: props.width,
          height: props.height,
        }),
      )
    }
  }, [])

  useEffect(() => {
    if (threeParts) {
      const { controls, clock, renderer, scene, camera } = threeParts

      const animate = () => {
        const delta = clock.getDelta()
        const hasControlsUpdated = controls.update(delta)

        requestAnimationFrame(animate)

        // you can skip this condition to render though
        if (hasControlsUpdated) {
          renderer.render(scene, camera)
        }
      }

      // updatePlanes(threeParts, testDetections)
      renderer.render(scene, camera)
      animate()
    }
  }, [threeParts])

  return (
    <div style={{ width: 640, height: 480 }}>
      <canvas width={640} height={480} ref={ref} />
    </div>
  )
}
