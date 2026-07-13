export const debounce = <T extends unknown[]>(
  func: (...args: T) => void,
  delay: number
): ((...args: T) => void) => {
  let timer: number | null = null
  return (...args: T) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => {
      func.call(null, ...args)
    }, delay)
  }
}

const createRange = (
  node: Node,
  chars: { count: number },
  range?: Range
): Range => {
  if (!range) {
    range = document.createRange()
    range.selectNode(node)
    range.setStart(node, 0)
  }

  if (chars.count === 0) {
    range.setEnd(node, chars.count)
  } else if (node && chars.count > 0) {
    if (node.nodeType === Node.TEXT_NODE && node.textContent) {
      if (node.textContent.length < chars.count) {
        chars.count -= node.textContent.length
      } else {
        range.setEnd(node, chars.count)
        chars.count = 0
      }
    } else {
      for (let lp = 0; lp < node.childNodes.length; lp++) {
        range = createRange(node.childNodes[lp], chars, range)

        if (chars.count === 0) {
          break
        }
      }
    }
  }

  return range
}

const isChildOf = (node: Node | null, parentElement: Node) => {
  while (node !== null) {
    if (node === parentElement) {
      return true
    }
    node = node.parentNode
  }

  return false
}

export const getCurrentCursorPosition = (parentElement: HTMLElement) => {
  const selection = window.getSelection()
  let charCount = -1

  if (selection?.focusNode) {
    if (isChildOf(selection.focusNode, parentElement)) {
      let node = selection.focusNode

      charCount = selection.focusOffset

      while (node) {
        if (node === parentElement) {
          break
        }

        if (node.previousSibling) {
          node = node.previousSibling
          charCount += node.textContent?.length || 0
        } else if (node.parentNode) {
          node = node.parentNode
          if (node === null) {
            break
          }
        }
      }
    }
  }

  return charCount
}

export const resetCursorPosition = (
  cursorPosition: number,
  parentElement: HTMLElement
) => {
  if (cursorPosition >= 0) {
    const selection = window.getSelection()
    const range = createRange(parentElement, { count: cursorPosition })

    if (range && selection) {
      range.collapse(false)
      selection.removeAllRanges()
      selection.addRange(range)
    }
    parentElement.focus()
  }
}
