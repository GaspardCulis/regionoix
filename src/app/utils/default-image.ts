import { Directive, HostListener, Input } from '@angular/core';

@Directive({
    selector: 'img[defaultImage]'
})
export class DefaultImageDirective {
    @Input() defaultImage: string = 'assets/idefault.png';

    @HostListener('error', ['$event'])
    onError(event: Event) {
        (event.target as HTMLImageElement).src = this.defaultImage;
    }
}
