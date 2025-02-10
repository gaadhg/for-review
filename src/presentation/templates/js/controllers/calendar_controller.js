const showHideBtn = document.querySelector('.show-hide-btn');
    const container = document.querySelector('#info');

    showHideBtn.addEventListener('click', () => {
      document.startViewTransition(() => {
        container.classList.toggle('expanded');
      });
    });
