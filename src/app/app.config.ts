import { ApplicationConfig, provideBrowserGlobalErrorListeners, provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideHttpClient } from '@angular/common/http';
import { Configuration, provideApi } from './generated/clients/regionoix-client';
import { provideNgxStripe } from 'ngx-stripe';

const PUBLIC_KEY = "pk_test_51S7X9JQunQaqPX1k0NBCgKA81b646HbCKAAx8HtcADqfSpTJgt9WiRPkYV60xdTsovZ5i97TOi70MLwYuSnoR0u100JXaDLw1s";

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
    provideNgxStripe(PUBLIC_KEY),
  ]
};
