import type { MetadataField } from '@pkg/mediakit_wasm'
import './MetadataTable.css'

interface MetadataTableProps {
  metadata: MetadataField[]
}

export default function MetadataTable({ metadata }: MetadataTableProps) {
  if (metadata.length === 0) return null

  return (
    <section className='metadata'>
      <h2 className='metadata-heading'>Parsed Metadata</h2>
      <div className='metadata-grid'>
        {metadata.map((field, i) => (
          <div className='metadata-row' key={`${field.key}-${i}`}>
            <span className='metadata-key'>{field.key}</span>
            <span className='metadata-value'>{field.value}</span>
          </div>
        ))}
      </div>
    </section>
  )
}
