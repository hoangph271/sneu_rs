const registerServiceWorker = async () => {
  if ('serviceWorker' in navigator) {
    try {
      const registration = await navigator.serviceWorker.register('/service_worker.js', {
        scope: '/',
      })

      if (registration.installing) {
        console.info('Service worker installing')
      } else if (registration.waiting) {
        console.info('Service worker installed')
      } else if (registration.active) {
        console.info('Service worker actived...!')
      }
    } catch (error) {
      console.error(`Registration failed with ${error}`)
    }
  }
}

registerServiceWorker()
