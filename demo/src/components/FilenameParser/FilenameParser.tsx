import { use, useRef, useEffect, useLayoutEffect, useCallback } from 'react'
import init, { callOnce, inspectFilepath } from '@pkg/mediakit_wasm'
import type { MetadataField } from '@pkg/mediakit_wasm'
import wasmUrl from '../../../wasm/pkg/mediakit_wasm_bg.wasm' with { type: 'file' }
import { useLocalStorage } from '@hooks'
import { getCurrentCursorPosition, resetCursorPosition } from './utils'
import './styles.css'

const wasmReady = init({ module_or_path: wasmUrl }).then(() => callOnce())

interface FilenameParserProps {
  onParse?: (metadata: MetadataField[]) => void
  externalFilename?: string | null
}

export default function FilenameParser({
  onParse,
  externalFilename,
}: FilenameParserProps) {
  use(wasmReady)

  const ref = useRef<HTMLDivElement>(null)
  const [filename, setFilename] = useLocalStorage(
    'filename',
    'Only.Murders.in.the.Building.S01E01.True.Crime.1080p.WEB.H264-FLAME.mkv',
    { raw: true }
  )

  useLayoutEffect(() => {
    const root = ref.current
    if (!root) return
    const cursorPos = getCurrentCursorPosition(root)
    const { tokens, metadata } = inspectFilepath(filename)
    onParse?.(metadata)
    const spans = tokens.map((token) => {
      const span = document.createElement('span')
      span.className = token.status
      span.textContent = filename.substring(token.start, token.end)
      if (token.status === 'matched') {
        span.dataset.tooltip = `${token.key} = '${token.value}'`
      }
      return span
    })
    root.replaceChildren(...spans)
    resetCursorPosition(cursorPos, root)
  }, [filename, onParse])

  useEffect(() => {
    if (externalFilename != null) {
      setFilename(externalFilename)
      // Focus and move cursor to end after injection
      requestAnimationFrame(() => {
        const root = ref.current
        if (!root) return
        root.focus()
        const sel = window.getSelection()
        sel?.selectAllChildren(root)
        sel?.collapseToEnd()
      })
    }
  }, [externalFilename])

  useEffect(() => {
    const root = ref.current
    if (!root) return
    root.focus()
    const sel = window.getSelection()
    sel?.selectAllChildren(root)
    sel?.collapseToEnd()
  }, [])

  const onInput = useCallback(
    (e: React.FormEvent<HTMLDivElement>) => {
      setFilename(e.currentTarget.textContent ?? '')
    },
    [setFilename]
  )

  return (
    <>
      <label htmlFor='filename-content' className='input-label'>
        Filename
      </label>
      <div
        ref={ref}
        className='container'
        contentEditable
        suppressContentEditableWarning
        autoCorrect='off'
        spellCheck={false}
        onInput={onInput}
        id='filename-content'
      />
    </>
  )
}
