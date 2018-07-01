const Fragment = Symbol('Fragment')

type Fragment = typeof Fragment

Object.assign(window, {
  React: { Fragment, createElement },
})

const booleanishPropertyKeys = new Set([
  'disabled',
  'autocomplete',
  'selected',
  'checked',
])

function appendChildren<T extends Node>(node: T, children: Node[]): T {
  ;[].concat(...children).forEach(child => {
    if (child instanceof Node) {
      node.appendChild(child)
    } else {
      node.appendChild(document.createTextNode(child))
    }
  })
  return node
}

export default function createElement(
  fragment: Fragment,
  attrs,
  ...children
): DocumentFragment
export default function createElement<T>(
  fun: (attrs, ...children) => T,
  attrs,
  ...children
): T
export default function createElement(
  tagName: 'text',
  attrs,
  ...children
): Text
export default function createElement(
  tagName: 'div',
  attrs,
  ...children
): HTMLDivElement
export default function createElement(
  tagName: 'input',
  attrs,
  ...children
): HTMLInputElement
export default function createElement(
  tagName: 'button',
  attrs,
  ...children
): HTMLButtonElement
export default function createElement(
  tagName: string,
  attrs,
  ...children
): Node
export default function createElement(
  tagName: string | Fragment | Function,
  attrs,
  ...children
) {
  if (tagName === Fragment) {
    return appendChildren(document.createDocumentFragment(), children)
  }

  if (typeof tagName === 'function') {
    return tagName(attrs, ...children);
  }

  if (tagName === 'text') {
    return document.createTextNode(children.join(''))
  }

  const el = document.createElement(tagName)
  for (let key in attrs) {
    if (key.startsWith('on')) {
      const evtName = key.slice(2)
      if (attrs[key] != null) {
        el.addEventListener(evtName, attrs[key], attrs[key].handleEventOptions)
      }
    } else if (booleanishPropertyKeys.has(key)) {
      el[key] = attrs[key] == null ? true : attrs[key]
    } else {
      if (attrs[key] != null) {
        el.setAttribute(key, attrs[key])
      }
    }
  }
  return children.length === 0 ? el : appendChildren(el, children)
}

declare global {
  class React {
    static readonly Fragment: symbol
    static readonly createElement: typeof createElement
  }
}
