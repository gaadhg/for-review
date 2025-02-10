import { Controller } from '/js/lib/stimulus.min.js';

export default class extends Controller {
  static targets = ["link"];

  setActive(event) {
    if (!this.linkTargets) return;

    this.linkTargets.forEach((link) => {
      if (link && link.classList) {
        link.classList.remove("active");
      }
    });

    if (event.currentTarget && event.currentTarget.classList) {
      event.currentTarget.classList.add("active");
    }
  }
}
