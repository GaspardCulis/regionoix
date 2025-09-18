import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { of, throwError } from 'rxjs';
import { Router } from '@angular/router';
import { HomePage } from './home-page';
import { ProductsService, ProductDto, CategoriesService, CategoryDto } from '../../generated/clients/regionoix-client';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { HttpResponse } from '@angular/common/http';

describe('HomePage', () => {
  let component: HomePage;
  let fixture: ComponentFixture<HomePage>;
  let routerSpy: jasmine.SpyObj<Router>;
  let productsServiceSpy: jasmine.SpyObj<ProductsService>;
  let categoriesServiceSpy: jasmine.SpyObj<CategoriesService>;

  const mockProducts: ProductDto[] = [
    {
      id: 1,
      name: 'Produit 1',
      price: 100,
      tags: [],
      image: '/assets/1.png',
      stock: 10,          // obligatoire
      description: 'Desc 1', // si description est obligatoire
      discount: null      // si discount est obligatoire
    } as ProductDto,
    {
      id: 2,
      name: 'Produit 2',
      price: 200,
      tags: [],
      image: '/assets/2.png',
      stock: 5,
      description: 'Desc 2',
      discount: null
    } as ProductDto
  ];

  const mockCategories: CategoryDto[] = [
    { id: 1, name: 'Catégorie 1', category_parent: null } as CategoryDto,
    { id: 2, name: 'Catégorie 2', category_parent: null } as CategoryDto
  ];

  beforeEach(async () => {
    routerSpy = jasmine.createSpyObj('Router', ['navigate']);
    productsServiceSpy = jasmine.createSpyObj('ProductsService', ['getDiscounts', 'search']);
    categoriesServiceSpy = jasmine.createSpyObj('CategoriesService', ['get']);

    await TestBed.configureTestingModule({
      imports: [HomePage, ProductCardComponent],
      providers: [
        { provide: Router, useValue: routerSpy },
        { provide: ProductsService, useValue: productsServiceSpy },
        { provide: CategoriesService, useValue: categoriesServiceSpy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(HomePage);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should handle errors during product/category loading', fakeAsync(() => {
    spyOn(console, 'error');
    productsServiceSpy.getDiscounts.and.returnValue(throwError(() => new Error('fail')));
    productsServiceSpy.search.and.returnValue(throwError(() => new Error('fail')));
    categoriesServiceSpy.get.and.returnValue(throwError(() => new Error('fail')));

    component.ngOnInit();
    tick();

    expect(console.error).toHaveBeenCalledWith('Erreur chargement promotions');
    expect(console.error).toHaveBeenCalledWith('Erreur chargement nouveautés');
    expect(console.error).toHaveBeenCalledWith('Erreur chargement best-sellers');
    expect(console.error).toHaveBeenCalledWith('Erreur chargement catégories');
  }));

  it('should navigate to product page', () => {
    component.goToProduct(5);
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/products/', 5]);
  });

  it('should navigate to showcase', () => {
    component.goToShowcase();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/showcase']);
  });

  it('should update currentIndex on nextSlide and prevSlide', () => {
    component.bestProducts = mockProducts;

    component.nextSlide();
    expect(component.currentIndex).toBe(1);

    component.nextSlide();
    expect(component.currentIndex).toBe(0); // boucle au début

    component.prevSlide();
    expect(component.currentIndex).toBe(1); // boucle à la fin
  });

  it('should return correct transform string', () => {
    component.currentIndex = 2;
    expect(component.getTransform()).toBe('translateX(-200%)'); // 2 * 100%
  });
});
