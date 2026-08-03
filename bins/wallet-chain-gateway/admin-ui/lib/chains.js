const CHAIN_NAMES = {
  0: 'BTC', 3: 'DOGE', 60: 'ETH', 195: 'TRON', 501: 'SOL', 966: 'POL',
  20000714: 'BSC', 8453: 'BASE', 10042221: 'ARB', 10000070: 'OP',
  10000900: 'AVAX', 10000999: 'HYPER', 10004663: 'ROBIN',
}

export const CHAIN_OPTIONS = Object.entries(CHAIN_NAMES)
  .map(([id, name]) => ({ id: Number(id), name }))
  .sort((a, b) => a.name.localeCompare(b.name))

export function chainName(id) {
  return CHAIN_NAMES[id] ?? String(id)
}
