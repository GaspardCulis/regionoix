export interface Product {
    id: number;
    name: string;
    description: string;
    weight: number;
    price: number;
    image: string;
    stock: number;

    brand: {
        id: number;
        name: string;
        description?: string;
    };

    region: {
        id: number;
        name: string;
        description?: string;
    };

    category: {
        id: number;
        name: string;
        category_parent?: number | null;
    };

    tags: {
        id: number;
        name: string;
    }[];
}