import { Component, inject, OnInit } from '@angular/core';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faCircleExclamation, faPlus } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { ProductDto, ProductsService } from '../../generated/clients/regionoix-client';

@Component({
  selector: 'app-backoffice-products',
  imports: [FontAwesomeModule],
  templateUrl: './backoffice-products.html',
  styleUrl: './backoffice-products.css',
})
export class BackofficeProducts implements OnInit {
  // font awesome icon plus
  faPlus = faPlus;
  faCircleExclamation = faCircleExclamation;

  products: ProductDto[] = [];
  selectedProduct: ProductDto | null = null;
  private readonly productService = inject(ProductsService);
  private readonly router = inject(Router);
  private readonly snackBar = inject(SnackbarService);

  ngOnInit(): void {
    this.loadProducts();
  }

  loadProducts(): void {
    this.productService.get().subscribe({
      next: (data) => (this.products = data),
      error: (err) => {
        this.snackBar.show('Echec lors de la récupération des produits', 'error');
        // update products list
        console.error('Somethings went wrong during products recuperation', err);
      },
    });
  }

  onCreateProduct(): void {
    this.router.navigate(['/backoffice/create-product']);
  }

  onSee(id: number): void {
    this.router.navigate(['/backoffice/products/', id]);
  }

  openDeleteModal(product: ProductDto): void {
    this.selectedProduct = product;
    const modal = document.getElementById('delete_modal');
    if (modal instanceof HTMLDialogElement) modal.showModal();
  }

  deleteProduct(): void {
    if (!this.selectedProduct) return;
    this.productService.deleteById(this.selectedProduct.id).subscribe({
      next: () => {
        this.snackBar.show(
          `Le produit "${this.selectedProduct?.name}" a bien été supprimé`,
          'success'
        );
        // Update products
        this.loadProducts();
        this.selectedProduct = null;
      },
      error: () => {
        this.snackBar.show(
          `Le produit "${this.selectedProduct?.name}" ne peut pas être supprimé`,
          'error'
        );
      },
    });
  }
}
