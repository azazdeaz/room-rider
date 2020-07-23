import React, { useEffect, useState, useRef, useCallback } from 'react'
import { Button } from '@storybook/react/demo'
import { Simulation } from './Simulation'

export default {
  title: 'Camera',
  component: Button,
}


export const View3D = () => {
  return (
    <div>
      <Simulation  width={640} height={480} />
    </div>
  )
}
