const registerServiceWorker = async () => {
  if ('serviceWorker' in navigator) {
    try {
      const options = { scope: '/' }
      const registration = await navigator.serviceWorker.register('/service_worker.js', options)

      if (registration.installing) {
        console.info('ServiceWorker@installing')
      } else if (registration.waiting) {
        console.info('ServiceWorker@installed')
      } else if (registration.active) {
        console.info('ServiceWorker@actived')
      }
    } catch (error) {
      console.error(`ServiceWorker@registration failed: ${error}`)
    }
  }
}

registerServiceWorker()
