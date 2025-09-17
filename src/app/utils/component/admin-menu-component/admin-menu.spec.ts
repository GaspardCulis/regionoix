import { ComponentFixture, TestBed } from '@angular/core/testing';
import { AdminMenu } from './admin-menu';
import { RouterLink } from '@angular/router';
import { NO_ERRORS_SCHEMA } from '@angular/core';
import { RouterTestingModule } from '@angular/router/testing';

describe('AdminMenu', () => {
  let component: AdminMenu;
  let fixture: ComponentFixture<AdminMenu>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AdminMenu, RouterTestingModule],
      schemas: [NO_ERRORS_SCHEMA] // ignore RouterLink et autres directives inconnues
    }).compileComponents();

    fixture = TestBed.createComponent(AdminMenu);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
