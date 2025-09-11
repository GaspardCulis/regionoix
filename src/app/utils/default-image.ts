import { Directive, HostListener, Input } from '@angular/core';

@Directive({
    selector: 'img[appDefaultImage]'
})
export class DefaultImageDirective {
    @Input() defaultImage = 'assets/idefault.png';

    @HostListener('error', ['$event'])
    onError(event: Event) {
        (event.target as HTMLImageElement).src = this.defaultImage;
    }
}
