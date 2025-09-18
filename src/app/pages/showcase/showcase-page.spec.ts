import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { of } from 'rxjs';
import { RouterTestingModule } from '@angular/router/testing';
import { ShowcasePage } from './showcase-page';
import { BasketService, ProductsService, CategoriesService, RegionsService, TagsService, BrandsService, LoggedUser, ProductDto, CategoryDto, RegionDto, TagDto, BrandDto } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { AuthStateService } from '../../services/auth-state-service';

describe('ShowcasePage', () => {
  let component: ShowcasePage;
  let fixture: ComponentFixture<ShowcasePage>;

  let basketServiceSpy: jasmine.SpyObj<BasketService>;
  let productsServiceSpy: jasmine.SpyObj<ProductsService>;
  let categoriesServiceSpy: jasmine.SpyObj<CategoriesService>;
  let regionsServiceSpy: jasmine.SpyObj<RegionsService>;
  let tagsServiceSpy: jasmine.SpyObj<TagsService>;
  let brandsServiceSpy: jasmine.SpyObj<BrandsService>;
  let snackbarSpy: jasmine.SpyObj<SnackbarService>;
  let basketStateSpy: jasmine.SpyObj<BasketStateService>;
  let authStateSpy: jasmine.SpyObj<AuthStateService>;

  const mockProduct: ProductDto = { id: 1, name: 'Produit', price: 100 } as any;
  const mockCategory: CategoryDto = { id: 1, name: 'Catégorie' } as any;
  const mockRegion: RegionDto = { id: 1, name: 'Région' } as any;
  const mockTag: TagDto = { id: 1, name: 'Tag' } as any;
  const mockBrand: BrandDto = { id: 1, name: 'Brand' } as any;

  beforeEach(async () => {
    basketServiceSpy = jasmine.createSpyObj('BasketService', ['add']);
    productsServiceSpy = jasmine.createSpyObj('ProductsService', ['search']);
    categoriesServiceSpy = jasmine.createSpyObj('CategoriesService', ['get']);
    regionsServiceSpy = jasmine.createSpyObj('RegionsService', ['get']);
    tagsServiceSpy = jasmine.createSpyObj('TagsService', ['get']);
    brandsServiceSpy = jasmine.createSpyObj('BrandsService', ['get']);
    snackbarSpy = jasmine.createSpyObj('SnackbarService', ['show']);
    basketStateSpy = jasmine.createSpyObj('BasketStateService', ['refreshCount']);
    authStateSpy = jasmine.createSpyObj('AuthStateService', [], { currentUser: { id: 1, email: 'a@b.com' } as LoggedUser });

    productsServiceSpy.search.and.returnValue(of([mockProduct] as any));
    categoriesServiceSpy.get.and.returnValue(of([mockCategory] as any));
    regionsServiceSpy.get.and.returnValue(of([mockRegion] as any));
    tagsServiceSpy.get.and.returnValue(of([mockTag] as any));
    brandsServiceSpy.get.and.returnValue(of([mockBrand] as any));

    await TestBed.configureTestingModule({
      imports: [ShowcasePage, RouterTestingModule],
      providers: [
        { provide: BasketService, useValue: basketServiceSpy },
        { provide: ProductsService, useValue: productsServiceSpy },
        { provide: CategoriesService, useValue: categoriesServiceSpy },
        { provide: RegionsService, useValue: regionsServiceSpy },
        { provide: TagsService, useValue: tagsServiceSpy },
        { provide: BrandsService, useValue: brandsServiceSpy },
        { provide: SnackbarService, useValue: snackbarSpy },
        { provide: BasketStateService, useValue: basketStateSpy },
        { provide: AuthStateService, useValue: authStateSpy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(ShowcasePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load products on init', fakeAsync(() => {
    component.loadProducts();
    tick();
    expect(component.products.length).toBe(1);
    expect(component.products[0].name).toBe('Produit');
  }));

  it('should toggle category filter', fakeAsync(() => {
    component.toggleCategory('Catégorie', true);
    tick();
    expect(component.selectedCategorys).toContain('Catégorie');

    component.toggleCategory('Catégorie', false);
    tick();
    expect(component.selectedCategorys).not.toContain('Catégorie');
  }));

  it('should add item to basket if user is logged in', fakeAsync(() => {
    basketServiceSpy.add.and.returnValue(of({} as any));
    component.addItem(1);
    tick();
    expect(snackbarSpy.show).toHaveBeenCalledWith('Produit ajouté au panier ✅', 'success');
  }));
});
