import {render, screen} from '@testing-library/react'
import {App} from '../app'

test('it works!', async () => {
  await render(<App />)
  expect(screen.getByText(/Hello Vite.js and Tauri/i)).toBeInTheDocument()
})
