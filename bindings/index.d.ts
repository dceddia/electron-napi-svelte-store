/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function sum(a: number, b: number): number
export class Ticker {
  constructor(value?: number | undefined | null)
  increment(): void
  set(value: number): void
  /**
   * Implement Svelte's Store Contract, defined as:
   *
   * store = {
   *   subscribe: (
   *     subscription: (value: any) => void
   *   ) => (() => void), set?: (value: any) => void
   * }
   *
   * - A store must contain a .subscribe method, which must accept as its argument a subscription function.
   *   This subscription function must be immediately and synchronously called with the store's current value
   *   upon calling .subscribe. All of a store's active subscription functions must later be synchronously
   *   called whenever the store's value changes.
   * - The .subscribe method MUST return an `unsubscribe` function.
   *   Calling an `unsubscribe` function must stop its subscription, and its corresponding subscription
   *   function must not be called again by the store.
   * - A store may optionally contain a `.set` method, which must accept as its argument a new value
   *   for the store, and which synchronously calls all of the store's active subscription functions.
   *   Such a store is called a writable store.
   */
  subscribe(callback: (...args: any[]) => any): (...args: any[]) => any
}
