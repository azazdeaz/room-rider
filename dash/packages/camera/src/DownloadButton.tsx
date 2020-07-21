import React, { useState, useCallback } from 'react'
import { Button } from '@storybook/react/demo'

type Props = {
  filePrefix: string
  getB64Content: () => string
}

function downloadBase64File(contentBase64: string, fileName: string) {
  const linkSource = `data:application/pdf;base64,${contentBase64}`
  const downloadLink = document.createElement('a')
  document.body.appendChild(downloadLink)

  downloadLink.href = linkSource
  downloadLink.target = '_self'
  downloadLink.download = fileName
  downloadLink.click()
}

export const DownloadButton = (props: Props) => {
  const [no, setNo] = useState(1)
  const { filePrefix, getB64Content } = props
  const save = useCallback(() => {
    downloadBase64File(getB64Content(), `${filePrefix}-${no}.jpeg`)
    setNo(no + 1)
  }, [filePrefix, no])

  return (
    <Button onClick={save}>
      <span role="img" aria-label="save">
        save
      </span>
    </Button>
  )
}
