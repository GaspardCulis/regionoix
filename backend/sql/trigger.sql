CREATE OR REPLACE TRIGGER add_cart_new_user 
AFTER INSERT ON USER_
FOR EACH ROW
EXECUTE FUNCTION add_cart_new_user_fn();

CREATE OR REPLACE FUNCTION add_cart_new_user_fn() RETURNS trigger AS $$
    BEGIN 
        INSERT INTO CART(user_id) VALUES(NEW.id);
        RETURN NEW;
    END;
$$ LANGUAGE plpgsql;