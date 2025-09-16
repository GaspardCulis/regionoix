import { Component, inject, OnInit } from '@angular/core';
import { Product } from '../../models/product-model';
import { ProductService } from '../../services/product-service';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faCircle, faCircleExclamation, faPlus } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';

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

  products: Product[] = [];
  selectedProduct: Product | null = null;
  private readonly productService = inject(ProductService);
  private readonly router = inject(Router);
  private readonly snackBar = inject(SnackbarService);

  ngOnInit(): void {
    this.loadProducts();
  }

  loadProducts(): void {
    this.productService.getProducts().subscribe({
      next: (data) => (this.products = data),
      error: (err) => {
        this.snackBar.show(
          'Echec lors de la récupération des produits',
          'error'
        );
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

  openDeleteModal(product: Product): void {
    this.selectedProduct = product;
    const modal: any = document.getElementById('delete_modal');
    if (modal) modal.showModal();
  }

  deleteProduct(): void {
    if (!this.selectedProduct) return;
    //TODO uncomment
    /*
    this.productService.delete(this.selectedProduct.id).subscribe({
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
          'Échec lors de la suppression du produit',
          'error'
        );
      },
    });*/
  }

}
