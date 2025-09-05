INSERT INTO CATEGORY(name) values('Confiture');
INSERT INTO REGION(name) values('Auverge Rhône-Alpes');
INSERT INTO BRAND(name) values('Jaaj organization');
INSERT INTO PRODUCT (name, description, weight, price, brand_id,stock, region_id) VALUES('Confiture artisanale de framboise', 'Une confiture gourmande et parfumée ! La délicate saveur de la framboise fraîchement ramassée sur notre exploitation et égrenée pour vous être plus agréable en bouche.',0.150, 5.80, 1,10,1);
INSERT INTO PRODUCT_CATEGORY(product_id, category_id) values(1,1);