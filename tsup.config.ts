import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['utils/xoev-xwasser-utils.ts'],
  outDir: 'pkg',
  dts: true,
  external: ['xoev-xwasser'],
  splitting: false,
  sourcemap: true,
  clean: false,
  tsconfig: 'tsconfig.utils.json',
})