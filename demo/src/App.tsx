import { Suspense, useState, useCallback } from 'react'
import type { MetadataField } from '@pkg/mediakit_wasm'
import FilenameParser from './components/FilenameParser/FilenameParser'
import ExamplePicker from './components/ExamplePicker'
import MetadataTable from './components/MetadataTable'

export default function App() {
  const [metadata, setMetadata] = useState<MetadataField[]>([])
  const [externalFilename, setExternalFilename] = useState<string | null>(null)

  const handleParse = useCallback(
    (metadata: MetadataField[]) => {
      setMetadata(metadata)
    },
    []
  )

  const handleExample = useCallback((name: string) => {
    setExternalFilename(name)
  }, [])

  return (
    <main>
      <header>
        <h1>Filename Parsing Demo</h1>
        <p className='subtitle'>
          Parse media filenames into structured metadata using{' '}
          <a href='https://github.com/jkwill87/mediakit'>mediakit</a>.
          <br />
          Type a filename or try an example below.
        </p>
      </header>
      <Suspense
        fallback={<div className='loading'>Loading WASM module...</div>}
      >
        <section className='parser-section'>
          <FilenameParser
            onParse={handleParse}
            externalFilename={externalFilename}
          />
          <ExamplePicker onSelect={handleExample} />
        </section>
        <MetadataTable metadata={metadata} />
      </Suspense>
    </main>
  )
}
