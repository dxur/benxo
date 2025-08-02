// Product service contains glue code for business logic and UI
import { create_product } from "@bindings/ProductRoutes";

async function createProduct() {
    // create product
    create_product()
        .then((res) => {
            console.log(res);
        })
        .catch((err) => {
            console.error(err);
        });
}



