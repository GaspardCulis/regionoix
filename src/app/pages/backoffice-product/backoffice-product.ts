import { Component, inject, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faArrowLeft } from '@fortawesome/free-solid-svg-icons';
import { ProductDto, ProductsService } from '../../generated/clients/regionoix-client';
import { DatePipe, DecimalPipe } from '@angular/common';

@Component({
  selector: 'app-backoffice-product',
  imports: [FontAwesomeModule, DatePipe, DecimalPipe],
  templateUrl: './backoffice-product.html',
  styleUrl: './backoffice-product.css',
})
export class BackofficeProduct implements OnInit {
  faArrowLeft = faArrowLeft;
  product: null | ProductDto = null;
  final_price: null | number = null;
  loading = true;
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);
  private readonly productService = inject(ProductsService);

  ngOnInit(): void {
    const id = Number(this.route.snapshot.paramMap.get('id'));

    if (id) {
      this.productService.getById(id).subscribe({
        next: (data) => {
          this.product = {
            ...data,
            image: data.image ?? 'assets/default.png',
          };

          if (this.product.discount) {
            this.final_price =
              this.product.price -
              (this.product.price * this.product.discount.percentage_off) / 100;
          }
          this.loading = false;
        },
        error: (err) => {
          console.error('Erreur lors de la récupération du produit', err);
          this.loading = false;
        },
      });
    } else {
      this.loading = false;
    }
  }

  onBack(): void {
    this.router.navigate(['/backoffice/products']);
  }
}
