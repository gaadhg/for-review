// addEventListener("turbo:before-frame-render", (event) => {
//     if (document.startViewTransition) {
//       const originalRender = event.detail.render
//       event.detail.render = (currentElement, newElement) => {
//         document.startViewTransition(()=> originalRender(currentElement, newElement))
//       }
//     }
//   })

// addEventListener("turbo:before-render", (event) => {

//   event.preventDefault()

//   document.startViewTransition(() => {
//       event.detail.resume();                
//   })
// })


document.addEventListener("turbo:before-render", async (event) => {
            
  let fn = null

  event.preventDefault()

  document.startViewTransition(() => {
      event.detail.resume()

      const promise = new Promise((resolve, reject) => {
          
          fn = (event) => {
              resolve()
          }

          document.addEventListener('turbo:render', fn)
      })

      promise.finally(() => {
          document.removeEventListener('turbo:render', fn)
      })
  
      return promise
  })

})