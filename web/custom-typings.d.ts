type RequestIdleCallbackHandle = any
type RequestIdleCallbackOptions = {
  timeout: number
}
type RequestIdleCallbackDeadline = {
  readonly didTimeout: boolean
  timeRemaining: (() => number)
}

interface Window {
  requestIdleCallback: ((
    callback: ((deadline: RequestIdleCallbackDeadline) => void),
    opts?: RequestIdleCallbackOptions,
  ) => RequestIdleCallbackHandle)
  cancelIdleCallback: ((handle: RequestIdleCallbackHandle) => void)
}

interface ReadonlyArray<T> {
  flatMap<U, This = undefined>(
    callback: (this: This, value: T, index: number, array: T[]) => U | U[],
    thisArg?: This,
  ): U[]
}

interface Array<T> {
  flatMap<U, This = undefined>(
    callback: (this: This, value: T, index: number, array: T[]) => U | U[],
    thisArg?: This,
  ): U[]
}
