import { ApplicationConfig, provideBrowserGlobalErrorListeners, provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideHttpClient } from '@angular/common/http';
import { Configuration, provideApi } from './generated/clients/regionoix-client';
import { provideNgxStripe } from 'ngx-stripe';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideHttpClient(),
    provideApi(
      new Configuration({
        basePath: '',
        withCredentials: true,
      })),
    provideNgxStripe(),
  ]
};
