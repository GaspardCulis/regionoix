import { ComponentFixture, TestBed } from '@angular/core/testing';
import { TopbarComponent } from './topbar-component';
import { Router } from '@angular/router';
import { AuthStateService } from '../../../services/auth-state-service';
import { CategoriesService, RegionsService } from '../../../generated/clients/regionoix-client';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { of } from 'rxjs';

describe('TopbarComponent', () => {
  let component: TopbarComponent;
  let fixture: ComponentFixture<TopbarComponent>;
  let routerSpy: jasmine.SpyObj<Router>;

  beforeEach(async () => {
    routerSpy = jasmine.createSpyObj('Router', ['navigate']);

    await TestBed.configureTestingModule({
      imports: [TopbarComponent, FontAwesomeModule],
      providers: [
        { provide: Router, useValue: routerSpy },
        {
          provide: AuthStateService,
          useValue: { user$: of({ id: 1, email: 'a@b.com' } as any) }  // <- utilisateur connecté
        },
        { provide: CategoriesService, useValue: { getParents: () => of([]) } },
        { provide: RegionsService, useValue: { get: () => of([]) } },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(TopbarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should navigate to profile if user is logged in', () => {
    component.user = { id: 1, email: 'a@b.com' } as any;
    component.onProfileClick();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/profile']);
  });

  it('should navigate to basket', () => {
    component.goToBasket();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/basket']);
  });

});
