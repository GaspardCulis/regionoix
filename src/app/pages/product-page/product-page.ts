import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { HttpClient, HttpClientModule } from '@angular/common/http';
import { OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';


@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule, HttpClientModule],
  templateUrl: './product-page.html',
  styleUrl: './product-page.css'
})
export class ProductPage implements OnInit {
  product: any;
  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');
    this.http.get(`/api/products/${id}`)
      .subscribe({
        next: (data) => {
          this.product = data;
        },
        error: (err) => {
          console.error('An error appear at the prodcut recuperation', err);
        }
      });
  }
}
