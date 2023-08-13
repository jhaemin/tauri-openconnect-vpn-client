import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { Command, open } from '@tauri-apps/api/shell'

function App() {
  async function connectVPN() {
    const command = Command.sidecar(
      'binaries/openconnect',
      '--protocol=gp vpn.woowa.in'
    )
    const child = await command.spawn()
    console.log('pid', child.pid)

    command.stdout.on('data', (line) => console.log(line))
    command.stderr.on('data', async (line) => {
      console.log(line)
      if (line.includes('login')) {
        console.log('login')
        await child.write('{username}\n')
        await child.write('{password}\n')
      } else if (line.includes('OTP')) {
        console.log('otp')
        await child.write('{otp}\n')
      }
    })

    // TODO: handle disconnect
  }

  return (
    <div>
      <button
        onClick={() => {
          // Listen for OAuth redirect
          listen('oauth://url', (data) => {
            console.log(data)
          })

          // Open local OAuth redirect server
          invoke('plugin:oauth|start').then((port) => {
            console.log(port)

            open(``) // Google sign in url
          })
        }}
      >
        Sign in with Google
      </button>
    </div>
  )
}

export default App
