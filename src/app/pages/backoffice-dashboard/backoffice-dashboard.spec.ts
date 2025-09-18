import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BackofficeDashboard } from './backoffice-dashboard';

describe('BackofficeDashboard', () => {
  let component: BackofficeDashboard;
  let fixture: ComponentFixture<BackofficeDashboard>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BackofficeDashboard]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BackofficeDashboard);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
